use crate::{
    commandline::Commandline,
    data_types::{Algorithm, Platform, REPLACE_ALG},
};

pub fn hardcoded_perf_report_commands(baseline: &Algorithm) -> Vec<Commandline> {
    let baseline = &baseline.to_string();
    #[rustfmt::skip]
    let res = vec![
        // Throughput Cache-Fit
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
        Commandline::with_args(Platform::Native,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
        Commandline::with_args(Platform::Native,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
        // Throughput Cache-Exceed
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
        Commandline::with_args(Platform::Native,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
        Commandline::with_args(Platform::Native,&vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
        // Scalability Cache-Fit
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","1","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","1","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","3","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","3","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","4","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","4","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","5","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","5","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","6","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","6","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","7","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","7","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-fit"   ,"-n","8","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-fit"   ,"-n","8","--csv"]),
        // Scalability Cache-Exceed
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","1","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","1","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","2","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","3","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","3","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","4","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","4","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","5","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","5","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","6","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","6","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","7","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","7","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",REPLACE_ALG,"-d","cache-exceed","-n","8","--csv"]),
        Commandline::with_args(Platform::Sgx   ,&vec!["-a",baseline   ,"-d","cache-exceed","-n","8","--csv"]),
        // EPC Paging Commit
        // TODO
        // EPC Paging baseline
        // TODO
    ];
    res
}
