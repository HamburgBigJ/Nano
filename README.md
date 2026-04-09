# Nano

## Dependencys:

for web:
```
cargo install wasm-server-runner
cargo install --locked trunk
```
maby:
```
rustup target add wasm32-unknown-unknown
```


## Build / Run
Editor:
```
cargo run --bin editor
```

Game:
Desktop:
```
cargo run --bin game
```

Debug
```
RUST_LOG=debug cargo run --bin game
```

**May have some problems**
```
cargo build --target wasm32-unknown-unknown
trunk serve
```

for faster runtime `RUSTFLAGS='-C target-cpu=native'`

```
RUSTFLAGS='-C target-cpu=native' cargo build -r --target-dir build-release/
```

```
cargo build --profile dev --target-dir build-dev/
```


Warning compile time can wary up to ~6-9 min depending on hardware


Bevy insperation:

https://bevy.org/examples/