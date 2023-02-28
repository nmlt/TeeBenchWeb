//! Fake teebench script that outputs teebench output if the right arguments are given.
//! Used for testing.
//!
use indoc::indoc;
use structopt::StructOpt;
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;


use common::data_types::{Algorithm, Dataset, Platform};

#[derive(Debug, StructOpt, PartialEq, Eq, Default)]
// #[allow(dead_code)]
#[structopt(
    name = "teebench",
    about = "fake placeholder for testing that outputs teebench output. Because I don't have SGX on my dev machine."
)]
struct TeebenchArgs {
    /// The name of the application. Used to determine whether it is simulating Sgx or native.
    #[structopt(skip = std::env::args().next().unwrap())]
    app_name: PathBuf,
    ///`-a` - join algorithm name. Currently working: `CHT`, `PHT`, `PSM`, `RHO`, `RHT`, `RSM`. Default: `RHO`
    #[structopt(short = "a", long, default_value = "RHO")]
    algorithm: Algorithm,
    ///`-c` - seal chunk size in kBs. if set to 0 then seal everything at once. Default: `0`
    #[structopt(short = "c", long, default_value = "0")]
    seal_chunk_size: u32,
    ///`-d` - name of pre-defined dataset. Currently working: `cache-fit`, `cache-exceed`, `L`. Default: `cache-fit`
    #[structopt(short = "d", long, parse(try_from_str = Dataset::from_cmd_arg), default_value = "CacheFit")]
    //#[structopt(skip = Dataset::CacheFit)]
    dataset: Dataset,
    ///`-l` - join selectivity. Should be a number between 0 and 100. Default: `100`
    #[structopt(short = "l", long, default_value = "100")]
    selectivity: u8,
    ///`-n` - number of threads used to execute the join algorithm. Default: `2`
    #[structopt(short = "n", long, default_value = "2")]
    threads: u8,
    ///`-r` - number of tuples of R relation. Default: `2097152`
    #[structopt(short = "r", long, default_value = "2097152")]
    r_tuples: u32,
    ///`-s` - number of tuples of S relation. Default: `2097152`
    #[structopt(short = "s", long, default_value = "2097152")]
    s_tuples: u32,
    ///`-t | --r-path` - filepath to build R relation. Default: `none`
    #[structopt(short = "t", long)]
    r_path: Option<String>,
    ///`-u | --s-path` - filepath to build S relation. Default `none`
    #[structopt(short = "u", long)]
    s_path: Option<String>,
    ///`-x` - size of R in MBs. Default: `none`
    #[structopt(short = "x", long)]
    r_size: Option<u32>,
    ///`-y` - size of S in MBs. Default: `none`
    #[structopt(short = "y", long)]
    s_size: Option<u32>,
    ///`-z` - data skew. Default: `0`
    #[structopt(short = "z", long, default_value = "0")]
    data_skew: u32,
    ///`--seal` - flag to seal join input data. Default: `false`
    #[structopt(long = "seal")]
    seal: bool,
    ///`--sort-r` - flag to pre-sort relation R. Default: `false`
    #[structopt(long = "sort-r")]
    sort_r: bool,
    ///`--sort-s` - flag to pre-sort relation S. Default: `false`
    #[structopt(long = "sort-s")]
    sort_s: bool,
    ///Change output to only print out data in csv format
    #[structopt(long)]
    csv: bool,
}

fn main() -> Result<()> {
    let opt = TeebenchArgs::from_args();
    //println!("{opt:?}");
    if !opt.csv {
        return Err(anyhow!("Only CSV output supported"));
    }
    let platform = match opt.app_name.file_name() {
        None => unreachable!(), // The application has to have a name, right?
        Some(name) => match name.to_str().unwrap() {
            "fake_teebench" | "sgx" => Platform::Sgx,
            "native" => Platform::Native,
            name => {
                return Err(anyhow!("Unknown app name: {name:?}"));
            },
        },
    };
    if let Some(output) = CSV_OUTPUT.get(&(platform, opt.algorithm.clone(), opt.dataset.clone())) {
        print!("{output}");
    } else {
        return Err(anyhow!("Could not find the combination of platform, algorithm and dataset: {opt:?}"));
    }
    Ok(())
}

static CSV_OUTPUT: Lazy<HashMap<(Platform, Algorithm, Dataset), &str>> = Lazy::new(|| {
    HashMap::from([
        ((Platform::Sgx, Algorithm::Rho, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            5242880,0,530802580,0,85,0,0,164574,39.8216
        "}),
        ((Platform::Sgx, Algorithm::Rho, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            52428800,0,8161090876,0,127,0,0,2461427,26.6252
        "}),
        ((Platform::Sgx, Algorithm::Crkj, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            5242880,0,0,0,166,13520,306410,319930,20.4845
        "}),
        ((Platform::Sgx, Algorithm::Crkj, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            52428800,0,0,0,123,198162,2170082,2368244,27.6728
        "}),
        ((Platform::Sgx, Algorithm::Pht, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Pht, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Psm, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Psm, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Mway, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Mway, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Rht, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Rht, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Cht, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Cht, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Rsm, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Rsm, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Inl, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Inl, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Nlj, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Sgx, Algorithm::Nlj, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        // native:
        ((Platform::Native, Algorithm::Rho, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Rho, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Crkj, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Crkj, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Pht, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Pht, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Psm, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Psm, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Mway, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Mway, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Rht, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Rht, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Cht, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Cht, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Rsm, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Rsm, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Inl, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Inl, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Nlj, Dataset::CacheFit), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
        ((Platform::Native, Algorithm::Nlj, Dataset::CacheExceed), indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,0,0
        "}),
    ])
});

// const csv_output = HashMap::from([
//     (Algorithm::Rho, indoc! {"

//     "}),
//     (Algorithm::Rho, indoc! {"

//     "}),
//     (Algorithm::Crkj, indoc! {"

//     "}),
//     (Algorithm::Crkj, indoc! {"

//     "}),
// ]);

#[allow(unused)]
static NORMAL_OUTPUT: [&str; 4] = [
    // RHO cache-exceed
    indoc! {"
        [  0.0000][DEBUG] Not checking the validity of algorithm when parsing - will check in the enclave.
        [  0.0001][DEBUG] Number of threads = 2 (N/A for every algorithm)
        [  0.0001][ INFO] Build relation R with size = 100.00 MB (13107200 tuples)
        [  0.3608][DEBUG] DONE
        [  0.3608][ INFO] Build relation S with size = 400.00 MB (52428800 tuples)
        [  1.7944][DEBUG] DONE
        [  3.9526][ INFO] Enclave id = 2
        [  3.9526][ INFO] Running algorithm RHO
        [  3.9633][ ENCL] NUM_PASSES=2, RADIX_BITS=14
        [  3.9633][ ENCL] fanOut = 128, R = 7, D = 7, thresh1 = 1048576
        [  3.9634][ ENCL] Radix partitioning. R=0, D=7, fanout=128, MASK=127
        [  4.2104][ ENCL] Radix partitioning. R=0, D=7, fanout=128, MASK=127
        [  5.2441][ ENCL] Pass-2: # partitioning tasks = 128
        [  6.3512][ ENCL] Number of join tasks = 16384
        [  6.3995][ ENCL] Total input tuples : 65536000
        [  6.3996][ ENCL] Result tuples : 52428800
        [  6.3996][ ENCL] Phase Total (cycles) : 8301097732
        [  6.3996][ ENCL] Phase Partition (cycles) : 8137498668
        [  6.3996][ ENCL] Phase Join (cycles) : 163583198
        [  6.3996][ ENCL] Cycles-per-tuple           : 126.6647
        [  6.3996][ ENCL] Cycles-per-tuple-partition : 124.1684
        [  6.3996][ ENCL] Cycles-per-tuple-pass1     : 66.6013
        [  6.3996][ ENCL] Cycles-per-tuple-pass2     : 57.5666
        [  6.3996][ ENCL] Cycles-per-tuple-join      : 2.4961
        [  6.3996][ ENCL] Total Runtime (us) : 2435780
        [  6.3997][ ENCL] Throughput (M rec/sec) : 26.91
        [  6.3997][DEBUG] ************************** RUSAGE **************************
        [  6.3997][DEBUG] user CPU time used               : 2.448417s
        [  6.3997][DEBUG] system CPU time used             : 3.-579760s
        [  6.3997][DEBUG] page reclaims (soft page faults) : 272025
        [  6.3997][DEBUG] page faults (hard page faults)   : 0
        [  6.3997][DEBUG] voluntary context switches       : 46
        [  6.3998][DEBUG] involuntary context switches     : 14
        [  6.3998][DEBUG] ************************** RUSAGE **************************
        [  6.3998][ INFO] Total join runtime: 2.45s
        [  6.3998][ INFO] throughput = 26.78 [M rec / s]
    "},
    // RHO cache-fit
    indoc! {"
        [  0.0000][DEBUG] Not checking the validity of algorithm when parsing - will check in the enclave.
        [  0.0001][DEBUG] Number of threads = 2 (N/A for every algorithm)
        [  0.0001][ INFO] Build relation R with size = 10.00 MB (1310720 tuples)
        [  0.0146][DEBUG] DONE
        [  0.0146][ INFO] Build relation S with size = 40.00 MB (5242880 tuples)
        [  0.0685][DEBUG] DONE
        [  2.2268][ INFO] Enclave id = 2
        [  2.2268][ INFO] Running algorithm RHO
        [  2.2374][ ENCL] NUM_PASSES=2, RADIX_BITS=14
        [  2.2374][ ENCL] fanOut = 128, R = 7, D = 7, thresh1 = 1048576
        [  2.2376][ ENCL] Radix partitioning. R=0, D=7, fanout=128, MASK=127
        [  2.2620][ ENCL] Radix partitioning. R=0, D=7, fanout=128, MASK=127
        [  2.3653][ ENCL] Pass-2: # partitioning tasks = 128
        [  2.3944][ ENCL] Number of join tasks = 16383
        [  2.4019][ ENCL] Total input tuples : 6553600
        [  2.4019][ ENCL] Result tuples : 5242880
        [  2.4019][ ENCL] Phase Total (cycles) : 559931732
        [  2.4020][ ENCL] Phase Partition (cycles) : 534467998
        [  2.4020][ ENCL] Phase Join (cycles) : 25447854
        [  2.4020][ ENCL] Cycles-per-tuple           : 85.4388
        [  2.4020][ ENCL] Cycles-per-tuple-partition : 81.5533
        [  2.4020][ ENCL] Cycles-per-tuple-pass1     : 66.4303
        [  2.4020][ ENCL] Cycles-per-tuple-pass2     : 15.1194
        [  2.4020][ ENCL] Cycles-per-tuple-join      : 3.8830
        [  2.4020][ ENCL] Total Runtime (us) : 164305
        [  2.4021][ ENCL] Throughput (M rec/sec) : 39.89
        [  2.4021][DEBUG] ************************** RUSAGE **************************
        [  2.4021][DEBUG] user CPU time used               : 0.172768s
        [  2.4021][DEBUG] system CPU time used             : 0.153839s
        [  2.4021][DEBUG] page reclaims (soft page faults) : 15253
        [  2.4021][DEBUG] page faults (hard page faults)   : 0
        [  2.4021][DEBUG] voluntary context switches       : 374
        [  2.4021][DEBUG] involuntary context switches     : 0
        [  2.4021][DEBUG] ************************** RUSAGE **************************
        [  2.4021][ INFO] Total join runtime: 0.18s
        [  2.4021][ INFO] throughput = 37.39 [M rec / s]  
    "},
    // PSM cache-exceed
    indoc! {"
        [  0.0000][DEBUG] Not checking the validity of algorithm when parsing - will check in the enclave.
        [  0.0001][DEBUG] Number of threads = 2 (N/A for every algorithm)
        [  0.0001][ INFO] Build relation R with size = 100.00 MB (13107200 tuples)
        [  0.3556][DEBUG] DONE
        [  0.3556][ INFO] Build relation S with size = 400.00 MB (52428800 tuples)
        [  1.7859][DEBUG] DONE
        [  3.9498][ INFO] Enclave id = 2
        [  3.9498][ INFO] Running algorithm PSM
        [  5.4080][ ENCL] R sorted
        [  8.6402][ ENCL] S sorted
        [  8.6402][ ENCL] Merge1
        [  8.6841][ ENCL] Merge2
        [  8.6842][ ENCL] Total input tuples : 65536000
        [  8.6842][ ENCL] Result tuples : 52428800
        [  8.6842][ ENCL] Phase Total (cycles) : 16100742442
        [  8.6842][ ENCL] Phase Partition (cycles) : 15951099664
        [  8.6842][ ENCL] Phase Join (cycles) : 149618736
        [  8.6842][ ENCL] Cycles-per-tuple           : 245.6778
        [  8.6842][ ENCL] Cycles-per-tuple-partition : 243.3945
        [  8.6842][ ENCL] Cycles-per-tuple-pass1     : 0.0000
        [  8.6842][ ENCL] Cycles-per-tuple-pass2     : 0.0000
        [  8.6843][ ENCL] Cycles-per-tuple-join      : 2.2830
        [  8.6843][ ENCL] Total Runtime (us) : 4724415
        [  8.6843][ ENCL] Throughput (M rec/sec) : 13.87
        [  8.6843][DEBUG] ************************** RUSAGE **************************
        [  8.6843][DEBUG] user CPU time used               : 9.311705s
        [  8.6843][DEBUG] system CPU time used             : 0.3825s
        [  8.6843][DEBUG] page reclaims (soft page faults) : 152
        [  8.6843][DEBUG] page faults (hard page faults)   : 0
        [  8.6843][DEBUG] voluntary context switches       : 6
        [  8.6843][DEBUG] involuntary context switches     : 21
        [  8.6843][DEBUG] ************************** RUSAGE **************************
        [  8.6843][ INFO] Total join runtime: 4.73s
        [  8.6843][ INFO] throughput = 13.84 [M rec / s]
    "},
    // PSM cache-fit
    indoc! {"
        [  0.0000][DEBUG] Not checking the validity of algorithm when parsing - will check in the enclave.
        [  0.0001][DEBUG] Number of threads = 2 (N/A for every algorithm)
        [  0.0001][ INFO] Build relation R with size = 10.00 MB (1310720 tuples)
        [  0.0144][DEBUG] DONE
        [  0.0144][ INFO] Build relation S with size = 40.00 MB (5242880 tuples)
        [  0.0690][DEBUG] DONE
        [  2.2331][ INFO] Enclave id = 2
        [  2.2331][ INFO] Running algorithm PSM
        [  2.3970][ ENCL] R sorted
        [  2.7040][ ENCL] S sorted
        [  2.7040][ ENCL] Merge1
        [  2.7083][ ENCL] Merge2
        [  2.7084][ ENCL] Total input tuples : 6553600
        [  2.7084][ ENCL] Result tuples : 5242880
        [  2.7084][ ENCL] Phase Total (cycles) : 1585966278
        [  2.7084][ ENCL] Phase Partition (cycles) : 1571296672
        [  2.7084][ ENCL] Phase Join (cycles) : 14646154
        [  2.7084][ ENCL] Cycles-per-tuple           : 241.9992
        [  2.7085][ ENCL] Cycles-per-tuple-partition : 239.7608
        [  2.7085][ ENCL] Cycles-per-tuple-pass1     : 0.0000
        [  2.7085][ ENCL] Cycles-per-tuple-pass2     : 0.0000
        [  2.7085][ ENCL] Cycles-per-tuple-join      : 2.2348
        [  2.7085][ ENCL] Total Runtime (us) : 465371
        [  2.7085][ ENCL] Throughput (M rec/sec) : 14.08
        [  2.7085][DEBUG] ************************** RUSAGE **************************
        [  2.7085][DEBUG] user CPU time used               : 1.-95829s
        [  2.7085][DEBUG] system CPU time used             : 0.1745s
        [  2.7086][DEBUG] page reclaims (soft page faults) : 146
        [  2.7086][DEBUG] page faults (hard page faults)   : 0
        [  2.7086][DEBUG] voluntary context switches       : 6
        [  2.7086][DEBUG] involuntary context switches     : 2
        [  2.7086][DEBUG] ************************** RUSAGE **************************
        [  2.7086][ INFO] Total join runtime: 0.48s
        [  2.7086][ INFO] throughput = 13.78 [M rec / s]
    "},
];