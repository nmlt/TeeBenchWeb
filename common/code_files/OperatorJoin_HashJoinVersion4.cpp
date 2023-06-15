#include <stdint.h>
#include <JoinCommons.h>
#include "data-types.h"
#include "../RadixJoin/radix_join.h"
#include "prj_params.h"
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

#define JOIN_NAME "HashJoinVersion4"

#define HASH_BIT_MODULO(K, MASK, NBITS) (((K) & MASK) >> NBITS)
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

static int64_t
bucket_chaining_join(const struct table_t * const R,
                     const struct table_t * const S,
                     struct table_t * const tmpR,
                     output_list_t ** output,
                     bool materialize)
{
    (void) (tmpR);
    (void) (output);
    int * next, * bucket;
    const uint64_t numR = R->num_tuples;
    uint32_t N = (uint32_t)numR;
    int64_t matches = 0;

    NEXT_POW_2(N);
    /* N <<= 1; */
    const uint32_t MASK = (N-1) << (NUM_RADIX_BITS);

    next   = (int*) malloc(sizeof(int) * numR);
    /* posix_memalign((void**)&next, CACHE_LINE_SIZE, numR * sizeof(int)); */
    bucket = (int*) calloc(N, sizeof(int));

    const struct row_t * const Rtuples = R->tuples;
    for(uint32_t i=0; i < numR; ){
        uint32_t idx = HASH_BIT_MODULO(R->tuples[i].key, MASK, NUM_RADIX_BITS);
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

        uint32_t idx = HASH_BIT_MODULO(Stuples[i].key, MASK, NUM_RADIX_BITS);

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
    result_t * res = join_init_run(relR, relS, bucket_chaining_join, config);
    logJoin(JOIN_NAME, config, res->jr);
    return res;
}
