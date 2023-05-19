use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};
use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::debug;

use crate::config::{
    EMPTY_CACHE_VAR_NAME, OUTPUT_CSV_PATH, SQLITE_FILE_VAR_NAME, TEEBENCHARGS_CSV_PATH,
};
use common::data_types::TeebenchArgs;

/// When SQLite imports csv, empty cells are set to "", because csv does not support NULL.
fn set_to_null_if_equals_empty_string(table: &str, column: &str) -> String {
    format!(
        r#"
        UPDATE {table}
            SET {column} = NULL
            WHERE {column} = "";
    "#
    )
}

lazy_static::lazy_static! {
    // TODO To make the TB csv output easier to change: Load a config file with the csv column names (and types?) as another static and then use it in `MIGRATIONS` to create the table.
    static ref MIGRATIONS: Migrations<'static> = {
        let v1 = r#"
            CREATE TABLE teebenchargs(
                id INTEGER PRIMARY KEY,
                app_name TEXT NOT NULL,
                dataset TEXT NOT NULL,
                algorithm TEXT NOT NULL,
                threads INTEGER NOT NULL,
                selectivity INTEGER NOT NULL,
                data_skew REAL NOT NULL,
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
                phase1L3CacheMisses INTEGER,
                phase1L3HitRatio REAL,
                phase1L2CacheMisses INTEGER,
                phase1L2HitRatio REAL,
                phase1IPC REAL,
                phase1IR INTEGER,
                phase1EWB INTEGER,
                phase1VoluntaryCS INTEGER,
                phase1InvoluntaryCS INTEGER,
                phase1UserCpuTime INTEGER,
                phase1SystemCpuTime INTEGER,
                phase2L3CacheMisses INTEGER,
                phase2L3HitRatio REAL,
                phase2L2CacheMisses INTEGER,
                phase2L2HitRatio REAL,
                phase2IPC REAL,
                phase2IR INTEGER,
                phase2EWB INTEGER,
                phase2VoluntaryCS INTEGER,
                phase2InvoluntaryCS INTEGER,
                phase2UserCpuTime INTEGER,
                phase2SystemCpuTime INTEGER,
                totalL3CacheMisses INTEGER,
                totalL3HitRatio REAL,
                totalL2CacheMisses INTEGER,
                totalL2HitRatio REAL,
                totalIPC REAL,
                totalIR INTEGER,
                totalEWB INTEGER,
                totalVoluntaryCS INTEGER,
                totalInvoluntaryCS INTEGER,
                totalUserCpuTime INTEGER,
                totalSystemCpuTime INTEGER
            );
        "#;
        let optionals_args = ["r_size", "s_size", "r_path", "s_path"];
        let optionals_args = optionals_args.map(|s| set_to_null_if_equals_empty_string("teebenchargs", s)).join("");
        let optionals_output = [
            "phase1L3CacheMisses",
            "phase1L3HitRatio",
            "phase1L2CacheMisses",
            "phase1L2HitRatio",
            "phase1IPC",
            "phase1IR",
            "phase1EWB",
            "phase1VoluntaryCS",
            "phase1InvoluntaryCS",
            "phase1UserCpuTime",
            "phase1SystemCpuTime",
            "phase2L3CacheMisses",
            "phase2L3HitRatio",
            "phase2L2CacheMisses",
            "phase2L2HitRatio",
            "phase2IPC",
            "phase2IR",
            "phase2EWB",
            "phase2VoluntaryCS",
            "phase2InvoluntaryCS",
            "phase2UserCpuTime",
            "phase2SystemCpuTime",
            "totalL3CacheMisses",
            "totalL3HitRatio",
            "totalL2CacheMisses",
            "totalL2HitRatio",
            "totalIPC",
            "totalIR",
            "totalEWB",
            "totalVoluntaryCS",
            "totalInvoluntaryCS",
            "totalUserCpuTime",
            "totalSystemCpuTime",
        ];
        let optionals_output = optionals_output.map(|s| set_to_null_if_equals_empty_string("output", s)).join("");
        fn string_to_static_str(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }
        let v2 = format!(r#"
            CREATE VIRTUAL TABLE teebenchargs_csv
                USING csv(filename={}, header=YES);
            CREATE VIRTUAL TABLE output_csv
                USING csv(filename={}, header=YES);
            INSERT INTO teebenchargs SELECT * FROM teebenchargs_csv;
            INSERT INTO output SELECT * FROM output_csv;
            {}
            {}
            DROP TABLE teebenchargs_csv;
            DROP TABLE output_csv;
        "#, TEEBENCHARGS_CSV_PATH, OUTPUT_CSV_PATH, optionals_args, optionals_output);
        // Leaking `v2` is discouraged but the only way to continue using the migrations library.
        let v2 = string_to_static_str(v2);
        let v2_down = r#"
            DROP TABLE teebenchargs;
            DROP TABLE output;
        "#;
        Migrations::new(vec![
            M::up(v1),
            M::up(v2).down(v2_down),
        ])
    };
}

pub fn setup_sqlite() -> Result<Connection> {
    let sqlite_dir = PathBuf::from(
        var(SQLITE_FILE_VAR_NAME).unwrap_or_else(|_| panic!("{SQLITE_FILE_VAR_NAME} not set")),
    );
    let mut conn = Connection::open(sqlite_dir)?;
    let do_not_load_csv = var(EMPTY_CACHE_VAR_NAME).is_ok();
    if do_not_load_csv {
        MIGRATIONS.to_version(&mut conn, 1)?;
    } else {
        rusqlite::vtab::csvtab::load_module(&conn)?;
        MIGRATIONS.to_latest(&mut conn)?;
    }

    Ok(conn)
}

pub fn insert_experiment(
    conn: Arc<Mutex<Connection>>,
    args: TeebenchArgs,
    data: HashMap<String, String>,
) -> Result<()> {
    let conn = conn.lock().unwrap();
    conn.execute("INSERT INTO teebenchargs (app_name, dataset, algorithm, threads, selectivity, data_skew, seal_chunk_size, r_tuples, s_tuples, r_path, s_path, r_size, s_size, seal, sort_r, sort_s) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)", (&args.app_name.to_string(), &args.dataset.to_string(), &format!("{:?}", args.algorithm), &args.threads, &args.selectivity, &args.data_skew, &args.seal_chunk_size, &args.r_tuples, &args.s_tuples, &args.r_path, &args.s_path, &args.x, &args.y, &args.seal, &args.sort_r, &args.sort_s))?;
    let id = conn.last_insert_rowid();
    conn.execute("INSERT INTO output (teebenchargs_id, algorithm, threads, relR, relS, matches, phase1Cycles, phase2Cycles, cyclesPerTuple, phase1Time, phase2Time, totalTime, throughput, phase1L3CacheMisses, phase1L3HitRatio, phase1L2CacheMisses, phase1L2HitRatio, phase1IPC, phase1IR, phase1EWB, phase1VoluntaryCS, phase1InvoluntaryCS, phase1UserCpuTime, phase1SystemCpuTime, phase2L3CacheMisses, phase2L3HitRatio, phase2L2CacheMisses, phase2L2HitRatio, phase2IPC, phase2IR, phase2EWB, phase2VoluntaryCS, phase2InvoluntaryCS, phase2UserCpuTime, phase2SystemCpuTime, totalL3CacheMisses, totalL3HitRatio, totalL2CacheMisses, totalL2HitRatio, totalIPC, totalIR, totalEWB, totalVoluntaryCS, totalInvoluntaryCS, totalUserCpuTime, totalSystemCpuTime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45, ?46)", params![
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
        data.get("phase1L3CacheMisses"),
        data.get("phase1L3HitRatio"),
        data.get("phase1L2CacheMisses"),
        data.get("phase1L2HitRatio"),
        data.get("phase1IPC"),
        data.get("phase1IR"),
        data.get("phase1EWB"),
        data.get("phase1VoluntaryCS"),
        data.get("phase1InvoluntaryCS"),
        data.get("phase1UserCpuTime"),
        data.get("phase1SystemCpuTime"),
        data.get("phase2L3CacheMisses"),
        data.get("phase2L3HitRatio"),
        data.get("phase2L2CacheMisses"),
        data.get("phase2L2HitRatio"),
        data.get("phase2IPC"),
        data.get("phase2IR"),
        data.get("phase2EWB"),
        data.get("phase2VoluntaryCS"),
        data.get("phase2InvoluntaryCS"),
        data.get("phase2UserCpuTime"),
        data.get("phase2SystemCpuTime"),
        data.get("totalL3CacheMisses"),
        data.get("totalL3HitRatio"),
        data.get("totalL2CacheMisses"),
        data.get("totalL2HitRatio"),
        data.get("totalIPC"),
        data.get("totalIR"),
        data.get("totalEWB"),
        data.get("totalVoluntaryCS"),
        data.get("totalInvoluntaryCS"),
        data.get("totalUserCpuTime"),
        data.get("totalSystemCpuTime"),
    ])?;
    Ok(())
}

fn query_none<T>(val: &Option<T>, name: &str, idx: usize) -> String {
    match val {
        Some(_) => format!("{name}=?{idx}"),
        None => format!("{name} IS NULL"),
    }
}

pub fn search_for_exp(
    conn: Arc<Mutex<Connection>>,
    args: &TeebenchArgs,
) -> Result<Option<HashMap<String, String>>> {
    let conn = conn.lock().unwrap();
    debug!("Searching cache for {args:?}...");
    let arg_params = params![
        &args.app_name.to_string(),
        &args.dataset.to_string(),
        &format!("{:#?}", args.algorithm),
        &args.threads,
        &args.selectivity,
        &args.data_skew,
        &args.seal_chunk_size,
        &args.r_tuples,
        &args.s_tuples,
        &args.r_path,
        &args.s_path,
        &args.x,
        &args.y,
        &args.seal,
        &args.sort_r,
        &args.sort_s
    ];
    let id = conn.query_row(&format!("SELECT id FROM teebenchargs WHERE app_name=?1 AND dataset=?2 AND algorithm=?3 AND threads=?4 AND selectivity=?5 AND data_skew=?6 AND seal_chunk_size=?7 AND r_tuples=?8 AND s_tuples=?9 AND {} AND {} AND {} AND {} AND seal=?14 AND sort_r=?15 AND sort_s=?16", query_none(&args.r_path, "r_path", 10), query_none(&args.s_path, "s_path", 11), query_none(&args.x, "r_size", 12), query_none(&args.y, "s_size", 13)), arg_params, |r| r.get::<usize, usize>(0));
    let id = match id {
        Ok(id) => id,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            debug!("Command not found in `teebenchargs`.");
            return Ok(None);
        }
        Err(e) => bail!(e),
    };
    let mut map: HashMap<String, String> = HashMap::new();
    conn.query_row("SELECT algorithm, threads, relR, relS, matches, phase1Cycles, phase2Cycles, cyclesPerTuple, phase1Time, phase2Time, totalTime, throughput, phase1L3CacheMisses, phase1L3HitRatio, phase1L2CacheMisses, phase1L2HitRatio, phase1IPC, phase1IR, phase1EWB, phase1VoluntaryCS, phase1InvoluntaryCS, phase1UserCpuTime, phase1SystemCpuTime, phase2L3CacheMisses, phase2L3HitRatio, phase2L2CacheMisses, phase2L2HitRatio, phase2IPC, phase2IR, phase2EWB, phase2VoluntaryCS, phase2InvoluntaryCS, phase2UserCpuTime, phase2SystemCpuTime, totalL3CacheMisses, totalL3HitRatio, totalL2CacheMisses, totalL2HitRatio, totalIPC, totalIR, totalEWB, totalVoluntaryCS, totalInvoluntaryCS, totalUserCpuTime, totalSystemCpuTime FROM output WHERE teebenchargs_id=(?1)", [id], |r| {
        map.insert("algorithm".to_owned(), r.get::<usize, String>(0)?);
        map.insert("threads".to_owned(), r.get::<usize, usize>(1).map(|v| v.to_string())?);
        map.insert("relR".to_owned(), r.get::<usize, usize>(2).map(|v| v.to_string())?);
        map.insert("relS".to_owned(), r.get::<usize, usize>(3).map(|v| v.to_string())?);
        map.insert("matches".to_owned(), r.get::<usize, usize>(4).map(|v| v.to_string())?);
        map.insert("phase1Cycles".to_owned(), r.get::<usize, usize>(5).map(|v| v.to_string())?);
        map.insert("phase2Cycles".to_owned(), r.get::<usize, usize>(6).map(|v| v.to_string())?);
        map.insert("cyclesPerTuple".to_owned(), r.get::<usize, usize>(7).map(|v| v.to_string())?);
        map.insert("phase1Time".to_owned(), r.get::<usize, usize>(8).map(|v| v.to_string())?);
        map.insert("phase2Time".to_owned(), r.get::<usize, usize>(9).map(|v| v.to_string())?);
        map.insert("totalTime".to_owned(), r.get::<usize, usize>(10).map(|v| v.to_string())?);
        map.insert("throughput".to_owned(), r.get::<usize, f64>(11).map(|v| v.to_string())?);
        if let Some(val) = r.get::<usize, Option<usize>>(12)? {
            let val = val.to_string();
            map.insert("phase1L3CacheMisses".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(13)? {
            let val = val.to_string();
            map.insert("phase1L3HitRatio".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(14)? {
            let val = val.to_string();
            map.insert("phase1L2CacheMisses".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(15)? {
            let val = val.to_string();
            map.insert("phase1L2HitRatio".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(16)? {
            let val = val.to_string();
            map.insert("phase1IPC".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(17)? {
            let val = val.to_string();
            map.insert("phase1IR".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(18)? {
            let val = val.to_string();
            map.insert("phase1EWB".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(19)? {
            let val = val.to_string();
            map.insert("phase1VoluntaryCS".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(20)? {
            let val = val.to_string();
            map.insert("phase1InvoluntaryCS".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(21)? {
            let val = val.to_string();
            map.insert("phase1UserCpuTime".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(22)? {
            let val = val.to_string();
            map.insert("phase1SystemCpuTime".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(23)? {
            let val = val.to_string();
            map.insert("phase2L3CacheMisses".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(24)? {
            let val = val.to_string();
            map.insert("phase2L3HitRatio".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(25)? {
            let val = val.to_string();
            map.insert("phase2L2CacheMisses".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(26)? {
            let val = val.to_string();
            map.insert("phase2L2HitRatio".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(27)? {
            let val = val.to_string();
            map.insert("phase2IPC".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(28)? {
            let val = val.to_string();
            map.insert("phase2IR".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(29)? {
            let val = val.to_string();
            map.insert("phase2EWB".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(30)? {
            let val = val.to_string();
            map.insert("phase2VoluntaryCS".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(31)? {
            let val = val.to_string();
            map.insert("phase2InvoluntaryCS".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(32)? {
            let val = val.to_string();
            map.insert("phase2UserCpuTime".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(33)? {
            let val = val.to_string();
            map.insert("phase2SystemCpuTime".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(34)? {
            let val = val.to_string();
            map.insert("totalL3CacheMisses".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(35)? {
            let val = val.to_string();
            map.insert("totalL3HitRatio".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(36)? {
            let val = val.to_string();
            map.insert("totalL2CacheMisses".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(37)? {
            let val = val.to_string();
            map.insert("totalL2HitRatio".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<f64>>(38)? {
            let val = val.to_string();
            map.insert("totalIPC".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(39)? {
            let val = val.to_string();
            map.insert("totalIR".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(40)? {
            let val = val.to_string();
            map.insert("totalEWB".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(41)? {
            let val = val.to_string();
            map.insert("totalVoluntaryCS".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(42)? {
            let val = val.to_string();
            map.insert("totalInvoluntaryCS".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(43)? {
            let val = val.to_string();
            map.insert("totalUserCpuTime".to_owned(), val);
        }
        if let Some(val) = r.get::<usize, Option<usize>>(44)? {
            let val = val.to_string();
            map.insert("totalSystemCpuTime".to_owned(), val);
        }
        Ok(())
    })?;
    Ok(Some(map))
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
