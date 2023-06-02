#include <stdint.h>
#include <pthread.h>
#include <JoinCommons.h>
#include "data-types.h"
#ifdef NATIVE_COMPILATION
#include <malloc.h>
#include "Logger.h"
#include "native_ocalls.h"
#include <cstring>
#include "pcm_commons.h"
#else
#include "Enclave_t.h"
#include "Enclave.h"
#include <sgx_spinlock.h>
#endif

#define JOIN_NAME "HashJoinVersion1"

#ifndef HASH
#define HASH(X, MASK, SKIP) (((X) & MASK) >> SKIP)
#endif

#ifndef NEXT_POW_2
/**
 *  compute the next number, greater than or equal to 32-bit unsigned v.
 *  taken from "bit twiddling hacks":
 *  http://graphics.stanford.edu/~seander/bithacks.html
 */
#define NEXT_POW_2(V)                           \
    do {                                        \
        V--;                                    \
        V |= V >> 1;                            \
        V |= V >> 2;                            \
        V |= V >> 4;                            \
        V |= V >> 8;                            \
        V |= V >> 16;                           \
        V++;                                    \
    } while(0)
#endif

#ifndef CACHE_LINE_SIZE
#define CACHE_LINE_SIZE 64
#endif

#ifndef BUCKET_SIZE
#define BUCKET_SIZE 2
#endif

struct bucket_t {
#ifdef NATIVE_COMPILATION
    volatile char latch;
#else
    sgx_spinlock_t    latch;
#endif

    /* 3B hole */ // Kajetan: there is no hole anymore -
    // sgx_spinlock_t takes 32 bits
    uint32_t          count;
    struct row_t      tuples[BUCKET_SIZE];
    struct bucket_t * next;
};

struct hashtable_t {
    bucket_t * buckets;
    int32_t    num_buckets;
    uint32_t   hash_mask;
    uint32_t   skip_bits;
};

typedef struct bucket_t        bucket_t;
typedef struct hashtable_t     hashtable_t;


static void allocate_hashtable(hashtable_t ** ppht, uint32_t nbuckets)
{
    hashtable_t * ht;

    ht              = (hashtable_t*)malloc(sizeof(hashtable_t));
    ht->num_buckets = nbuckets;
    NEXT_POW_2((ht->num_buckets));

    /* allocate hashtable buckets cache line aligned */
    ht->buckets = (bucket_t*) memalign(CACHE_LINE_SIZE, ht->num_buckets * sizeof(bucket_t));
    if (ht->buckets == nullptr){
        exit(EXIT_FAILURE);
    }

    memset(ht->buckets, 0, ht->num_buckets * sizeof(bucket_t));
    ht->skip_bits = 0; /* the default for modulo hash */
    ht->hash_mask = (ht->num_buckets - 1) << ht->skip_bits;
    *ppht = ht;
}

/**
 * Single-thread hashtable build method, ht is pre-allocated.
 *
 * @param ht hastable to be built
 * @param rel the build relation
 */
static void
build_hashtable_st(hashtable_t *ht, struct table_t *rel)
{
    uint64_t i;
    const uint32_t hashmask = ht->hash_mask;
    const uint32_t skipbits = ht->skip_bits;

    for(i=0; i < rel->num_tuples; i++){
        struct row_t * dest;
        bucket_t * curr, * nxt;
        int64_t idx = HASH(rel->tuples[i].key, hashmask, skipbits);

        /* copy the tuple to appropriate hash bucket */
        /* if full, follow nxt pointer to find correct place */
        curr = ht->buckets + idx;
        nxt  = curr->next;

        if(curr->count == BUCKET_SIZE) {
            if(!nxt || nxt->count == BUCKET_SIZE) {
                bucket_t * b;
                b = (bucket_t*) calloc(1, sizeof(bucket_t));
                curr->next = b;
                b->next = nxt;
                b->count = 1;
                dest = b->tuples;
            }
            else {
                dest = nxt->tuples + nxt->count;
                nxt->count ++;
            }
        }
        else {
            dest = curr->tuples + curr->count;
            curr->count ++;
        }
        *dest = rel->tuples[i];
    }
}


/**
 * Probes the hashtable for the given outer relation, returns num results.
 * This probing method is used for both single and multi-threaded version.
 *
 * @param ht hashtable to be probed
 * @param rel the probing outer relation
 *
 * @return number of matching tuples
 */
static int64_t
probe_hashtable(hashtable_t *ht, struct table_t *rel)
{
    uint64_t i, j;
    int64_t matches;

    const uint32_t hashmask = ht->hash_mask;
    const uint32_t skipbits = ht->skip_bits;

    matches = 0;

    for (i = 0; i < rel->num_tuples; i++)
    {
        type_key idx = HASH(rel->tuples[i].key, hashmask, skipbits);
        bucket_t * b = ht->buckets+idx;

        do {
            for(j = 0; j < b->count; j++) {
                if(rel->tuples[i].key == b->tuples[j].key){
                    matches ++;
                    /* we don't materialize the results. */
                }
            }

            b = b->next;/* follow overflow pointer */
        } while(b);
    }

    return matches;
}

static void
destroy_hashtable(hashtable_t * ht)
{
    free(ht->buckets);
    free(ht);
}

result_t* OperatorJoin (struct table_t* relR, struct table_t* relS, joinconfig_t * config) {
    hashtable_t * ht;
    int64_t result;
    config->NTHREADS = 1;
    uint64_t timer1, timer2, start, end;
    uint32_t nbuckets = (relR->num_tuples / BUCKET_SIZE);
    allocate_hashtable(&ht, nbuckets);

    ocall_get_system_micros(&start);
    ocall_startTimer(&timer1);
    timer2 = timer1;

#ifdef PCM_COUNT
    ocall_set_system_counter_state("build");
#endif

    build_hashtable_st(ht, relR);

#ifdef PCM_COUNT
    hw_counters_t * phase1HwCounters = (hw_counters_t*) (malloc(sizeof(hw_counters_t)));
    ocall_get_system_counter_state2(0, phase1HwCounters);
#endif

    ocall_stopTimer(&timer2); /* for build */

#ifdef PCM_COUNT
    ocall_set_system_counter_state("probe");
#endif

    result = probe_hashtable(ht, relS);

#ifdef PCM_COUNT
    hw_counters_t * phase2HwCounters = (hw_counters_t*) (malloc(sizeof(hw_counters_t)));
    hw_counters_t * totalHwCounters = (hw_counters_t*) (malloc(sizeof(hw_counters_t)));
    ocall_get_system_counter_state2(0, phase2HwCounters);
    ocall_get_system_counter_state2(1, totalHwCounters);
#endif

    ocall_get_system_micros(&end);
    ocall_stopTimer(&timer1); /* over all */
    join_result_t * jr = (join_result_t *) calloc(1, sizeof(join_result_t));
//    memset(jr, 0, sizeof(join_result_t));
    jr->inputTuplesR = relR->num_tuples;
    jr->inputTuplesS = relS->num_tuples;
    jr->matches = result;
    jr->totalCycles = timer1;
    jr->totalTime = end - start;
    jr->phase1Cycles = timer2;
    jr->phase2Cycles = timer1 - timer2;
#ifdef PCM_COUNT
    jr->phase1HwCounters = phase1HwCounters;
    jr->phase2HwCounters = phase2HwCounters;
    jr->totalHwCounters = totalHwCounters;
    jr->hwFlag = 1;
#endif

    destroy_hashtable(ht);

    result_t * joinresult;
    joinresult = (result_t *) malloc(sizeof(result_t));
    joinresult->totalresults = result;
    joinresult->nthreads = 1;
    joinresult->jr = jr;
    logJoin(JOIN_NAME, config, jr);
    return joinresult;
}
