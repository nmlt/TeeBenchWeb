#include <stdint.h>
#include "btree.h"
#include <pthread.h>
#include <JoinCommons.h>
#include "data-types.h"
#ifdef NATIVE_COMPILATION
#include "Logger.h"
#include "native_ocalls.h"
#include <cstring>
#else
#include "Enclave_t.h"
#include "Enclave.h"
#endif

struct arg_inl_t
{
    uint32_t my_tid;
    tuple_t *relR;
    //    tuple_t * relS;

    uint32_t numR;
    uint32_t totalR;

    stx::btree<type_key, type_value> *indexS;

    uint32_t matches = 0;
};

void *my_inl_thread(void *param)
{
    uint32_t i, matches = 0;
    arg_inl_t *args = (arg_inl_t *)param;
    //    uint32_t my_tid = args->my_tid;

    stx::btree<type_key, type_value> *index = args->indexS;

    // for each R scan S-index
    for (i = 0; i < args->numR; i++)
    {
        row_t r = args->relR[i];
        size_t count = index->count(r.key);
        if (count)
        {
            auto it = index->find(r.key);
            for (size_t j = 0; j < count; j++)
            {
                matches++;
                it++;
            }
        }
    }
    //    logger(INFO, "Thread-%d matches: %u", my_tid, matches);
    args->matches = matches;
    return nullptr;
}

result_t *OperatorJoin(struct table_t *relR, struct table_t *relS, joinconfig_t *config)
{
    uint64_t i, matches = 0;
    int rv;
    stx::btree<type_key, type_value> index;

    int nthreads = config->NTHREADS;

    pthread_t tid[nthreads];
    arg_inl_t args[nthreads];
    uint64_t numperthr[2];

    uint64_t timer, start, end;

    numperthr[0] = relR->num_tuples / nthreads;
    numperthr[1] = relS->num_tuples / nthreads;

    // build index on S
    for (i = 0; i < relS->num_tuples; i++)
    {
        index.insert(std::make_pair(relS->tuples[i].key, relS->tuples[i].payload));
    }

    logger(DBG, "Index complete. Size: %zu", index.size());

    ocall_startTimer(&timer);
    ocall_get_system_micros(&start);
#ifdef PCM_COUNT
    ocall_set_system_counter_state("Start join phase");
#endif
    for (i = 0; i < nthreads; i++)
    {
        args[i].relR = relR->tuples + i * numperthr[0];

        args[i].numR = (i == (nthreads - 1)) ? (relR->num_tuples - i * numperthr[0]) : numperthr[0];
        args[i].totalR = relR->num_tuples;

        args[i].my_tid = i;
        args[i].indexS = &index;

        rv = pthread_create(&tid[i], nullptr, my_inl_thread, (void *)&args[i]);

        if (rv)
        {
            logger(ERROR, "return code from pthread_create() is %d\n", rv);
            ocall_exit(-1);
        }
    }

    for (i = 0; i < nthreads; i++)
    {
        pthread_join(tid[i], nullptr);
        matches += args[i].matches;
    }
#ifdef PCM_COUNT
    ocall_get_system_counter_state("Join", 0);
#endif
    ocall_get_system_micros(&end);
    ocall_stopTimer(&timer);
    join_result_t *jr = (join_result_t *)malloc(sizeof(join_result_t));
    memset(jr, 0, sizeof(join_result_t));
    jr->input_tuples_R = relR->num_tuples;
    jr->input_tuples_S = relS->num_tuples;
    jr->matches = matches;
    jr->total_cycles = timer;
    jr->timer_total_usec = end - start;
    logJoin("TBW", config, jr);
    free(jr);
    //    print_timing(timer, relR->num_tuples + relS->num_tuples, matches, start, end);

    result_t *joinresult;
    joinresult = (result_t *)malloc(sizeof(result_t));
    joinresult->totalresults = matches;
    joinresult->nthreads = nthreads;
    return joinresult;
}