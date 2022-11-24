TeeBenchWeb
===========

A frontend for [TeeBench](https://github.com/agora-ecosystem/tee-bench).

Requirements
------------

- Rust (best via [rustup.rs](rustup.rs), latest version or at least 1.56 (untested))
- Rust target `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- Trunk (install as explained on their [website](https://trunkrs.dev/))

Testing the web app
-------------------

For this the location of static assets needs to change. To prevent this from interfering with the build of the server, we'll also change the dist directory.

```
trunk serve -d "dist" --public-url "/"
```

Running the server app with the web app
---------------------------------------

First build the web app while in the `frontend` directory with `trunk build`. Then change to the `backend` directory and run the axum server: `cargo run`.

Testing
-------

... is very lacking.
Check the json extractor of the server with
```sh
curl --header "Content-Type:application/json" \
--request POST \
--data '{"title": "first commit", "datetime": "2020-03-28T16:29:04.644008111Z", "code": "auto a = 2", "report": null }' \
http://localhost:3000/api/commit
```
