use common::data_types::{
    Dataset, ExperimentChartResult, ExperimentType, FindingStyle, JobConfig, Measurement,
    Parameter, Platform, Report,
};
use tracing::instrument;

// Machine-dependent variables
const CPU_PHYSICAL_CORES: u8 = 4;
// const CPU_LOGICAL_CORES: i32  = 16;
// const L1_SIZE_KB: i32        = 256;
// const L2_SIZE_KB: i32        = 2048;
// const L3_SIZE_KB: i32        = 16384;
// const EPC_SIZE_KB: i32       = 262144; // 256 MB

#[instrument]
pub fn enrich_report_with_findings(jr: &mut Report) {
    // 1. iterate over each experiment chart and enrich it with findings
    for ex in &mut jr.charts {
        match &ex.config {
            JobConfig::Profiling(c) => {
                match c.measurement {
                    Measurement::Throughput => {
                        match c.parameter {
                            Parameter::Threads => {
                                let max_threads = ex
                                    .results
                                    .iter()
                                    .map(|(_, a)| a.get("threads").unwrap().parse::<u8>().unwrap())
                                    .max()
                                    .unwrap();

                                let max_result = ex
                                    .results
                                    .iter()
                                    .filter(|t| {
                                        t.0.app_name == Platform::Sgx
                                            && t.0.dataset == Dataset::CacheFit
                                    })
                                    .max_by(|(_, a), (_, b)| {
                                        a.get("throughput")
                                            .unwrap()
                                            .parse::<f32>()
                                            .unwrap()
                                            .partial_cmp(
                                                &b.get("throughput")
                                                    .unwrap()
                                                    .parse::<f32>()
                                                    .unwrap(),
                                            )
                                            .unwrap()
                                    });

                                if let Some(max_result) = max_result {
                                    jr.findings.push(common::data_types::Finding {
                                        title: "Max Throughput".to_string(),
                                        message: format!(
                                            "{:?} [M rec/s]",
                                            max_result
                                                .1
                                                .get("throughput")
                                                .unwrap()
                                                .parse::<f32>()
                                                .unwrap()
                                        ),
                                        style: FindingStyle::Good,
                                    });

                                    if max_result.0.threads + 2 < CPU_PHYSICAL_CORES
                                        && max_threads != max_result.0.threads
                                    {
                                        jr.findings.push(common::data_types::Finding {
                                            title: "Very Poor Scalability".to_string(),
                                            message: format!(
                                                "Used only {:?}/{:?} physical cores",
                                                max_result.0.threads, CPU_PHYSICAL_CORES
                                            ),
                                            style: FindingStyle::Bad,
                                        });
                                    } else if max_result.0.threads + 1 < CPU_PHYSICAL_CORES
                                        && max_threads != max_result.0.threads
                                    {
                                        jr.findings.push(common::data_types::Finding {
                                            title: "Poor Scalability".to_string(),
                                            message: format!(
                                                "Used only {:?}/{:?} physical cores",
                                                max_result.0.threads, CPU_PHYSICAL_CORES
                                            ),
                                            style: FindingStyle::SoSo,
                                        });
                                    } else {
                                        jr.findings.push(common::data_types::Finding {
                                            title: "Good Scalability".to_string(),
                                            message: format!(
                                                "Best for {:?} threads",
                                                max_result.0.threads
                                            ),
                                            style: FindingStyle::Good,
                                        });
                                    }
                                }
                                let mut ht_improved_algorithms: Vec<String> = Vec::<String>::new();
                                let mut ht_max_improvement: f32 = 1 as f32;

                                for a in c.algorithms.iter() {
                                    // find max throughput
                                    let ht_results: ExperimentChartResult = ex
                                        .results
                                        .iter()
                                        .filter(|t| {
                                            t.0.app_name == Platform::Sgx
                                                && t.0.dataset == Dataset::CacheFit
                                                && t.0.algorithm == *a
                                        })
                                        .filter(|(_, r)| {
                                            r.get("threads").unwrap().parse::<u8>().unwrap()
                                                > CPU_PHYSICAL_CORES
                                        })
                                        .map(|a| a.clone())
                                        .collect::<ExperimentChartResult>();

                                    let non_ht_results: ExperimentChartResult = ex
                                        .results
                                        .iter()
                                        .filter(|t| {
                                            t.0.app_name == Platform::Sgx
                                                && t.0.dataset == Dataset::CacheFit
                                                && t.0.algorithm == *a
                                        })
                                        .filter(|(_, r)| {
                                            r.get("threads").unwrap().parse::<u8>().unwrap()
                                                <= CPU_PHYSICAL_CORES
                                        })
                                        .map(|a| a.clone())
                                        .collect::<ExperimentChartResult>();

                                    let ht_max_throughput = ht_results
                                        .iter()
                                        .map(|(_, a)| a.get("throughput"))
                                        .map(|a| match a {
                                            Some(x) => x.parse::<f32>().unwrap(),
                                            None => 0 as f32,
                                        })
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap_or_default();

                                    let non_ht_max_throughput = non_ht_results
                                        .iter()
                                        .map(|(_, a)| a.get("throughput"))
                                        .map(|a| match a {
                                            Some(x) => x.parse::<f32>().unwrap(),
                                            None => 0 as f32,
                                        })
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap_or_default();

                                    let ht_improvement = ht_max_throughput / non_ht_max_throughput;
                                    let ht_improvement = if ht_improvement.is_infinite() {
                                        0 as f32
                                    } else {
                                        ht_improvement
                                    };

                                    if ht_improvement > 1 as f32 {
                                        ht_improved_algorithms.push(a.to_string());
                                        if ht_improvement > ht_max_improvement {
                                            ht_max_improvement = ht_improvement;
                                        }
                                    }
                                }
                                if ht_improved_algorithms.len() > 0 {
                                    jr.findings.push(common::data_types::Finding {
                                        title: "Hyper Threading".to_string(),
                                        message: format!(
                                            "Improved: {:?} by up to {:?}%",
                                            ht_improved_algorithms,
                                            (ht_max_improvement * 100 as f32 - 100 as f32)
                                        ),
                                        style: FindingStyle::Good,
                                    });
                                } else {
                                    jr.findings.push(common::data_types::Finding {
                                        title: "Hyper Threading".to_string(),
                                        message: format!("No algorithm benefits from HT"),
                                        style: FindingStyle::Bad,
                                    });
                                }

                                // calculate the diff and evaluate
                                // is max throughput close to pcores? --> add finding if the algorithm scales at all
                                // is throughput going down? --> add finding to check CPU context switches
                            }
                            Parameter::DataSkew => {}
                            Parameter::JoinSelectivity => {}
                        }
                    }
                    Measurement::EpcPaging => {}
                }
            }
            JobConfig::PerfReport(c) => match c.exp_type {
                ExperimentType::EpcPaging => {}
                ExperimentType::Throughput => {}
                ExperimentType::Scalability => {}
                ExperimentType::Custom => {}
            },
            JobConfig::Compile(_) => {}
        }
    }
    // 2. add top-level findings
}
