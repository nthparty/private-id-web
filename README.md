# Private-ID-Web
WebAssembly variant and accompanying browser interface for [Private-ID generation](
https://engineering.fb.com/open-source/private-matching/) adapted from
and inspired by the [Rust variant of Private-ID by Facebook Research](https://github.com/facebookresearch/Private-ID).

## Development Dependencies
The build process for this project requires [Rust](https://www.rust-lang.org/tools/install) and [Node.js](https://nodejs.org/). In addition, it is necessary to install [wasm-pack](https://rustwasm.github.io/wasm-pack/):
```shell
cargo install wasm-pack
```

## Build Process
The build process can be invoked in the following way via the Node package definitions:
```shell
npm run-script build # dev: build-debug
```
Alternatively, it can be invoked directly in the following way:
```shell
wasm-pack build --release --no-typescript --target no-modules && npm run-script link-web
```

## Testing
Tests can be invoked in the following way via the Node package definitions:
```shell
npm test
```
Alternatively, they can be invoked directly in the following way:
```shell
cd src/lib; cargo run --bin private-id-test
```

## Running in the Browser
Navigate two separate browser tabs or windows to the following two URLs (replacing `<localhost>` as necessary):
```
http://<localhost>/index.html?this=aaabcc&other=cccdee  # Party A example.
http://<localhost>/index.html?other=cccdee&this=aaabcc  # Party B example.
```

## TODOs
 - parallelism using web workers
 - compress messages as base64
