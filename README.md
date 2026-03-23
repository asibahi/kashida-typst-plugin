This lives as [an example](https://git.sr.ht/~asibahi/kashida/tree/main/item/examples/typst-plugin) in the [`kashida` repositry](https://git.sr.ht/~asibahi/kashida). No updates will be made here, not that it needs any. To build it there: `just build-typst-plugin`.

This is a [typst wasm plugin](https://typst.app/docs/reference/foundations/plugin/) to use [kashida](https://git.sr.ht/~asibahi/kashida) in typst documents. This is probably pretty useless but a useful proof of concept of both a simple plugin and a simple rust library.

to build: you need to have rust installed

```sh
cargo build --release --target wasm32-unknown-unknown    
```

Or just download the wasm binary from releases.

For usage, see `example.typ`.
