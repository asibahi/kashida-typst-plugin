This is a [typst wasm plugin](https://typst.app/docs/reference/foundations/plugin/) to use [kashida](https://github.com/asibahi/kashida) in typst documents. This is probably pretty useless but a useful proof of concept of both a simple plugin and a simple rust library.

to build: you need to have rust installed

```sh
cargo build --release --target wasm32-unknown-unknown    
```

Or just download the wasm binary from releases.

For usage, see `example.typ`.