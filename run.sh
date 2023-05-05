#!/bin/bash
trap '' INT # Disables Ctrl+C
set -o errexit   # abort on nonzero exitstatus
set -o pipefail  # don't hide errors within pipes

# source: https://stackoverflow.com/a/21188136/15282333
get_abs_filename() {
    # $1 : relative filename
    echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
}

cleanup_cache_db() {
    # This only works as long as we didn't change the directory since starting the backend executable
    rm -f "${TEEBENCHWEB_SQLITE_FILE}"
    kill -term $$
}

run_backend () (
    trap "cleanup_cache_db" INT # Runs cleanup_cache_db on Ctrl+C
    if [[ -z "${TEEBENCHWEB_RUN_DIR}" ]]; then
        cargo build --bin fake_teebench
        mkdir -p ../fake_teebench/Joins/TBW
        touch ../fake_teebench/Joins/TBW/OperatorJoin.cpp
        touch ../fake_teebench/enclave.signed.so
        cat <<EOF > ../fake_teebench/Makefile
native:
	echo "Recipe native running in Makefile"

sgx:
	echo "Recipe sgx running in Makefile"

EOF
        cp ../target/debug/fake_teebench ../fake_teebench/app
        export TEEBENCHWEB_RUN_DIR=$(get_abs_filename "../fake_teebench")
    fi
    if [[ -z "${TEEBENCHWEB_SQLITE_FILE}" ]]; then
        echo "TEEBENCHWEB_SQLITE_FILE not set!"
        return -1
    fi
    cargo build --bin backend
    sudo -E ./../target/debug/backend
)

build_frontend () (
    trap - INT # Enables Ctrl+C in here
    trunk build
)

pushd frontend
build_frontend
popd

pushd backend
run_backend
popd
