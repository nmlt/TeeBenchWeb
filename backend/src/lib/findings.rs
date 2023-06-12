use anyhow::{bail, Result};
use common::data_types::{
    Algorithm, Dataset, ExperimentType, FindingStyle, JobConfig, Measurement, Parameter, Platform,
    Report, UnwrapedExperimentResult, CPU_PHYSICAL_CORES,
};
use std::iter::zip;
use tracing::instrument;
use tracing::log::info;

#[instrument(skip(jr))]
pub fn enrich_report_with_findings(jr: &mut Report) -> Result<()> {
    let mut epc_commit_ewb: Vec<i32> = vec![];
    let mut epc_baseline_ewb: Vec<i32> = vec![];
    let mut throughput_improvements: Vec<f32> = vec![];

    // 1. iterate over each experiment chart and enrich it with findings
    for ex in &mut jr.charts {
        if ex.results.iter().any(|res| res.1.is_err()) {
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
                                        jr.findings.push(common::data_types::Finding {
                                            title: "Check for excessive OCALLS".to_string(),
                                            message: format!(
                                                "You can check for excessive OCALLS by looking up Voluntary Context Switches"
                                            ),
                                            style: FindingStyle::Neutral,
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
                                        jr.findings.push(common::data_types::Finding {
                                            title: "Check for excessive OCALLS".to_string(),
                                            message: format!(
                                                "You can check for excessive OCALLS by looking up Voluntary Context Switches"
                                            ),
                                            style: FindingStyle::Neutral,
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
                            //Throughput(algorithms)
                            Parameter::Algorithms => {}
                            Parameter::OuterTableSize => {}
                        }
                    }
                    Measurement::TotalEpcPaging => {}
                    Measurement::ThroughputAndTotalEPCPaging => {}
                    Measurement::ThroughputAndContextSwitches => {}
                    Measurement::ContextSwitches => {
                        // do it only once
                        if jr.findings.len() == 0 {
                            match results.iter().find(|(_, r)| {
                                r.get("totalVoluntaryCS").unwrap().parse::<i32>().unwrap() > 1000
                            }) {
                                None => {}
                                Some(_) => {
                                    jr.findings.push(common::data_types::Finding {
                                        title: "Detected excessive voluntary CPU context switches"
                                            .to_string(),
                                        message: format!(""),
                                        style: FindingStyle::Bad,
                                    });
                                    jr.findings.push(common::data_types::Finding {
                                        title: "Try mutex-free data structures".to_string(),
                                        message: format!(""),
                                        style: FindingStyle::Neutral,
                                    });
                                }
                            }
                        }
                    }
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
                ExperimentType::EpcPaging => {
                    let mut res_tmp = results.clone();
                    res_tmp.sort_by_key(|(t, _)| t.x.unwrap());
                    let total_ewbs: Vec<i32> = res_tmp
                        .iter()
                        .map(|(_, b)| b.get("totalEWB").unwrap().parse::<i32>().unwrap())
                        .collect();
                    info!("total_ewbs: {:?}", total_ewbs);
                    match c.baseline {
                        Algorithm::Commit(_) => {
                            epc_commit_ewb = total_ewbs;
                        }
                        _ => {
                            epc_baseline_ewb = total_ewbs;
                        }
                    }
                }
                ExperimentType::Throughput => {}
                ExperimentType::Scalability => {
                    let mut tmp = results.clone();
                    tmp.sort_by_key(|(t, _)| t.threads);
                    let throughput_commit: Vec<f32> = tmp
                        .iter()
                        .filter(|(a, _)| match a.algorithm {
                            Algorithm::Commit(_) => true,
                            _ => false,
                        })
                        .map(|(_, r)| r.get("throughput").unwrap().parse::<f32>().unwrap())
                        .collect();
                    let throughput_baseline: Vec<f32> = tmp
                        .iter()
                        .filter(|(a, _)| match a.algorithm {
                            Algorithm::Commit(_) => false,
                            _ => true,
                        })
                        .map(|(_, r)| r.get("throughput").unwrap().parse::<f32>().unwrap())
                        .collect();
                    for (a, b) in zip(throughput_commit, throughput_baseline) {
                        let impr = ((a - b) / b) * 100.0;
                        throughput_improvements.push(impr);
                    }
                }
                ExperimentType::Custom => {}
            },
            JobConfig::Compile(_) => {}
        }
    }
    if jr.charts.len() == 2 {
        match (jr.charts[0].clone().config, jr.charts[1].clone().config) {
            (JobConfig::Profiling(c0), JobConfig::Profiling(c1)) => {
                match (c0.parameter, c0.measurement, c1.parameter, c1.measurement) {
                    (
                        Parameter::Algorithms,
                        Measurement::Throughput,
                        Parameter::Algorithms,
                        Measurement::Throughput,
                    ) => {
                        // if there are two datasets:
                        // compare throughput per algorithm - report if throughput goes significantly down
                        //compare EPC paging for both datasets - report if EPC paging goes significantly up
                        let res0: Vec<(Algorithm, Dataset, String, String)> = jr
                            .charts
                            .get(0)
                            .unwrap()
                            .results
                            .iter()
                            .map(|(a, b)| {
                                (
                                    a.algorithm,
                                    a.dataset,
                                    b.clone().unwrap().get("throughput").unwrap().clone(),
                                    b.clone().unwrap().get("totalEWB").unwrap().clone(),
                                )
                            })
                            .collect();
                        let res1: Vec<(Algorithm, Dataset, String, String)> = jr
                            .charts
                            .get(1)
                            .unwrap()
                            .results
                            .iter()
                            .map(|(a, b)| {
                                (
                                    a.algorithm,
                                    a.dataset,
                                    b.clone().unwrap().get("throughput").unwrap().clone(),
                                    b.clone().unwrap().get("totalEWB").unwrap().clone(),
                                )
                            })
                            .collect();
                        for i in &res0 {
                            match res1.iter().find(|(a, _, _, _)| a == &i.0) {
                                None => (),
                                Some(r) => {
                                    let _t0 = i.2.parse::<f32>().unwrap();
                                    let _t1 = r.2.parse::<f32>().unwrap();
                                    let epc0 = i.3.parse::<i32>().unwrap();
                                    let epc1 = r.3.parse::<i32>().unwrap();

                                    if (epc1 / epc0) > 1000 {
                                        jr.findings.push(common::data_types::Finding {
                                            title: "Excessive EPC Paging".to_string(),
                                            message: format!(
                                                "Increase by {}x for {}",
                                                (epc1 / epc0),
                                                r.0.to_string(),
                                            ),
                                            style: FindingStyle::Bad,
                                        });
                                        jr.findings.push(common::data_types::Finding {
                                            title:
                                                "Reduce memory consumption and avoid random access"
                                                    .to_string(),
                                            message: "".to_string(),
                                            style: FindingStyle::Neutral,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    // 2. add top-level findings

    // report average throughput change
    if throughput_improvements.len() > 0 {
        let sum = throughput_improvements.iter().sum::<f32>();
        let avg = sum / throughput_improvements.len() as f32;
        let style;
        let message;
        if avg > 0.0 {
            message = format!("Throughput increased by {}%!", avg as i32);
            style = FindingStyle::Good;
        } else {
            message = format!("Throughput deacresed by {}%.", avg as i32);
            style = FindingStyle::Bad
        }
        jr.findings.push(common::data_types::Finding {
            title: message,
            message: "".to_string(),
            style,
        });
    }

    // calculate average EPC paging change
    if epc_baseline_ewb.len() > 0 && epc_baseline_ewb.len() == epc_commit_ewb.len() {
        let improvements: Vec<f32> = epc_baseline_ewb
            .iter()
            .zip(epc_commit_ewb)
            .map(|(a, b)| ((*a as f32 - b as f32) / b as f32) * 100.0)
            .collect();

        let sum = improvements.iter().sum::<f32>();
        let avg = sum / epc_baseline_ewb.clone().len() as f32;
        let style;
        let message;
        if avg > 0.0 {
            let max = improvements.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
            message = format!("Reduced EPC Paging by up to {}%!", *max as i32);
            style = FindingStyle::Good;
        } else {
            message = format!("Increased EPC Paging by {}% on average.", avg as i32);
            style = FindingStyle::Bad
        }
        jr.findings.push(common::data_types::Finding {
            title: message,
            message: "".to_string(),
            style,
        });
    }

    Ok(())
}
