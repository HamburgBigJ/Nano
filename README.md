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


## Ai notice

Teile von dem js und mod loading sind mit ai generirt ( claude )
Documatioan für mods wie auch nano-api.ts für autocompleet is ai generirt


# TODO

mod list with json to enable / disable mods
mods not loading in the same directory as nano

Sand auf wasser fallen not implemented
Explosion chained