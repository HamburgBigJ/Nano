# Nano

es gabt ein problem dass ich mit dem projejct ienfach nicht ferig werde, jetz habe ich mich entschiden einfalch ein powsder Simulation zu bauen was deutlich realisticsch is ferig zu machen.

die laten protocolle und code lasse sich im dev oder Web-Version finden, und die protokolle unter der git commit history


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