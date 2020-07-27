# Private-ID Web

Web-Assembly library for [Private-ID generation](
https://engineering.fb.com/open-source/private-matching/) adapted from
and inspired by [this](https://github.com/facebookresearch/Private-ID) Rust implementation by Facebook Research

### Build for web

```shell
npm run-script build  # dev: build-debug

# same as
wasm-pack build --release --no-typescript --target no-modules; npm run-script link-web
```

### Run tests locally

```shell
npm test

# same as
cd src/lib; cargo run --bin private-id-test
```

### TODO

 - parallelism using web workers
 -