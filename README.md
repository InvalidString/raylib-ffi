# Direct raylib bindings for Rust
[![Crates.io](https://img.shields.io/crates/v/raylib-ffi)](https://crates.io/crates/raylib-ffi)
[![Docs.rs](https://docs.rs/raylib-ffi/badge.svg)](https://docs.rs/raylib-ffi)
[![Build Status](https://github.com/Ewpratten/raylib-ffi/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/raylib-ffi/actions/workflows/build.yml)

`raylib-ffi` aims to provide a "no frills" direct binding to [raylib](https://www.raylib.com/) for rust developers.

## Dependencies

Fedora:

```sh
dnf install clang-devel alsa-lib-devel mesa-libGL-devel libX11-devel libXrandr-devel libXi-devel libXcursor-devel libXinerama-devel libatomic cmake
```

## Verifying your build

`raylib-ffi` bundles a rust version of the example project from raylib proper. To verify your build of this library worked, run:

```sh
cargo run --example basic
```

## Versioning policy

`raylib-ffi` follow [SemVer](https://semver.org/).

The major and minor version numbers of a `raylib-ffi` will always match the version of raylib it was built against. The patch version may be incremented if a rust-only fix is needed at any point.
