# POC: Dynamically loading Rust plugins

This POC showcases how a Rust application can (re)load plugins by using [Dynamic Link Libraries (DLL)](https://en.wikipedia.org/wiki/Dynamic-link_library).

## Running the example

### Requirements

- Rust
- An operating system with support for DLL (e.g. macOS, Windows, Linux, ...)

### Executing the example

1. Compile all the plugins and the application. Because the application does not depend on any plugins directly, we need to manually build those.

```sh
cargo build --release
```

2. Run the aplication. We are using `cargo` to simplify the execution, but it can be done without it. See [packaging and distribution](#packaging-and-distribution) for more information.

```sh
# Change the extension to match your operating system
# macOS: .dylib
# Windows: .dll
# Linux: .so
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

And we can list the exact dependencies of our project requires by running the following command:

```sh
dyld_info -dependents target/release/{app,lib{hello-world,app_core}.dylib}
```

# Usefull links

- Speeding up incremental Rust compilation with dylibs: https://robert.kra.hn/posts/2022-09-09-speeding-up-incremental-rust-compilation-with-dylibs/
- Fyrox hot reload demo: https://www.reddit.com/r/rust/comments/1bjae38/media_fyrox_now_supports_hot_reloading_you_can/
  - Fyrox plugin system https://github.com/FyroxEngine/Fyrox/blob/e7493ccbf39a12ce14eca08f0f3438b32159697a/fyrox-impl/src/engine/mod.rs#L2452
  - Fyrox dynamic loader: https://github.com/FyroxEngine/Fyrox/blob/e7493ccbf39a12ce14eca08f0f3438b32159697a/fyrox-impl/src/plugin/dynamic.rs
- Reddit discussion: https://www.reddit.com/r/rust/comments/1bmqhui/plugins_systems_in_rust_using_dynamic_link/
- More usefull links: https://www.reddit.com/r/rust/comments/1bmqhui/comment/kwgtq9m/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
