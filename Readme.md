TeeBenchWeb
===========

A frontend for [TeeBench](https://github.com/agora-ecosystem/tee-bench).

Requirements
------------

- Rust (best via [rustup.rs](rustup.rs), latest version or at least 1.65)
- Rust target `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- Trunk (install as explained on their [website](https://trunkrs.dev/))
- [TeeBench](https://github.com/agora-ecosystem/tee-bench)

Running the server app with the web app
---------------------------------------

### Setup

This assumes that TeeBench is in a subdirectory called `tee-bench`. Change the created `.env` file when `tee-bench` is somewhere else.

```sh
mv .env.example .env
```

### Run

```sh
./run.sh # You will be asked for your password by sudo
```

Use `run.sh`. You can set the environment variable `TEEBENCHWEB_RUN_DIR` to a directory with TeeBench. TeeBenchWeb will compile TeeBench, create a `bin` folder, and copy the executables (native and sgx) and enclave file there. It also relies on TeeBench having a `Joins/TBW/` folder, in which it will copy uploaded Operators (called Commits in the code) into a file called `OperatorJoin.cpp`.

If you do not set this environment variable, TeeBenchWeb will use a fake version of TeeBench which outputs precomputed results.

```sh
export TEEBENCHWEB_RUN_DIR="<path to TeeBench>"; ./run.sh
```

==FIRST RUN==: On the first run, first upload an operator (that compiles successfully) to compile TeeBench. Otherwise Profiling will crash the application. Afterwards, even after restarting TBW, the `bin` directory should already exist and always be filled with functioning executables.

Or, first build the web app while in the `frontend` directory with `trunk build`. Then change to the `backend` directory and run the axum server: `cargo run` (This part needs the environment variable `TEEBENCHWEB_RUN_DIR` set to the directory of TeeBench, as explained above).

Running the axum server from the workspace does not work, as the path to the `dist` directory that trunk created is then wrong. Just switch to the `backend` directory or use the `run.sh` script.

==BUG:== There is a bug in trunk/cargo that makes any rebuild a full rebuild (ie. not reusing previous unchanged artifacts). The workaround I used is to change the artifact directory of the trunk build (via `frontend/.cargo/config.toml`). It still seems to happen sometimes, but not always.

Testing the static web app
-------------------

For this the location of static assets needs to change. To prevent this from interfering with the build of the server, we'll also change the dist directory. `trunk` by default serves the webapp at port 8080. Add `--port 3000` to change it to 3000. 

To get the static files to be served by another server, just take all the files in the created `frontend/dist` folder.

```
cd frontend
trunk serve -d "dist" --public-url "/" --features static
```

Testing
-------

There are some tests, run them with `cargo test`.

