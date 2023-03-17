//! Fake teebench script that outputs teebench output if the right arguments are given.
//! Used for testing.
//!
use anyhow::{anyhow, Result};
use indoc::indoc;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

use common::data_types::{Algorithm, Dataset, Platform};

use common::data_types::TeebenchArgs;

fn main() -> Result<()> {
    let opt = TeebenchArgs::from_args();
    //println!("{opt:?}");
    if !opt.csv {
        return Err(anyhow!("Only CSV output supported"));
    }
    let platform = opt.app_name;
    if let Some(output) = CSV_OUTPUT.get(&(platform, opt.algorithm.clone(), opt.dataset.clone())) {
        let mut rdr = csv::Reader::from_reader(output.as_bytes());
        let mut iter = rdr.records();
        // iter.next(); // First line is skipped anyway because a header is expected.
        let data_record = iter.next().unwrap()?;
        let time_total_usec: u64 = data_record.get(7).unwrap().parse()?;
        sleep(Duration::from_micros(time_total_usec));
        print!("{output}");
    } else {
        return Err(anyhow!(
            "Could not find the combination of platform, algorithm and dataset: {opt:?}"
        ));
    }
    Ok(())
}

static CSV_OUTPUT: Lazy<HashMap<(Platform, Algorithm, Dataset), &str>> = Lazy::new(|| {
    HashMap::from([
        (
            (Platform::Sgx, Algorithm::Rho, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHO,2,1310720,5242880,5242880,0,481746914,0,78,0,0,257808,25.4205
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Rho, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHO,2,13107200,52428800,52428800,0,5930547977,0,92,0,0,3047240,21.5067
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Crkj, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CrkJoin,2,1310720,5242880,5242880,0,0,0,150,21165,475384,496549,13.1983
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Crkj, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CrkJoin,2,13107200,52428800,52428800,0,0,0,133,340694,4056896,4397590,14.9027
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Pht, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PHT,2,1310720,5242880,5242880,298166084,0,486162275,119,0,0,393755,16.6439
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Pht, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput            0,0,0,0,0,0,0,5000000,0
            PHT,2,13107200,52428800,52428800,219376788914,0,723447139495,14386,0,0,473306934,0.1385
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Psm, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PSM,2,1310720,5242880,5242880,1176578271,9193003,0,180,0,0,595286,11.0092
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Psm, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PSM,2,13107200,52428800,52428800,11814713271,93232254,0,181,0,0,5977924,10.9630
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Mway, Dataset::CacheFit),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,1000000,0
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Mway, Dataset::CacheExceed),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,5000000,0
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Rht, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHT,2,1310720,5242880,5242880,0,381947479,0,61,0,0,203048,32.2761
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Rht, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHT,2,13107200,52428800,52428800,0,5906061283,0,92,0,0,3035341,21.5910
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Cht, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CHT,2,1310720,5242880,5242880,34228389,61312999,307212842,61,0,0,202202,32.4112
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Cht, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CHT,2,13107200,52428800,52428800,2667835856,656401775,153445200425,2392,0,0,78699826,0.8327
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Rsm, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RSM,2,1310720,5242880,5242880,0,420542556,0,113,0,0,371827,17.6254
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Rsm, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RSM,2,13107200,52428800,52428800,0,6933755145,0,178,0,0,5869445,11.1656
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Inl, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            INL,2,1310720,5242880,5242880,0,0,0,124,0,0,409273,16.0128
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Inl, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            INL,2,13107200,52428800,0,0,0,0,0,0,0,0,0.00
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Nlj, Dataset::CacheFit),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,1000000,0
        "},
        ),
        (
            (Platform::Sgx, Algorithm::Nlj, Dataset::CacheExceed),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,5000000,0
        "},
        ),
        // native:
        (
            (Platform::Native, Algorithm::Rho, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHO,2,1310720,5242880,5242880,0,67402791,0,13,0,0,43788,149.6666
        "},
        ),
        (
            (Platform::Native, Algorithm::Rho, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHO,2,13107200,52428800,52428800,0,739501213,0,13,0,0,445093,147.2411
        "},
        ),
        (
            (Platform::Native, Algorithm::Crkj, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CrkJoin,2,1310720,5242880,5242880,0,0,0,109,19433,339738,359171,18.2465
        "},
        ),
        (
            (Platform::Native, Algorithm::Crkj, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CrkJoin,2,13107200,52428800,52428800,0,0,0,95,250358,2885621,3135979,20.8981
        "},
        ),
        (
            (Platform::Native, Algorithm::Pht, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PHT,2,1310720,5242880,5242880,116549274,0,118558692,35,0,0,118027,55.5263
        "},
        ),
        (
            (Platform::Native, Algorithm::Pht, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PHT,2,13107200,52428800,52428800,1398107221,0,1595668763,45,0,0,1502906,43.6062
        "},
        ),
        (
            (Platform::Native, Algorithm::Psm, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PSM,2,1310720,5242880,5242880,1206610407,10233038,0,185,0,0,610868,10.7283
        "},
        ),
        (
            (Platform::Native, Algorithm::Psm, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            PSM,2,13107200,52428800,52428800,15422516805,101285232,0,236,0,0,7793105,8.4095
        "},
        ),
        (
            (Platform::Native, Algorithm::Mway, Dataset::CacheFit),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,1000000,0
        "},
        ),
        (
            (Platform::Native, Algorithm::Mway, Dataset::CacheExceed),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,5000000,0
        "},
        ),
        (
            (Platform::Native, Algorithm::Rht, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHT,2,1310720,5242880,5242880,0,278009794,0,56,0,0,186142,35.2075
        "},
        ),
        (
            (Platform::Native, Algorithm::Rht, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RHT,2,13107200,52428800,52428800,0,783459653,0,14,0,0,470854,139.1854
        "},
        ),
        (
            (Platform::Native, Algorithm::Cht, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CHT,2,1310720,5242880,5242880,14717179,9502994,107315399,20,0,0,66033,99.2473
        "},
        ),
        (
            (Platform::Native, Algorithm::Cht, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            CHT,2,13107200,52428800,52428800,173920738,77403114,1947970581,33,0,0,1104069,59.3586
        "},
        ),
        (
            (Platform::Native, Algorithm::Rsm, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RSM,2,1310720,5242880,5242880,0,67389227,0,53,0,0,176443,37.1429
        "},
        ),
        (
            (Platform::Native, Algorithm::Rsm, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            RSM,2,13107200,52428800,52428800,0,732636786,0,95,0,0,3126966,20.9583
        "},
        ),
        (
            (Platform::Native, Algorithm::Inl, Dataset::CacheFit),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            INL,2,1310720,5242880,5242880,0,0,0,77,0,0,255321,25.6681
        "},
        ),
        (
            (Platform::Native, Algorithm::Inl, Dataset::CacheExceed),
            indoc! {"
            algorithm,threads,relR,relS,matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            INL,2,13107200,52428800,52428800,0,0,0,149,0,0,4905579,13.3595
        "},
        ),
        (
            (Platform::Native, Algorithm::Nlj, Dataset::CacheFit),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,1000000,0
        "},
        ),
        (
            (Platform::Native, Algorithm::Nlj, Dataset::CacheExceed),
            indoc! {"
            matches,phaseBuildCycles,phasePartitionCycles,phaseProbeCycles,cyclesPerTuple,timePartitionUsec,timeJoinUsec,timeTotalUsec,throughput
            0,0,0,0,0,0,0,5000000,0
        "},
        ),
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
