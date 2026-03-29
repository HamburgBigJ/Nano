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

**May be not ready yet**
Web: Currenty complety broken
```
cargo build --target wasm32-unknown-unknown
trunk serve
```