use anyhow::{bail, Result};
use common::data_types::{
    Dataset, ExperimentChartResult, ExperimentType, FindingStyle, JobConfig, Measurement,
    Parameter, Platform, Report, UnwrapedExperimentResult, CPU_PHYSICAL_CORES,
};
use tracing::instrument;

#[instrument]
pub fn enrich_report_with_findings(jr: &mut Report) -> Result<()> {
    // 1. iterate over each experiment chart and enrich it with findings
    for ex in &mut jr.charts {
        if ex.results.iter().all(|res| !res.1.is_ok()) {
            bail!("Some results are errors!");
        };
        let results: UnwrapedExperimentResult = ex
            .results
            .iter()
            .map(|res| (res.0.clone(), res.1.clone().unwrap()))
            .collect();
        match &ex.config {
            JobConfig::Profiling(c) => {
                match c.measurement {
                    Measurement::Throughput => {
                        match c.parameter {
                            Parameter::Threads => {
                                let max_threads = results
                                    .iter()
                                    .map(|(_, a)| a.get("threads").unwrap().parse::<u8>().unwrap())
                                    .max()
                                    .unwrap();

                                let max_result = results
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
                                    let ht_results: UnwrapedExperimentResult = results
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
                                        .collect();

                                    let non_ht_results: UnwrapedExperimentResult = results
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
                                        .collect();

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
                            Parameter::Algorithms => {}
                            Parameter::OuterTableSize => {}
                        }
                    }
                    Measurement::TotalEpcPaging => {}
                    Measurement::ThroughputAndTotalEPCPaging => {}
                    Measurement::Phase1Cycles => {}
                    Measurement::Phase2Cycles => {}
                    Measurement::TotalCycles => {}
                    Measurement::TotalL2HitRatio => {}
                    Measurement::TotalL3HitRatio => {}
                    Measurement::TotalL2CacheMisses => {}
                    Measurement::TotalL3CacheMisses => {}
                    Measurement::IPC => {}
                    Measurement::IR => {}
                    Measurement::TotalVoluntaryCS => {}
                    Measurement::TotalInvoluntaryCS => {}
                    Measurement::TotalUserCpuTime => {}
                    Measurement::TotalSystemCpuTime => {}
                    Measurement::TwoPhasesCycles => {
                        // find the fastest 1 phase
                        let slowest_1phase = results
                            .iter()
                            .max_by(|(_, r1), (_, r2)| {
                                r1.get("phase1Cycles")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap()
                                    .partial_cmp(
                                        &r2.get("phase1Cycles").unwrap().parse::<u64>().unwrap(),
                                    )
                                    .unwrap()
                            })
                            .unwrap()
                            .clone();

                        let slowest_2phase = results
                            .iter()
                            .max_by(|(_, r1), (_, r2)| {
                                r1.get("phase2Cycles")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap()
                                    .partial_cmp(
                                        &r2.get("phase2Cycles").unwrap().parse::<u64>().unwrap(),
                                    )
                                    .unwrap()
                            })
                            .unwrap()
                            .clone();
                        if slowest_1phase
                            .1
                            .get("phase1Cycles")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap_or_default()
                            > slowest_2phase
                                .1
                                .get("phase2Cycles")
                                .unwrap()
                                .parse::<u64>()
                                .unwrap_or_default()
                        {
                            jr.findings.push(common::data_types::Finding {
                                title: "Slowest Phase".to_string(),
                                message: format!(
                                    "{:?} Phase 1: {:?} CPU Cycles",
                                    slowest_1phase.1.get("algorithm").unwrap(),
                                    slowest_1phase.1.get("phase1Cycles").unwrap(),
                                ),
                                style: FindingStyle::Bad,
                            });
                        } else {
                            jr.findings.push(common::data_types::Finding {
                                title: "Slowest Phase".to_string(),
                                message: format!(
                                    "{:?} Phase 2: {:?} CPU Cycles",
                                    slowest_2phase.1.get("algorithm").unwrap(),
                                    slowest_2phase.1.get("phase2Cycles").unwrap(),
                                ),
                                style: FindingStyle::Bad,
                            });
                        }
                        // find the fastest phase
                        let fastest_1phase = results
                            .iter()
                            .filter(|(_, r)| {
                                r.get("phase1Cycles")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap_or_default()
                                    > 0
                            })
                            .min_by(|(_, r1), (_, r2)| {
                                r1.get("phase1Cycles")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap()
                                    .partial_cmp(
                                        &r2.get("phase1Cycles").unwrap().parse::<u64>().unwrap(),
                                    )
                                    .unwrap()
                            })
                            .unwrap()
                            .clone();

                        let fastest_2phase = results
                            .iter()
                            .filter(|(_, r)| {
                                r.get("phase2Cycles")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap_or_default()
                                    > 0
                            })
                            .min_by(|(_, r1), (_, r2)| {
                                r1.get("phase2Cycles")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap()
                                    .partial_cmp(
                                        &r2.get("phase2Cycles").unwrap().parse::<u64>().unwrap(),
                                    )
                                    .unwrap()
                            })
                            .unwrap()
                            .clone();
                        if fastest_1phase
                            .1
                            .get("phase1Cycles")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap_or_default()
                            < fastest_2phase
                                .1
                                .get("phase2Cycles")
                                .unwrap()
                                .parse::<u64>()
                                .unwrap_or_default()
                        {
                            jr.findings.push(common::data_types::Finding {
                                title: "Fastest Phase".to_string(),
                                message: format!(
                                    "{:?} Phase 1: {:?} CPU Cycles",
                                    fastest_1phase.1.get("algorithm").unwrap(),
                                    fastest_1phase.1.get("phase1Cycles").unwrap(),
                                ),
                                style: FindingStyle::Good,
                            });
                        } else {
                            jr.findings.push(common::data_types::Finding {
                                title: "Fastest Phase".to_string(),
                                message: format!(
                                    "{:?} Phase 2: {:?} CPU Cycles",
                                    fastest_2phase.1.get("algorithm").unwrap(),
                                    fastest_2phase.1.get("phase2Cycles").unwrap(),
                                ),
                                style: FindingStyle::Good,
                            });
                        }
                        // find the most imbalanced algorithm
                    }
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
    Ok(())
}
