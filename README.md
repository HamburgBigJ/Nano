# Nano


Notis tehr is a limetid web wesion of this prject under the bracnh web-version. It is not the full game or engein i build but a part that is only for web and only for testing purposes or demo. The main project is going to go full desktip and dropp web build supprot ( main branch ) so interpelation with c or cpp can be achived or even moddin supprot with lua ( currently working on lua skirpng ) -> only possible on desktop because of mlua-sys or ohter dependecies. 

for the web version:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 JSON                      9           72           68            0            4
 Rust                     26          812          585           61          166
 TOML                      5           92           75            6           11
─────────────────────────────────────────────────────────────────────────────────
 HTML                      1           12           12            0            0
 |- CSS                    1            2            2            0            0
 (Total)                               14           14            0            0
─────────────────────────────────────────────────────────────────────────────────
 Markdown                 11          152            0           97           55
 |- JSON                   1           39           39            0            0
 (Total)                              191           39           97           55
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                    52         1181          781          164          236
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```


## Dependencys:

for web:
```shell
cargo install wasm-server-runner
cargo install --locked trunk
```
maby:
```shell
rustup target add wasm32-unknown-unknown
```


## Build / Run
Editor:
```shell
cargo run --bin editor
```

Game:
Desktop:
```shell
cargo run --bin game
```

profiling:
```shell
cargo run --release --bin game --features bevy/trace_tracy
```

Debug
```shell
RUST_LOG=debug cargo run --bin game
```

**May have some problems**
```shell
cargo build --target wasm32-unknown-unknown
trunk serve
```

for faster runtime `RUSTFLAGS='-C target-cpu=native'`

```shell
RUSTFLAGS='-C target-cpu=native' cargo build -r --target-dir build-release/
```

```shell
cargo build --profile dev --target-dir build-dev/
```


Warning compile time can wary up to ~6-9 min depending on hardware

Around ~15-60 gb of storage are requierd to build

The executable is around 1.4 gb in debug and in in relece mod 0.5 gb or so


*****

Bevy insperation:

https://bevy.org/examples/