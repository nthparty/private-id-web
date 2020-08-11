# Private-ID Web

Web-Assembly library for [Private-ID generation](
https://engineering.fb.com/open-source/private-matching/) adapted from
and inspired by [this](https://github.com/facebookresearch/Private-ID) Rust implementation by Facebook Research

### Dev dependency

```shell
cargo install wasm-pack
```

### Build for web

```shell
npm run-script build  # dev: build-debug

# same as
wasm-pack build --release --no-typescript --target no-modules && npm run-script link-web
```

### Run tests locally

```shell
npm test

# same as
cd src/lib; cargo run --bin private-id-test
```

### Run the web demo

Navigate to
```
http://<localhost>/index.html?this=aaabcc&other=cccdee  # Party A example

http://<localhost>/index.html?other=cccdee&this=aaabcc  # Party B example
```

### TODO

 - parallelism using web workers
 - compress messages as base64
