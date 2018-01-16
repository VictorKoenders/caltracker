# Client
To run this client, you first need to install `cargo-web` and the `wasm32-unknown-unknown` target:

``` bash
cargo install cargo-web
rustup target add wasm32-unknown-unknown 
```

Then you can build and run the project with a simple command:

``` bash
cargo web start --target-webasm 
```

Note: you can add the `--host <host>` and `--port <port>` parameters to change this

