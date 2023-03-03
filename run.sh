#!/bin/bash
trap '' INT # Disables Ctrl+C
set -o errexit   # abort on nonzero exitstatus
set -o pipefail  # don't hide errors within pipes

# source: https://stackoverflow.com/a/21188136/15282333
get_abs_filename() {
  # $1 : relative filename
  echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
}


run_backend () (
    trap - INT # Enables Ctrl+C in here
    if [[ -z "${TEEBENCHWEB_RUN_DIR}" ]]; then
       cargo build --bin fake_teebench
       mkdir -p ../fake_teebench/sgx
       mkdir -p ../fake_teebench/native
       cp ../target/debug/fake_teebench ../fake_teebench/sgx/sgx
       cp ../target/debug/fake_teebench ../fake_teebench/native/native
       export TEEBENCHWEB_RUN_DIR=$(get_abs_filename "../fake_teebench")
    fi
    cargo run
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
