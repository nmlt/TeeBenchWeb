#!/bin/bash
trap '' INT # Disables Ctrl+C

run_backend () (
   trap - INT # Enables Ctrl+C in here
   cargo run
)

pushd frontend
trunk build
popd

pushd backend
run_backend
popd
