# Nano

Install:
```
cargo install wasm-server-runner
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

Run:

```
cargo build --target wasm32-unknown-unknown
trunk serve
```
