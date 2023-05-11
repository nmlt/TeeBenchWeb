use crate::data_types::{Algorithm, Platform, TeebenchArgs};
use std::fmt::Display;
use structopt::StructOpt;

/// Commandline is a builder for a std::process::Command or its tokio equivalent.
/// The actual `std::process::Command` struct cannot be `Clone`, so this is needed to easily pass it around before actually running the command.
/// Sadly, I cannot include a method to create a `tokio::process::Command` from this, because including tokio in common is impossible: the frontend also uses the common crate, and you cannot use tokio in a webapp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commandline {
    pub app: Platform,
    pub algorithm: Algorithm,
    pub args: Vec<String>, // Not a HashSet because order is important and flags could be passed multiple times.
}

impl Commandline {
    pub fn new(platform: Platform, algorithm: Algorithm) -> Self {
        Self {
            app: platform,
            algorithm,
            args: vec![],
        }
    }
    pub fn with_args(platform: Platform, algorithm: Algorithm, args: &[&str]) -> Self {
        let args = args.iter().map(|a| a.to_string()).collect();
        Self {
            app: platform,
            algorithm,
            args,
        }
    }
    pub fn add_args<S: Display>(&mut self, name: &str, value: S) {
        self.args.push(name.to_string());
        self.args.push(value.to_string());
    }
    pub fn add_flag(&mut self, name: &str) {
        self.args.push(name.to_string());
    }
    /// Adds all the values in `iter` as values of the last option of the `Commandline`s in `cmds` for each item in `cmds`.
    /// Example: `cmds` =  `["./app -a CHT"]` becomes `["./app -a CHT", "./app -a RHO"] if `iter` contains "RHO".
    /// Panics if `cmds` is empty.
    pub fn double_cmds_with_different_arg_value<S: Display, I: Iterator<Item = S>>(
        cmds: &mut Vec<Commandline>,
        iter: &mut I,
    ) {
        let l = cmds.len();
        for val in iter {
            let curr_l = cmds.len();
            cmds.extend_from_within(0..l);
            for cmd in cmds.iter_mut().skip(curr_l) {
                let d_arg = cmd.args.last_mut().unwrap();
                *d_arg = val.to_string();
            }
        }
    }
    pub fn to_teebench_args(&self) -> TeebenchArgs {
        let app_name = vec![self.app.to_app_name()];
        let iter = app_name.iter().chain(self.args.iter());
        let mut args = TeebenchArgs::from_iter_safe(iter).unwrap();
        // TODO Remove app_name as the way to determine the platform, use an environment variable and compile it two times.
        if args.x.is_some() || args.y.is_some() {
            if let Some(x) = args.x {
                if let Some(y) = args.y {
                    args.dataset = crate::data_types::Dataset::CustomSize{ x, y };
                } else {
                    args.dataset = crate::data_types::Dataset::CustomSize{ x, y: 128 };
                }
            } else {
                if let Some(y) = args.y {
                    args.dataset = crate::data_types::Dataset::CustomSize{ x: 16, y };
                } else {
                    args.dataset = crate::data_types::Dataset::CustomSize{ x: 16, y: 128 };
                }
            }
        }
        args.app_name = self.app;
        args.algorithm = self.algorithm;
        args
    }
}

impl std::fmt::Display for Commandline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_joined = self.args.join(" ");
        write!(f, "{} {}", self.app.to_app_name(), args_joined)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_types::{
        Algorithm, Dataset, ExperimentType, Measurement, Parameter, ProfilingConfiguration,
    };

    #[test]
    fn profiling_configuration_to_teebench_cmd_default() {
        let c = ProfilingConfiguration::new(
            vec![Algorithm::Cht],
            ExperimentType::Custom,
            Parameter::Threads,
            Measurement::Throughput,
            2.to_string(),
            2.to_string(),
            2.to_string(),
            vec![Dataset::CacheFit],
            vec![Platform::Sgx],
            false,
        );
        // let mut cmd = Commandline::new(&Platform::Sgx);
        // cmd.add_args("-a", "CHT");
        // cmd.add_args("-d", "Cache Fit");
        // cmd.add_args("-n", "2");
        let cmd = Commandline::with_args(
            Platform::Sgx,
            Algorithm::Cht,
            &vec!["-a", "CHT", "-d", "cache-fit", "-n", "2", "--csv"],
        );
        for (to_be_tested, expected) in c.to_teebench_cmd().iter().zip(vec![vec![cmd]]) {
            assert_eq!(to_be_tested, &expected);
        }
    }

    #[test]
    fn profiling_configuration_to_teebench_cmd_multiple_cmds() {
        let c = ProfilingConfiguration::new(
            vec![Algorithm::Cht, Algorithm::Rho],
            ExperimentType::default(),
            Parameter::DataSkew,
            Measurement::default(),
            2.to_string(),
            8.to_string(),
            2.to_string(),
            vec![Dataset::CacheExceed, Dataset::CacheFit],
            vec![Platform::Sgx, Platform::Native],
            true,
        );
        #[rustfmt::skip]
        let cmds = vec![
            vec![
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            ],
            vec![
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Cht, &vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            Commandline::with_args(Platform::Sgx   , Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            Commandline::with_args(Platform::Native, Algorithm::Rho, &vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            ],
        ];
        let to_be_tested = c.to_teebench_cmd();

        assert_eq!(to_be_tested.len(), cmds.len());
        for tested_cmds in to_be_tested {
            let expected_cmds = if tested_cmds[0].args[5] == "cache-exceed" {
                &cmds[0]
            } else {
                &cmds[1]
            };
            for tested_cmd in tested_cmds {
                println!("tested_cmd: {tested_cmd:#?}");
                let mut found = false;
                for expected_cmd in expected_cmds {
                    if &tested_cmd == expected_cmd {
                        println!("equal: {expected_cmd}");
                        found = true;
                        break;
                    } else {
                        println!("not equal: {expected_cmd}");
                    }
                }
                if !found {
                    panic!();
                }
                // println!("expected_cmds: {expected_cmds:#?}");
                // let contained = expected_cmds.contains(&tested_cmd);
                // assert!(contained);
            }
        }
    }
}
