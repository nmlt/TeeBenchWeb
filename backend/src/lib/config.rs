pub const SQLITE_FILE_VAR_NAME: &str = "TEEBENCHWEB_SQLITE_FILE";
pub const RUN_DIR_VAR_NAME: &str = "TEEBENCHWEB_RUN_DIR";
/// Set this variable to disable loading default data to the cache
pub const EMPTY_CACHE_VAR_NAME: &str = "TEEBENCHWEB_EMPTY_CACHE";
/// Path to the file that should be read in as default content of the table `teebenchargs`. Relative paths are relative to the `backend` subdirectory.
pub const TEEBENCHARGS_CSV_PATH: &str = "../cached/teebenchargs.csv";
/// Path to the file that should be read in as default content of the table `output`. Relative paths are relative to the `backend` subdirectory.
pub const OUTPUT_CSV_PATH: &str = "../cached/output.csv";

// TODO Eventually we should evaluate all environment variables first thing after the backend was started, via a function in here, that is called in the main.rs.
