# WebUwU

A real-time, high performance, and customizable text UwUifier written in Rust. The algorithm
is a fork of another UwUing project I wrote with @sgoudham. [This one](https://github.com/sgoudham/uwuifyy)

## Features

- Blazingly fast using Rust WASM with [rust-dominator](https://github.com/Pauan/rust-dominator)
- Lots of customization to get that *perfect* UwU out of your text
- Responsive design that looks great on desktop *and* mobile!
- Dead easy to deploy using [trunk](https://github.com/thedodd/trunk)
- Very fast to load, only needing just 500KB

## Cloning

Make sure you clone this project with
```sh
git clone --recursive https://github.com/StratusFearMe21/webuwu
```

## Building

You must add `RUSTFLAGS="--cfg=web_sys_unstable_apis"` to your build commands because this app uses the
`Navigator.Clipboard` API, which is unstable.

### Deploying

Just run trunk
```sh
RUSTFLAGS="--cfg=web_sys_unstable_apis" trunk build --release
```

### Testing

Just run trunk
```sh
RUSTFLAGS="--cfg=web_sys_unstable_apis" trunk serve
```