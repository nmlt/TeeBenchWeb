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
#endif

#define JOIN_NAME "HashJoinVersion3"

#ifndef HASH_BIT_MODULO
#define HASH_BIT_MODULO(K, MASK, NBITS) (((K) & MASK) >> NBITS)
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

/**
 * Radix clustering algorithm which does not put padding in between
 * clusters. This is used only by single threaded radix join implementation RJ.
 *
 * @param outRel
 * @param inRel
 * @param hist
 * @param R
 * @param D
 */
static void
radix_cluster_nopadding(struct table_t * outRel, struct table_t * inRel, int R, int D)
{
    row_t ** dst;
    row_t * input;
    /* tuple_t ** dst_end; */
    uint32_t * tuples_per_cluster;
    uint32_t i;
    uint32_t offset;
    const uint32_t M = ((1 << D) - 1) << R;
    const uint32_t fanOut = 1 << D;
    const uint64_t ntuples = inRel->num_tuples;

    tuples_per_cluster = (uint32_t*)calloc(fanOut, sizeof(uint32_t));
    /* the following are fixed size when D is same for all the passes,
       and can be re-used from call to call. Allocating in this function
       just in case D differs from call to call. */
    dst     = (row_t**)malloc(sizeof(row_t*)*fanOut);

    input = inRel->tuples;
    /* count tuples per cluster */
    for( i=0; i < ntuples; i++ ){
        uint32_t idx = (uint32_t)(HASH_BIT_MODULO(input->key, M, R));
        tuples_per_cluster[idx]++;
        input++;
    }

    offset = 0;
    /* determine the start and end of each cluster depending on the counts. */
    for ( i=0; i < fanOut; i++ ) {
        dst[i]      = outRel->tuples + offset;
        offset     += tuples_per_cluster[i];
    }

    input = inRel->tuples;
    /* copy tuples to their corresponding clusters at appropriate offsets */
    for( i=0; i < ntuples; i++ ){
        uint32_t idx   = (uint32_t)(HASH_BIT_MODULO(input->key, M, R));
        *dst[idx] = *input;
        ++dst[idx];
        input++;
    }

    free(dst);
    free(tuples_per_cluster);
}

/**
 *  This algorithm builds the hashtable using the bucket chaining idea and used
 *  in PRO implementation. Join between given two relations is evaluated using
 *  the "bucket chaining" algorithm proposed by Manegold et al. It is used after
 *  the partitioning phase, which is common for all algorithms. Moreover, R and
 *  S typically fit into L2 or at least R and |R|*sizeof(int) fits into L2 cache.
 *
 * @param R input relation R
 * @param S input relation S
 *
 * @return number of result tuples
 */
static int64_t
bucket_chaining_join(const struct table_t * const R,
                     const struct table_t * const S,
                     struct table_t * const tmpR,
                     output_list_t ** output,
                     bool materialize,
                     int num_radix_bits)
{
    (void) (tmpR);
    (void) (output);
    int * next, * bucket;
    const uint64_t numR = R->num_tuples;
    uint32_t N = (uint32_t)numR;
    int64_t matches = 0;

    NEXT_POW_2(N);
    /* N <<= 1; */
    const uint32_t MASK = (N-1) << (num_radix_bits);

    next   = (int*) malloc(sizeof(int) * numR);
    /* posix_memalign((void**)&next, CACHE_LINE_SIZE, numR * sizeof(int)); */
    bucket = (int*) calloc(N, sizeof(int));

    const struct row_t * const Rtuples = R->tuples;
    for(uint32_t i=0; i < numR; ){
        uint32_t idx = HASH_BIT_MODULO(R->tuples[i].key, MASK, num_radix_bits);
        next[i]      = bucket[idx];
        bucket[idx]  = ++i;     /* we start pos's from 1 instead of 0 */

        /* Enable the following tO avoid the code elimination
           when running probe only for the time break-down experiment */
        /* matches += idx; */
    }

    const struct row_t * const Stuples = S->tuples;
    const uint64_t        numS    = S->num_tuples;

    /* Disable the following loop for no-probe for the break-down experiments */
    /* PROBE- LOOP */
    for(uint32_t i=0; i < numS; i++ ){

        uint32_t idx = HASH_BIT_MODULO(Stuples[i].key, MASK, num_radix_bits);

        for(int hit = bucket[idx]; hit > 0; hit = next[hit-1]){

            if(Stuples[i].key == Rtuples[hit-1].key){
                matches ++;
            }
        }
    }
    /* PROBE-LOOP END  */

    /* clean up temp */
    free(bucket);
    free(next);

    return matches;
}


result_t* OperatorJoin (struct table_t* relR, struct table_t* relS, joinconfig_t * config) {
    config->NTHREADS = 1;
    // 8 is minimum
    int num_radix_bits = 8;
    int num_passes = 1;
    int cache_line_size = 64;
    int fanout_pass1 = (1 << (num_radix_bits/num_passes));
    int fanout_pass2 = (1 << (num_radix_bits-(num_radix_bits/num_passes)));
    int small_padding_tuples = (3 * cache_line_size/sizeof(struct row_t));
    int padding_tuples = (small_padding_tuples * (fanout_pass2+1));
    int relation_padding = (padding_tuples * fanout_pass1 * sizeof(struct row_t));
    int64_t result = 0;
    uint64_t i;

    uint64_t start, end;
    uint64_t timer1, timer2;

    struct table_t *outRelR, *outRelS;

    outRelR = (struct table_t*) malloc(sizeof(struct table_t));
    outRelS = (struct table_t*) malloc(sizeof(struct table_t));

    /* allocate temporary space for partitioning */
    size_t sz = relR->num_tuples * sizeof(row_t) + relation_padding;
    outRelR->tuples     = (row_t*) malloc(sz);
    outRelR->num_tuples = relR->num_tuples;

    sz = relS->num_tuples * sizeof(row_t) + relation_padding;
    outRelS->tuples     = (row_t*) malloc(sz);
    outRelS->num_tuples = relS->num_tuples;

    ocall_get_system_micros(&start);
    ocall_startTimer(&timer1);
    timer2=timer1;

#ifdef PCM_COUNT
    ocall_set_system_counter_state("Partition");
#endif

    /***** do the multi-pass partitioning *****/
    if (num_passes == 1) {
        /* apply radix-clustering on relation R for pass-1 */
        radix_cluster_nopadding(outRelR, relR, 0, num_radix_bits);
        relR = outRelR;

        /* apply radix-clustering on relation S for pass-1 */
        radix_cluster_nopadding(outRelS, relS, 0, num_radix_bits);
        relS = outRelS;
    } else if (num_passes == 2) {
        /* apply radix-clustering on relation R for pass-1 */
        radix_cluster_nopadding(outRelR, relR, 0, num_radix_bits/num_passes);

        /* apply radix-clustering on relation S for pass-1 */
        radix_cluster_nopadding(outRelS, relS, 0, num_radix_bits/num_passes);

        /* apply radix-clustering on relation R for pass-2 */
        radix_cluster_nopadding(relR, outRelR,
                                num_radix_bits/num_passes,
                                num_radix_bits-(num_radix_bits/num_passes));

        /* apply radix-clustering on relation S for pass-2 */
        radix_cluster_nopadding(relS, outRelS,
                                num_radix_bits/num_passes,
                                num_radix_bits-(num_radix_bits/num_passes));

        /* clean up temporary relations */
        free(outRelR->tuples);
        free(outRelS->tuples);
        free(outRelR);
        free(outRelS);
    } else {
        ocall_exit(0);
    }

#ifdef PCM_COUNT
    hw_counters_t * phase1HwCounters = (hw_counters_t*) (malloc(sizeof(hw_counters_t)));
    ocall_get_system_counter_state2(0, phase1HwCounters);
    ocall_set_system_counter_state("Join");
#endif

    ocall_stopTimer(&timer1);

    uint64_t * R_count_per_cluster = (uint64_t*)calloc((1<<num_radix_bits), sizeof(uint64_t));
    uint64_t * S_count_per_cluster = (uint64_t*)calloc((1<<num_radix_bits), sizeof(uint64_t));

    /* compute number of tuples per cluster */
    for( i=0; i < relR->num_tuples; i++ ){
        uint32_t idx = (relR->tuples[i].key) & ((1<<num_radix_bits)-1);
        R_count_per_cluster[idx] ++;
    }
    for( i=0; i < relS->num_tuples; i++ ){
        uint32_t idx = (relS->tuples[i].key) & ((1<<num_radix_bits)-1);
        S_count_per_cluster[idx] ++;
    }

    /* build hashtable on inner */
    int r, s; /* start index of next clusters */
    r = s = 0;
    for( i=0; i < (1<<num_radix_bits); i++ ){
        struct table_t tmpR, tmpS;

        if(R_count_per_cluster[i] > 0 && S_count_per_cluster[i] > 0){

            tmpR.num_tuples = R_count_per_cluster[i];
            tmpR.tuples = relR->tuples + r;
            r += (int)R_count_per_cluster[i];

            tmpS.num_tuples = S_count_per_cluster[i];
            tmpS.tuples = relS->tuples + s;
            s += (int)S_count_per_cluster[i];

            result += bucket_chaining_join(&tmpR, &tmpS, NULL, nullptr, config->MATERIALIZE, num_radix_bits);
        }
        else {
            r += (int)R_count_per_cluster[i];
            s += (int)S_count_per_cluster[i];
        }
    }

    ocall_stopTimer(&timer2);
    ocall_get_system_micros(&end);
#ifdef PCM_COUNT
    hw_counters_t * phase2HwCounters = (hw_counters_t*) (malloc(sizeof(hw_counters_t)));
    hw_counters_t * totalHwCounters = (hw_counters_t*) (malloc(sizeof(hw_counters_t)));
    ocall_get_system_counter_state2(0, phase2HwCounters);
    ocall_get_system_counter_state2(1, totalHwCounters);
#endif

    join_result_t * jr = (join_result_t *) calloc(1, sizeof(join_result_t));
    jr->inputTuplesR = relR->num_tuples;
    jr->inputTuplesS = relS->num_tuples;
    jr->matches = result;
    jr->totalCycles = timer2;
    jr->totalTime = end - start;
    jr->phase1Cycles = timer1;
    jr->phase2Cycles = timer2-timer1;
#ifdef PCM_COUNT
    jr->phase1HwCounters = phase1HwCounters;
    jr->phase2HwCounters = phase2HwCounters;
    jr->totalHwCounters = totalHwCounters;
    jr->hwFlag = 1;
#endif
    logJoin(JOIN_NAME, config, jr);

    /* clean-up temporary buffers */
    free(S_count_per_cluster);
    free(R_count_per_cluster);

    if (num_passes == 1) {
        /* clean up temporary relations */
        free(outRelR->tuples);
        free(outRelS->tuples);
        free(outRelR);
        free(outRelS);
    }

    result_t * joinresult;
    joinresult = (result_t *) malloc(sizeof(result_t));
    joinresult->totalresults = result;
    joinresult->nthreads = 1;
    return joinresult;
}
