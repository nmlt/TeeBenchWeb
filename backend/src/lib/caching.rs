use anyhow::Result;
use indoc::indoc;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};
use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use common::data_types::TeebenchArgs;
const SQLITE_FILE_VAR_NAME: &str = "TEEBENCHWEB_SQLITE_FILE";

lazy_static::lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![M::up(indoc!(r#"
        CREATE TABLE teebenchargs(
            id INTEGER PRIMARY KEY,
            app_name TEXT NOT NULL,
            dataset TEXT NOT NULL,
            algorithm TEXT NOT NULL,
            threads INTEGER NOT NULL,
            selectivity INTEGER NOT NULL,
            data_skew TEXT NOT NULL,
            seal_chunk_size INTEGER NOT NULL,
            r_tuples INTEGER NOT NULL,
            s_tuples INTEGER NOT NULL,
            r_path TEXT,
            s_path TEXT,
            r_size INTEGER,
            s_size INTEGER,
            seal INTEGER NOT NULL,
            sort_r INTEGER NOT NULL,
            sort_s INTEGER NOT NULL
        );
        CREATE TABLE output(
            teebenchargs_id INTEGER PRIMARY KEY,
            algorithm TEXT NOT NULL,
            threads INTEGER NOT NULL,
            relR INTEGER NOT NULL,
            relS INTEGER NOT NULL,
            matches INTEGER NOT NULL,
            phase1Cycles INTEGER NOT NULL,
            phase2Cycles INTEGER NOT NULL,
            cyclesPerTuple INTEGER NOT NULL,
            phase1Time INTEGER NOT NULL,
            phase2Time INTEGER NOT NULL,
            totalTime INTEGER NOT NULL,
            throughput REAL NOT NULL,
            phase1L3CacheMisses INTEGER NOT NULL,
            phase1L3HitRatio REAL NOT NULL,
            phase1L2CacheMisses INTEGER NOT NULL,
            phase1L2HitRatio REAL NOT NULL,
            phase1IPC REAL NOT NULL,
            phase1IR INTEGER NOT NULL,
            phase1EWB INTEGER NOT NULL,
            phase1VoluntaryCS INTEGER NOT NULL,
            phase1InvoluntaryCS INTEGER NOT NULL,
            phase1UserCpuTime INTEGER NOT NULL,
            phase1SystemCpuTime INTEGER NOT NULL,
            phase2L3CacheMisses INTEGER NOT NULL,
            phase2L3HitRatio REAL NOT NULL,
            phase2L2CacheMisses INTEGER NOT NULL,
            phase2L2HitRatio REAL NOT NULL,
            phase2IPC REAL NOT NULL,
            phase2IR INTEGER NOT NULL,
            phase2EWB INTEGER NOT NULL,
            phase2VoluntaryCS INTEGER NOT NULL,
            phase2InvoluntaryCS INTEGER NOT NULL,
            phase2UserCpuTime INTEGER NOT NULL,
            phase2SystemCpuTime INTEGER NOT NULL,
            totalL3CacheMisses INTEGER NOT NULL,
            totalL3HitRatio REAL NOT NULL,
            totalL2CacheMisses INTEGER NOT NULL,
            totalL2HitRatio REAL NOT NULL,
            totalIPC REAL NOT NULL,
            totalIR INTEGER NOT NULL,
            totalEWB INTEGER NOT NULL,
            totalVoluntaryCS INTEGER NOT NULL,
            totalInvoluntaryCS INTEGER NOT NULL,
            totalUserCpuTime INTEGER NOT NULL,
            totalSystemCpuTime INTEGER NOT NULL
        );
    "#))]);
}

pub fn setup_sqlite() -> Result<Connection> {
    let sqlite_dir = PathBuf::from(
        var(SQLITE_FILE_VAR_NAME).unwrap_or_else(|_| panic!("{SQLITE_FILE_VAR_NAME} not set")),
    );
    let mut conn = Connection::open(sqlite_dir)?;
    MIGRATIONS.to_latest(&mut conn)?;
    Ok(conn)
}

pub fn insert_experiment(
    conn: Arc<Mutex<Connection>>,
    args: TeebenchArgs,
    data: HashMap<String, String>,
) -> Result<()> {
    let conn = conn.lock().unwrap();
    conn.execute(indoc!("INSERT INTO teebenchargs (app_name, dataset, algorithm, threads, selectivity, data_skew, seal_chunk_size, r_tuples, s_tuples, r_path, s_path, r_size, s_size, seal, sort_r, sort_s) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)"), (&args.app_name.to_string(), &args.dataset.to_string(), &format!("{:#?}", args.algorithm), &args.threads, &args.selectivity, &args.data_skew, &args.seal_chunk_size, &args.r_tuples, &args.s_tuples, &args.r_path, &args.s_path, &args.r_size, &args.s_size, &args.seal, &args.sort_r, &args.sort_s))?;
    let id = conn.last_insert_rowid();
    conn.execute(indoc!("INSERT INTO output (teebenchargs_id, algorithm, threads, relR, relS, matches, phase1Cycles, phase2Cycles, cyclesPerTuple, phase1Time, phase2Time, totalTime, throughput, phase1L3CacheMisses, phase1L3HitRatio, phase1L2CacheMisses, phase1L2HitRatio, phase1IPC, phase1IR, phase1EWB, phase1VoluntaryCS, phase1InvoluntaryCS, phase1UserCpuTime, phase1SystemCpuTime, phase2L3CacheMisses, phase2L3HitRatio, phase2L2CacheMisses, phase2L2HitRatio, phase2IPC, phase2IR, phase2EWB, phase2VoluntaryCS, phase2InvoluntaryCS, phase2UserCpuTime, phase2SystemCpuTime, totalL3CacheMisses, totalL3HitRatio, totalL2CacheMisses, totalL2HitRatio, totalIPC, totalIR, totalEWB, totalVoluntaryCS, totalInvoluntaryCS, totalUserCpuTime, totalSystemCpuTime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45, ?46)"), params![
        id,
        data["algorithm"],
        data["threads"],
        data["relR"],
        data["relS"],
        data["matches"],
        data["phase1Cycles"],
        data["phase2Cycles"],
        data["cyclesPerTuple"],
        data["phase1Time"],
        data["phase2Time"],
        data["totalTime"],
        data["throughput"],
        data["phase1L3CacheMisses"],
        data["phase1L3HitRatio"],
        data["phase1L2CacheMisses"],
        data["phase1L2HitRatio"],
        data["phase1IPC"],
        data["phase1IR"],
        data["phase1EWB"],
        data["phase1VoluntaryCS"],
        data["phase1InvoluntaryCS"],
        data["phase1UserCpuTime"],
        data["phase1SystemCpuTime"],
        data["phase2L3CacheMisses"],
        data["phase2L3HitRatio"],
        data["phase2L2CacheMisses"],
        data["phase2L2HitRatio"],
        data["phase2IPC"],
        data["phase2IR"],
        data["phase2EWB"],
        data["phase2VoluntaryCS"],
        data["phase2InvoluntaryCS"],
        data["phase2UserCpuTime"],
        data["phase2SystemCpuTime"],
        data["totalL3CacheMisses"],
        data["totalL3HitRatio"],
        data["totalL2CacheMisses"],
        data["totalL2HitRatio"],
        data["totalIPC"],
        data["totalIR"],
        data["totalEWB"],
        data["totalVoluntaryCS"],
        data["totalInvoluntaryCS"],
        data["totalUserCpuTime"],
        data["totalSystemCpuTime"],])?;
    Ok(())
}

pub fn search_for_exp(
    conn: Arc<Mutex<Connection>>,
    args: &TeebenchArgs,
) -> Result<HashMap<String, String>> {
    let conn = conn.lock().unwrap();
    let id = conn.query_row(indoc!("SELECT id FROM teebenchargs WHERE app_name=(?1) AND dataset=(?2) AND algorithm=(?3) AND threads=(?4) AND selectivity=(?5) AND data_skew=(?6) AND seal_chunk_size=(?7) AND r_tuples=(?8) AND s_tuples=(?9) AND r_path=(?10) AND s_path=(?11) AND r_size=(?12) AND s_size=(?13) AND seal=(?14) AND sort_r=(?15) AND sort_s=(?16)"), (&args.app_name.to_string(), &args.dataset.to_string(), &format!("{:#?}", args.algorithm), &args.threads, &args.selectivity, &args.data_skew, &args.seal_chunk_size, &args.r_tuples, &args.s_tuples, &args.r_path, &args.s_path, &args.r_size, &args.s_size, &args.seal, &args.sort_r, &args.sort_s), |r| r.get::<usize, usize>(0))?;
    let mut map: HashMap<String, String> = HashMap::new();
    conn.query_row(indoc!("SELECT algorithm, threads, relR, relS, matches, phase1Cycles, phase2Cycles, cyclesPerTuple, phase1Time, phase2Time, totalTime, throughput, phase1L3CacheMisses, phase1L3HitRatio, phase1L2CacheMisses, phase1L2HitRatio, phase1IPC, phase1IR, phase1EWB, phase1VoluntaryCS, phase1InvoluntaryCS, phase1UserCpuTime, phase1SystemCpuTime, phase2L3CacheMisses, phase2L3HitRatio, phase2L2CacheMisses, phase2L2HitRatio, phase2IPC, phase2IR, phase2EWB, phase2VoluntaryCS, phase2InvoluntaryCS, phase2UserCpuTime, phase2SystemCpuTime, totalL3CacheMisses, totalL3HitRatio, totalL2CacheMisses, totalL2HitRatio, totalIPC, totalIR, totalEWB, totalVoluntaryCS, totalInvoluntaryCS, totalUserCpuTime, totalSystemCpuTime FROM output WHERE teebenchargs_id=(?1)"), [id], |r| {
        map.insert("algorithm".to_owned(), r.get::<usize, String>(0)?);
        map.insert("threads".to_owned(), r.get::<usize, String>(1)?);
        map.insert("relR".to_owned(), r.get::<usize, String>(2)?);
        map.insert("relS".to_owned(), r.get::<usize, String>(3)?);
        map.insert("matches".to_owned(), r.get::<usize, String>(4)?);
        map.insert("phase1Cycles".to_owned(), r.get::<usize, String>(5)?);
        map.insert("phase2Cycles".to_owned(), r.get::<usize, String>(6)?);
        map.insert("cyclesPerTuple".to_owned(), r.get::<usize, String>(7)?);
        map.insert("phase1Time".to_owned(), r.get::<usize, String>(8)?);
        map.insert("phase2Time".to_owned(), r.get::<usize, String>(9)?);
        map.insert("totalTime".to_owned(), r.get::<usize, String>(10)?);
        map.insert("throughput".to_owned(), r.get::<usize, String>(11)?);
        map.insert("phase1L3CacheMisses".to_owned(), r.get::<usize, String>(12)?);
        map.insert("phase1L3HitRatio".to_owned(), r.get::<usize, String>(13)?);
        map.insert("phase1L2CacheMisses".to_owned(), r.get::<usize, String>(14)?);
        map.insert("phase1L2HitRatio".to_owned(), r.get::<usize, String>(15)?);
        map.insert("phase1IPC".to_owned(), r.get::<usize, String>(16)?);
        map.insert("phase1IR".to_owned(), r.get::<usize, String>(17)?);
        map.insert("phase1EWB".to_owned(), r.get::<usize, String>(18)?);
        map.insert("phase1VoluntaryCS".to_owned(), r.get::<usize, String>(19)?);
        map.insert("phase1InvoluntaryCS".to_owned(), r.get::<usize, String>(20)?);
        map.insert("phase1UserCpuTime".to_owned(), r.get::<usize, String>(21)?);
        map.insert("phase1SystemCpuTime".to_owned(), r.get::<usize, String>(22)?);
        map.insert("phase2L3CacheMisses".to_owned(), r.get::<usize, String>(23)?);
        map.insert("phase2L3HitRatio".to_owned(), r.get::<usize, String>(24)?);
        map.insert("phase2L2CacheMisses".to_owned(), r.get::<usize, String>(25)?);
        map.insert("phase2L2HitRatio".to_owned(), r.get::<usize, String>(26)?);
        map.insert("phase2IPC".to_owned(), r.get::<usize, String>(27)?);
        map.insert("phase2IR".to_owned(), r.get::<usize, String>(28)?);
        map.insert("phase2EWB".to_owned(), r.get::<usize, String>(29)?);
        map.insert("phase2VoluntaryCS".to_owned(), r.get::<usize, String>(30)?);
        map.insert("phase2InvoluntaryCS".to_owned(), r.get::<usize, String>(31)?);
        map.insert("phase2UserCpuTime".to_owned(), r.get::<usize, String>(32)?);
        map.insert("phase2SystemCpuTime".to_owned(), r.get::<usize, String>(33)?);
        map.insert("totalL3CacheMisses".to_owned(), r.get::<usize, String>(34)?);
        map.insert("totalL3HitRatio".to_owned(), r.get::<usize, String>(35)?);
        map.insert("totalL2CacheMisses".to_owned(), r.get::<usize, String>(36)?);
        map.insert("totalL2HitRatio".to_owned(), r.get::<usize, String>(37)?);
        map.insert("totalIPC".to_owned(), r.get::<usize, String>(38)?);
        map.insert("totalIR".to_owned(), r.get::<usize, String>(39)?);
        map.insert("totalEWB".to_owned(), r.get::<usize, String>(40)?);
        map.insert("totalVoluntaryCS".to_owned(), r.get::<usize, String>(41)?);
        map.insert("totalInvoluntaryCS".to_owned(), r.get::<usize, String>(42)?);
        map.insert("totalUserCpuTime".to_owned(), r.get::<usize, String>(43)?);
        map.insert("totalSystemCpuTime".to_owned(), r.get::<usize, String>(44)?);
        Ok(())
    })?;
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use serial_test::serial;

    fn setup_environment() -> std::path::PathBuf {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("TeebenchWeb");
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temp_dir!");
        temp_dir.push("cache.sqlite");
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&temp_dir)
            .expect("Failed to touch 'cache.sqlite'!");
        std::env::set_var(SQLITE_FILE_VAR_NAME, format!("{temp_dir:?}"));
        temp_dir
    }

    fn cleanup_environment(path: PathBuf) -> Result<()> {
        std::fs::remove_file(&path)?;
        std::env::set_var(SQLITE_FILE_VAR_NAME, "");
        Ok(())
    }

    fn test_setup_sqlite(path: &PathBuf) -> Result<()> {
        let conn = setup_sqlite()?;
        assert_eq!(path.to_str(), conn.path());
        let app_name = "Sgx";
        conn.execute(
            "INSERT INTO teebenchargs (app_name) VALUES (?1)",
            [app_name],
        )?;
        let id = conn.query_row("SELECT id, app_name FROM teebenchargs", [], |r| {
            let id = r.get::<usize, usize>(0)?;
            let queried_app_name = r.get::<usize, String>(1)?;
            assert_eq!(app_name, queried_app_name);
            Ok(id)
        })?;
        assert_eq!(id, conn.last_insert_rowid() as usize);
        conn.close().map_err(|(_c, e)| e)?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_setup_sqlite_wrapper() -> Result<()> {
        let path = setup_environment();
        match test_setup_sqlite(&path) {
            Ok(_) => {
                cleanup_environment(path)?;
                Ok(())
            }
            Err(e) => {
                cleanup_environment(path)?;
                bail!(e);
            }
        }
    }
}
