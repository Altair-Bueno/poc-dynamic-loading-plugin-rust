# POC: Dynamically loading Rust plugins

This POC showcases how a Rust application can (re)load plugins by using [Dynamic Link Libraries (DLL)](https://en.wikipedia.org/wiki/Dynamic-link_library).

## Running the example

### Requirements

- Rust
- An operating system with support for some sort of DLL support.

### Executing the example

1. Compile all the plugins and the application. Because the application does not depend on any plugins, they are not automatically built. This step must be done before running the aplication for the POC to be usefull.

```sh
cargo build --release
```

2. Run the aplication. We are using `cargo` to simplify the execution, but it can be done without it. See [packaging and distribution](#packaging-and-distribution) for more information. Change the extension to match your operating system.

```sh
cargo run --release -- libhello_world.dylib
```

# FAQs

## Sharing libraries between the application and plugins

Take a look to [cargo-dynamic](https://github.com/rksm/cargo-add-dynamic). Please note that some dependencies might be tricky to set up, particuarly those that use global variables. Some examples include [`tracing`](https://github.com/tokio-rs/tracing/issues/1478) and [`log`](https://github.com/rust-lang/log/issues/421).

## Packaging and distribution

Because all the code is dynamically linked, all dynamic dependencies must be included with the resulting application binary. These include all generated under the `target` directory, plus some other that live within the Rust installation. you can find these files by running `rustc --print sysroot`.

For example, on a macOS x86_64 system, we can run the application without `cargo` by using the following command:

```sh
DYLD_LIBRARY_PATH="$(rustc --print sysroot)/lib/rustlib/x86_64-apple-darwin/lib:$PWD/target/release" target/release/app libhello_world.dylib
```

And we can list the exact dependencies of our project loads by running the following command:

```sh
dyld_info -dependents target/release/{app,lib{hello_world,app_core}.dylib}
```

# Usefull links

- https://robert.kra.hn/posts/2022-09-09-speeding-up-incremental-rust-compilation-with-dylibs/
- https://www.reddit.com/r/rust/comments/1bjae38/media_fyrox_now_supports_hot_reloading_you_can/
  - https://github.com/FyroxEngine/Fyrox/blob/e7493ccbf39a12ce14eca08f0f3438b32159697a/fyrox-impl/src/engine/mod.rs#L2452
  - https://github.com/FyroxEngine/Fyrox/blob/e7493ccbf39a12ce14eca08f0f3438b32159697a/fyrox-impl/src/plugin/dynamic.rs
