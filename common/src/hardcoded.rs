use crate::{
    commandline::Commandline,
    commit::CommitIdType,
    data_types::{Algorithm, JobConfig, PerfReportConfig, Platform, REPLACE_ALG},
};

/// Get the data out of the results vector of the `ExperimentChart`, depending on which experiment it is.
///
/// A function here, to be easily changed if we change the order of the experiments in `hardcoded_perf_report_commands`.
// pub fn get_data_perf_report(baseline: bool, exp_type: ExperimentType, dataset: Dataset) -> Vec<f64> {
//     match exp_type {
//         ExperimentType::Throughput => {
//             match dataset {
//                 Dataset::CacheFit => {
//                     if baseline {

//                     }
//                 }
//             }
//         }
//         ExperimentType::Custom => {}
//         ExperimentType::EpcPaging => {}
//         ExperimentType::Scalability => {}
//     }
// }

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
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Native, commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Native, *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
        ],
        // Throughput Cache-Exceed
        vec![
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Native, *baseline_t, &vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Native, commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
        ],
        // Scalability Cache-Fit
        vec![
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","8","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","8","--csv"]),
        ],
            // Scalability Cache-Exceed
        vec![
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   , commit_id  , &vec!["-a",baseline   ,"-d","cache-exceed","-n","8","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","1","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","3","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","4","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","5","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","6","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","7","--csv"]),
            Commandline::with_args(Platform::Sgx   , *baseline_t, &vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","8","--csv"]),
        ],
        // EPC Paging Commit
        // TODO
        // EPC Paging baseline
        // TODO
    ];
    res
}
