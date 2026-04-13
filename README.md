# Nano


Notis tehr is a limetid web wesion of this prject under the bracnh web-version. It is not the full game or engein i build but a part that is only for web and only for testing purposes or demo. The main project is going to go full desktip and dropp web build supprot ( main branch ) so interpelation with c or cpp can be achived or even moddin supprot with lua ( currently working on lua skirpng ) -> only possible on desktop because of mlua-sys or ohter dependecies. 

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

Around ~15-60 gb of storage are requierd to build

The executable is around 1.4 gb in debug and in in relece mod 0.5 gb or so


*****

Bevy insperation:

https://bevy.org/examples/