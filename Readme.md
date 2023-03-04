TeeBenchWeb
===========

A frontend for [TeeBench](https://github.com/agora-ecosystem/tee-bench).

Requirements
------------

- Rust (best via [rustup.rs](rustup.rs), latest version or at least 1.65)
- Rust target `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- Trunk (install as explained on their [website](https://trunkrs.dev/))

Running the server app with the web app
---------------------------------------

Use `run.sh`. You can set the environment variable `TEEBENCHWEB_RUN_DIR` to a directory with two versions of TeeBench: one folder, called "sgx" with an SGX version of TeeBench, where the executable is called `sgx`. And another folder, called "native" with the executable called `native`.

If you do not set this environment variable, TeeBenchWeb will use a fake version of TeeBench which outputs precomputed results.

```sh
export TEEBENCHWEB_RUN_DIR="<path to TeeBench>"; ./run.sh
```

Or, first build the web app while in the `frontend` directory with `trunk build`. Then change to the `backend` directory and run the axum server: `cargo run` (This part needs the environment variable `TEEBENCHWEB_RUN_DIR` set to the directory of TeeBench, as explained above).

Running the axum server from the workspace does not work, as the path to the `dist` directory that trunk created is then wrong. Just switch to the `backend` directory or use the `run.sh` script.

==BUG:== There is a bug in trunk/cargo that makes any rebuild a full rebuild (ie. not reusing previous unchanged artifacts). The workaround I used is to change the artifact directory of the trunk build (via `frontend/.cargo/config.toml`). It still seems to happen sometimes, but not always.

Testing the web app
-------------------

For this the location of static assets needs to change. To prevent this from interfering with the build of the server, we'll also change the dist directory. 

```
cd frontend
trunk serve -d "dist" --public-url "/"
```

Testing
-------

There are some tests, run them with `cargo test`.

Check the json extractor of the server with
```sh
curl --header "Content-Type:application/json" \
--request POST \
--data '{"title": "first commit", "datetime": "2020-03-28T16:29:04.644008111Z", "code": "auto a = 2", "report": null }' \
http://localhost:3000/api/commit
```
