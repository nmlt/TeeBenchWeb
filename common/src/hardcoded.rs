use crate::{
    commandline::Commandline,
    commit::CommitIdType,
    data_types::{Algorithm, JobConfig, PerfReportConfig, Platform, REPLACE_ALG},
};

// TODO Hardcoded Vecs could become arrays.
pub fn hardcoded_perf_report_configs(id: CommitIdType, baseline: Algorithm) -> Vec<JobConfig> {
    let (throughput_fit, throughput_exceed) = PerfReportConfig::for_throughput(id, baseline);
    let (scalability_fit, scalability_exceed) = PerfReportConfig::for_scalability(id, baseline);
    vec![
        JobConfig::PerfReport(throughput_fit),
        JobConfig::PerfReport(throughput_exceed),
        JobConfig::PerfReport(scalability_fit),
        JobConfig::PerfReport(scalability_exceed),
    ]
}

pub fn hardcoded_throughput_commands(
    alg: Algorithm,
    alg_cmd_string: &str,
    dataset_cmd_string: &str,
) -> Vec<Commandline> {
    #[rustfmt::skip]
    let v = vec![
        Commandline::with_args(Platform::Sgx   ,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","2","--csv"]),
        Commandline::with_args(Platform::Native,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","2","--csv"]),
    ];
    v
}

pub fn hardcoded_scalability_commands(
    alg: Algorithm,
    alg_cmd_string: &str,
    dataset_cmd_string: &str,
) -> Vec<Commandline> {
    #[rustfmt::skip]
    let v = vec![
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","1","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","3","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","4","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","5","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","6","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","7","--csv"]),
        Commandline::with_args(Platform::Sgx,alg,&vec!["-a",alg_cmd_string,"-d",dataset_cmd_string,"-n","8","--csv"]),
    ];
    v
}

pub fn hardcoded_perf_report_commands(
    id: CommitIdType,
    baseline_t: &Algorithm,
) -> Vec<Vec<Commandline>> {
    let baseline = &baseline_t.to_cmd_arg();
    let commit_id = Algorithm::Commit(id);
    #[rustfmt::skip]
    let res = vec![
        // Throughput Cache-Fit
        vec![
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Native,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Native,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
        ],
        // Throughput Cache-Exceed
        vec![
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Native,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Native,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
        ],
        // Scalability Cache-Fit
        vec![
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","8","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","8","--csv"]),
        ],
            // Scalability Cache-Exceed
        vec![
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   ,*baseline_t,&vec!["-a",baseline   ,"-d","cache-exceed","-n","8","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   ,commit_id  ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","8","--csv"]),
        ],
        // EPC Paging Commit
        // TODO
        // EPC Paging baseline
        // TODO
    ];
    res
}

use crate::commit::{Commit, Operator};
use indoc::indoc;
pub fn predefined_commit() -> Commit {
    let merge_ = indoc! {r#"
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

        #define JOIN_NAME "SortMergeJoin_MergeSort"

        // Quicksort implementation adapted from https://www.geeksforgeeks.org/merge-sort/
        // Merges two subarrays of array[].
        // First subarray is arr[begin..mid]
        // Second subarray is arr[mid+1..end]
        void merge(tuple_t * array, uint64_t left, uint64_t mid,
                uint64_t right)
        {
            auto const subArrayOne = mid - left + 1;
            auto const subArrayTwo = right - mid;

            // Create temp arrays
            auto *leftArray = new tuple_t[subArrayOne],
                    *rightArray = new tuple_t[subArrayTwo];

            // Copy data to temp arrays leftArray[] and rightArray[]
            for (auto i = 0; i < subArrayOne; i++)
                leftArray[i] = array[left + i];
            for (auto j = 0; j < subArrayTwo; j++)
                rightArray[j] = array[mid + 1 + j];

            auto indexOfSubArrayOne
                    = 0, // Initial index of first sub-array
            indexOfSubArrayTwo
            = 0; // Initial index of second sub-array
            uint64_t indexOfMergedArray
                    = left; // Initial index of merged array

            // Merge the temp arrays back into array[left..right]
            while (indexOfSubArrayOne < subArrayOne
                && indexOfSubArrayTwo < subArrayTwo) {
                if (leftArray[indexOfSubArrayOne].key
                    <= rightArray[indexOfSubArrayTwo].key) {
                    array[indexOfMergedArray]
                            = leftArray[indexOfSubArrayOne];
                    indexOfSubArrayOne++;
                }
                else {
                    array[indexOfMergedArray]
                            = rightArray[indexOfSubArrayTwo];
                    indexOfSubArrayTwo++;
                }
                indexOfMergedArray++;
            }
            // Copy the remaining elements of
            // left[], if there are any
            while (indexOfSubArrayOne < subArrayOne) {
                array[indexOfMergedArray]
                        = leftArray[indexOfSubArrayOne];
                indexOfSubArrayOne++;
                indexOfMergedArray++;
            }
            // Copy the remaining elements of
            // right[], if there are any
            while (indexOfSubArrayTwo < subArrayTwo) {
                array[indexOfMergedArray]
                        = rightArray[indexOfSubArrayTwo];
                indexOfSubArrayTwo++;
                indexOfMergedArray++;
            }
            delete[] leftArray;
            delete[] rightArray;
        }

        // begin is for left index and end is
        // right index of the sub-array
        // of arr to be sorted */
        void mergeSort(tuple_t * rel, uint64_t begin, uint64_t end)
        {
            if (begin >= end)
                return; // Returns recursively

            auto mid = begin + (end - begin) / 2;
            mergeSort(rel, begin, mid);
            mergeSort(rel, mid + 1, end);
            merge(rel, begin, mid, end);
        }


        result_t* OperatorJoin (struct table_t* relR, struct table_t* relS, joinconfig_t * config) {
            uint64_t timerStart, timerStop, cycles, timerSort, timerMerge;

            ocall_get_system_micros(&timerStart);
            ocall_startTimer(&cycles);
            // Sort
            mergeSort(relR->tuples, 0, relR->num_tuples - 1);
            mergeSort(relS->tuples, 0, relS->num_tuples - 1);

            ocall_get_system_micros(&timerStop);
            timerSort = timerStop - timerStart;
            timerStart = timerStop;

            // Merge
            tuple_t *outer = relR->tuples;
            tuple_t *inner = relS->tuples;
            tuple_t *mark;
            uint64_t matches = 0;

            tuple_t *olast = outer + relR->num_tuples;
            tuple_t *ilast = inner + relS->num_tuples;

            while(outer < olast && inner < ilast) {
                while(outer->key != inner->key) {
                    if (outer->key < inner->key) {
                        outer++;
                    } else {
                        inner++;
                    }
                }
                mark = inner;
                while(true) {
                    while(outer->key == inner->key) {
                        matches++;
                        inner++;
                    }
                    outer++;
                    if (outer == mark) {
                        inner = mark;
                    } else {
                        break;
                    }
                }
            }

            ocall_stopTimer(&cycles);
            ocall_get_system_micros(&timerStop);
            timerMerge = timerStop - timerStart;

            join_result_t jr = {};

            double throughput = (double) (relR->num_tuples + relS->num_tuples) / (double) (timerSort+timerMerge);
            jr.inputTuplesR = relR->num_tuples;
            jr.inputTuplesS = relS->num_tuples;
            jr.totalCycles = cycles;
            jr.phase1Time = timerSort;
            jr.phase2Time = timerMerge;
            jr.totalTime = (timerSort+timerMerge);
            jr.matches = matches;

            logJoin(JOIN_NAME, config, &jr);

            auto joinresult = new result_t;
            joinresult->nthreads = 1;
            joinresult->totalresults = matches;

            return joinresult;
        }
    "#}.to_string();
    use crate::data_types::{
        Algorithm::Rho,
        Dataset::{CacheExceed, CacheFit},
        ExperimentChart,
        ExperimentType::{Scalability, Throughput},
        JobConfig::PerfReport,
        JobResult::Exp,
        Platform::{Native, Sgx},
        Report, TeebenchArgs,
    };
    use std::collections::HashMap;
    let report = Some(Exp(Ok(Report {
        charts: vec![
            ExperimentChart {
                config: PerfReport(PerfReportConfig {
                    id: 1,
                    exp_type: Throughput,
                    dataset: CacheFit,
                    baseline: Rho,
                }),
                results: vec![
                    (
                        TeebenchArgs {
                            app_name: Native,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("cyclesPerTuple".to_string(), "16".to_string()),
                            ("phase2Cycles".to_string(), "28499098".to_string()),
                            ("totalTime".to_string(), "31908".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("throughput".to_string(), "205.3905".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("phase1Cycles".to_string(), "80242162".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Native,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("phase2Time".to_string(), "5668".to_string()),
                            ("totalTime".to_string(), "430608".to_string()),
                            ("throughput".to_string(), "15.2194".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase1Time".to_string(), "424940".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "223".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("throughput".to_string(), "38.0344".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("cyclesPerTuple".to_string(), "89".to_string()),
                            ("totalTime".to_string(), "172307".to_string()),
                            ("phase1Cycles".to_string(), "555768398".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "31417312".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("throughput".to_string(), "14.9952".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "5583".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("cyclesPerTuple".to_string(), "227".to_string()),
                            ("phase1Time".to_string(), "431463".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("totalTime".to_string(), "437046".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                        ]),
                    ),
                ],
                findings: vec![],
            },
            ExperimentChart {
                config: PerfReport(PerfReportConfig {
                    id: 1,
                    exp_type: Throughput,
                    dataset: CacheExceed,
                    baseline: Rho,
                }),
                results: vec![
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("totalTime".to_string(), "2468680".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("throughput".to_string(), "26.5470".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "187156882".to_string()),
                            ("cyclesPerTuple".to_string(), "128".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("phase1Cycles".to_string(), "8226046256".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Native,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase1Cycles".to_string(), "0".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("phase1Time".to_string(), "4957181".to_string()),
                            ("totalTime".to_string(), "5012861".to_string()),
                            ("throughput".to_string(), "13.0736".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase2Time".to_string(), "55680".to_string()),
                            ("cyclesPerTuple".to_string(), "260".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("throughput".to_string(), "12.9637".to_string()),
                            ("cyclesPerTuple".to_string(), "262".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "55460".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("totalTime".to_string(), "5055358".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase1Time".to_string(), "4999898".to_string()),
                            ("threads".to_string(), "2".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Native,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("cyclesPerTuple".to_string(), "14".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Cycles".to_string(), "187798056".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "749569428".to_string()),
                            ("totalTime".to_string(), "275050".to_string()),
                            ("throughput".to_string(), "238.2694".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                        ]),
                    ),
                ],
                findings: vec![],
            },
            ExperimentChart {
                config: PerfReport(PerfReportConfig {
                    id: 1,
                    exp_type: Scalability,
                    dataset: CacheFit,
                    baseline: Rho,
                }),
                results: vec![
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("throughput".to_string(), "39.6187".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "86".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("totalTime".to_string(), "165417".to_string()),
                            ("phase1Cycles".to_string(), "530011722".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "33695012".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("threads".to_string(), "2".to_string()),
                            ("throughput".to_string(), "14.9850".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("totalTime".to_string(), "437344".to_string()),
                            ("phase1Time".to_string(), "431840".to_string()),
                            ("phase2Time".to_string(), "5504".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "227".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 3,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "227".to_string()),
                            ("phase1Time".to_string(), "431258".to_string()),
                            ("totalTime".to_string(), "436717".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("threads".to_string(), "3".to_string()),
                            ("phase2Time".to_string(), "5459".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("relS".to_string(), "5242880".to_string()),
                            ("throughput".to_string(), "15.0065".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 3,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("totalTime".to_string(), "225796".to_string()),
                            ("phase1Cycles".to_string(), "487045936".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "282430982".to_string()),
                            ("cyclesPerTuple".to_string(), "117".to_string()),
                            ("throughput".to_string(), "29.0244".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("threads".to_string(), "3".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 4,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "285763134".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("phase1Cycles".to_string(), "652819238".to_string()),
                            ("threads".to_string(), "4".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("cyclesPerTuple".to_string(), "143".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "275417".to_string()),
                            ("throughput".to_string(), "23.7952".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 5,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase1Cycles".to_string(), "784086642".to_string()),
                            ("cyclesPerTuple".to_string(), "164".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "315886".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("throughput".to_string(), "20.7467".to_string()),
                            ("threads".to_string(), "5".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "292418104".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 7,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase2Time".to_string(), "5495".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "435847".to_string()),
                            ("phase1Time".to_string(), "430352".to_string()),
                            ("throughput".to_string(), "15.0365".to_string()),
                            ("threads".to_string(), "7".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("cyclesPerTuple".to_string(), "226".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 8,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "5242880".to_string()),
                            ("cyclesPerTuple".to_string(), "227".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("threads".to_string(), "8".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("phase1Time".to_string(), "431362".to_string()),
                            ("phase2Time".to_string(), "5497".to_string()),
                            ("totalTime".to_string(), "436859".to_string()),
                            ("throughput".to_string(), "15.0016".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 5,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relR".to_string(), "1310720".to_string()),
                            ("throughput".to_string(), "14.9472".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("threads".to_string(), "5".to_string()),
                            ("phase1Time".to_string(), "433048".to_string()),
                            ("cyclesPerTuple".to_string(), "227".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "5402".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "438450".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 1,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("cyclesPerTuple".to_string(), "226".to_string()),
                            ("phase2Time".to_string(), "5462".to_string()),
                            ("totalTime".to_string(), "435478".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("threads".to_string(), "1".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase1Time".to_string(), "430016".to_string()),
                            ("throughput".to_string(), "15.0492".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 4,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("cyclesPerTuple".to_string(), "226".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase1Time".to_string(), "430985".to_string()),
                            ("phase2Time".to_string(), "5406".to_string()),
                            ("totalTime".to_string(), "436391".to_string()),
                            ("throughput".to_string(), "15.0177".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("threads".to_string(), "4".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 7,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("cyclesPerTuple".to_string(), "217".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "381193574".to_string()),
                            ("totalTime".to_string(), "417966".to_string()),
                            ("throughput".to_string(), "15.6797".to_string()),
                            ("threads".to_string(), "7".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("phase1Cycles".to_string(), "1043157714".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 8,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase1Time".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "989".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("throughput".to_string(), "3.4450".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("threads".to_string(), "8".to_string()),
                            ("totalTime".to_string(), "1902337".to_string()),
                            ("phase2Cycles".to_string(), "3419103076".to_string()),
                            ("phase1Cycles".to_string(), "3063948414".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 1,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relR".to_string(), "1310720".to_string()),
                            ("phase2Cycles".to_string(), "43284636".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "762161740".to_string()),
                            ("totalTime".to_string(), "236350".to_string()),
                            ("cyclesPerTuple".to_string(), "122".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("threads".to_string(), "1".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("throughput".to_string(), "27.7284".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Algorithm::Commit(1),
                            threads: 6,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("throughput".to_string(), "14.9652".to_string()),
                            ("phase2Time".to_string(), "5456".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("cyclesPerTuple".to_string(), "227".to_string()),
                            ("phase1Time".to_string(), "432467".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("threads".to_string(), "6".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "437923".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheFit,
                            algorithm: Rho,
                            threads: 6,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("totalTime".to_string(), "322788".to_string()),
                            ("relS".to_string(), "5242880".to_string()),
                            ("cyclesPerTuple".to_string(), "167".to_string()),
                            ("throughput".to_string(), "20.3031".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "298642494".to_string()),
                            ("phase1Cycles".to_string(), "801364368".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("matches".to_string(), "5242880".to_string()),
                            ("relR".to_string(), "1310720".to_string()),
                            ("threads".to_string(), "6".to_string()),
                        ]),
                    ),
                ],
                findings: vec![],
            },
            ExperimentChart {
                config: PerfReport(PerfReportConfig {
                    id: 1,
                    exp_type: Scalability,
                    dataset: CacheExceed,
                    baseline: Rho,
                }),
                results: vec![
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 4,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("throughput".to_string(), "26.0781".to_string()),
                            ("phase1Cycles".to_string(), "8286369108".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("threads".to_string(), "4".to_string()),
                            ("phase2Cycles".to_string(), "278079694".to_string()),
                            ("cyclesPerTuple".to_string(), "130".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("totalTime".to_string(), "2513062".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 6,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase2Cycles".to_string(), "348332420".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "9178323368".to_string()),
                            ("cyclesPerTuple".to_string(), "145".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("threads".to_string(), "6".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "2795399".to_string()),
                            ("throughput".to_string(), "23.4442".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase1Time".to_string(), "5008557".to_string()),
                            ("phase2Time".to_string(), "55516".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "263".to_string()),
                            ("threads".to_string(), "2".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("totalTime".to_string(), "5064073".to_string()),
                            ("throughput".to_string(), "12.9414".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 8,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase1Time".to_string(), "5006090".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Time".to_string(), "55276".to_string()),
                            ("throughput".to_string(), "12.9483".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("totalTime".to_string(), "5061366".to_string()),
                            ("threads".to_string(), "8".to_string()),
                            ("cyclesPerTuple".to_string(), "263".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 1,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("threads".to_string(), "1".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("totalTime".to_string(), "5121168".to_string()),
                            ("throughput".to_string(), "12.7971".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase1Time".to_string(), "5065499".to_string()),
                            ("phase2Time".to_string(), "55669".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "266".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 4,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Time".to_string(), "55399".to_string()),
                            ("throughput".to_string(), "12.9485".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("cyclesPerTuple".to_string(), "263".to_string()),
                            ("phase1Time".to_string(), "5005885".to_string()),
                            ("totalTime".to_string(), "5061284".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("threads".to_string(), "4".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 6,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "52428800".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("cyclesPerTuple".to_string(), "262".to_string()),
                            ("phase1Time".to_string(), "5002025".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("threads".to_string(), "6".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase2Time".to_string(), "55275".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("totalTime".to_string(), "5057300".to_string()),
                            ("throughput".to_string(), "12.9587".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 5,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "52428800".to_string()),
                            ("threads".to_string(), "5".to_string()),
                            ("totalTime".to_string(), "2667828".to_string()),
                            ("throughput".to_string(), "24.5653".to_string()),
                            ("phase1Cycles".to_string(), "8766491270".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Cycles".to_string(), "325404832".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("cyclesPerTuple".to_string(), "138".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 7,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "52428800".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "9424344144".to_string()),
                            ("threads".to_string(), "7".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("cyclesPerTuple".to_string(), "190".to_string()),
                            ("throughput".to_string(), "17.8528".to_string()),
                            ("totalTime".to_string(), "3670906".to_string()),
                            ("phase2Cycles".to_string(), "3085984936".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 1,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "52428800".to_string()),
                            ("totalTime".to_string(), "3416459".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase2Cycles".to_string(), "346621958".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase1Cycles".to_string(), "11296600124".to_string()),
                            ("threads".to_string(), "1".to_string()),
                            ("cyclesPerTuple".to_string(), "177".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("throughput".to_string(), "19.1824".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 7,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("phase2Time".to_string(), "55665".to_string()),
                            ("phase1Time".to_string(), "4998130".to_string()),
                            ("cyclesPerTuple".to_string(), "262".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("threads".to_string(), "7".to_string()),
                            ("totalTime".to_string(), "5053795".to_string()),
                            ("throughput".to_string(), "12.9677".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase1Cycles".to_string(), "0".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 3,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("matches".to_string(), "52428800".to_string()),
                            ("threads".to_string(), "3".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("cyclesPerTuple".to_string(), "263".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("phase1Time".to_string(), "5016109".to_string()),
                            ("phase2Time".to_string(), "56036".to_string()),
                            ("totalTime".to_string(), "5072145".to_string()),
                            ("throughput".to_string(), "12.9208".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 8,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("throughput".to_string(), "17.9746".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("threads".to_string(), "8".to_string()),
                            ("cyclesPerTuple".to_string(), "189".to_string()),
                            ("totalTime".to_string(), "3646033".to_string()),
                            ("phase1Cycles".to_string(), "9661897054".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "2763660838".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 2,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("threads".to_string(), "2".to_string()),
                            ("phase2Cycles".to_string(), "187494972".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("totalTime".to_string(), "2442270".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("throughput".to_string(), "26.8341".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("cyclesPerTuple".to_string(), "127".to_string()),
                            ("phase1Cycles".to_string(), "8135700342".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Algorithm::Commit(1),
                            threads: 5,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("relS".to_string(), "52428800".to_string()),
                            ("phase1Time".to_string(), "4996703".to_string()),
                            ("phase2Cycles".to_string(), "0".to_string()),
                            (
                                "algorithm".to_string(),
                                "SortMergeJoin_QuickSort".to_string(),
                            ),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase1Cycles".to_string(), "0".to_string()),
                            ("cyclesPerTuple".to_string(), "262".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("totalTime".to_string(), "5051978".to_string()),
                            ("phase2Time".to_string(), "55275".to_string()),
                            ("throughput".to_string(), "12.9723".to_string()),
                            ("threads".to_string(), "5".to_string()),
                        ]),
                    ),
                    (
                        TeebenchArgs {
                            app_name: Sgx,
                            dataset: CacheExceed,
                            algorithm: Rho,
                            threads: 3,
                            selectivity: 100,
                            data_skew: "0".to_string(),
                            seal_chunk_size: 0,
                            r_tuples: 2097152,
                            s_tuples: 2097152,
                            r_path: None,
                            s_path: None,
                            x: None,
                            y: None,
                            seal: false,
                            sort_r: false,
                            sort_s: false,
                            csv: true,
                        },
                        HashMap::from([
                            ("cyclesPerTuple".to_string(), "122".to_string()),
                            ("throughput".to_string(), "27.7553".to_string()),
                            ("phase1Time".to_string(), "0".to_string()),
                            ("totalTime".to_string(), "2361206".to_string()),
                            ("relS".to_string(), "52428800".to_string()),
                            ("relR".to_string(), "13107200".to_string()),
                            ("phase1Cycles".to_string(), "7906488236".to_string()),
                            ("algorithm".to_string(), "RHO".to_string()),
                            ("threads".to_string(), "3".to_string()),
                            ("matches".to_string(), "52428800".to_string()),
                            ("phase2Time".to_string(), "0".to_string()),
                            ("phase2Cycles".to_string(), "140443000".to_string()),
                        ]),
                    ),
                ],
                findings: vec![],
            },
        ],
        findings: vec![],
    })));
    let mut c = Commit::new(
        "merge_example".to_string(),
        "0.0.1".to_string(),
        Operator::Join,
        time::macros::datetime!(2023 - 04 - 25 12:00 +01),
        merge_,
        report,
        1,
        Algorithm::Rho,
    );
    c.compilation = crate::commit::CompilationStatus::Successful("".to_string());
    c
}
