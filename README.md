# nostalgiatan.ping

Minimal Tianshu community plugin. One `ping` node writes `Ping: <text>` down the pipe.

```bash
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/tianshu_plugin_ping.wasm plugin.wasm
```

Pack as TSP2 `.tsz` and attach it to a GitHub Release together with `release.json` (signed with your Tianshu user key). Then open a PR on [tianshu-plugin-index](https://github.com/tianshu48/tianshu-plugin-index) with `proposals/nostalgiatan.ping/1.0.0.json`.
