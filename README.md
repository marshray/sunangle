# Sunangle!

One can take measurements of the angle of the Sun from various locations times at various times.
This is an app to visualize the resulting geometry.

You can test the `sunangle` app at <https://marshray.github.io/sunangle/>.

This app is grateful to the `egui` project authors and their generosity for the
template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe),
a framework for writing apps using awesome [egui](https://github.com/emilk/egui/) UI,
a simple and straightforward way to get started writing a GUI app in Rust.

You can build it natively and run it on your local system,
or build it for the web and share it using Github Pages.

### Build and run locally as a native app

#### Set up
Make sure you are using the latest version of stable rust by running `rustup update`.

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

#### Build
From the top of your checkout (the directory containing this `README.md`):
1. Clean
   > `cargo clean; cargo clean --release`
1. Run
   > `cargo build`
   or
   > `cargo build --release`

#### Run
1. Run
   > `cargo run`
   or
   > `cargo run --release`

Then to run the app

`cargo run --release`

### Build, test locally, and deploy as a [WASM](https://en.wikipedia.org/wiki/WebAssembly) web app

#### Set up
[Trunk](https://trunkrs.dev/) is used to build for the web.
1. Install the `wasm32` compiler target
   > `rustup target add wasm32-unknown-unknown`
1. Install Trunk
   > `cargo install --locked trunk`

#### Build
From the top of your checkout (the directory containing this `README.md`):
1. Clean
   > `trunk clean`
1. Some of the build steps are extremely verbose at `debug` or `trace` level,
so reflect upon any value you may have set for the `RUST_LOG` environment variable.
   > `echo $RUST_LOG`
1. Build
   > `trunk build --release`

#### Test locally
1. **Build**, as described above.
1. Serve the build results over `http` on `127.0.0.1:8080` at the path `/sunangle`:
   > `trunk serve --release`
   * Note: Trunk will watch for changes to project files and rebuild automatically.
1. Navigate to [`http://127.0.0.1:8080/sunangle/index.html#dev`](
    http://127.0.0.1:8080/sunangle/index.html#dev) in a web browser.
   * Note: The `assets/sw.js` script will try to cache our app, and will load the cached version
   when it cannot connect to server, allowing your app to work offline (like PWA).
   Appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds
   during development.

#### Deploy to web
1. **Build**, as described above.
1. Upload the resulting `dist` directory to your website such that it is served at `/sunangle/` path.
* [GitHub Pages](
https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pagesconfiguring-a-publishing-source-for-your-github-pages-site
) is a free option used by many projects already using GitHub.
    1. Ensure you have a `gh-pages` branch with the top-level `index.html` in either
    `/` (root) or `docs`.
    1. Go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root) or `docs`.
    * There is a workflow that you can enable to auto-deploy the app to GitHub pages.

# Development

## Source organization

- `sunangle/`
  - `Cargo.toml` The workspace
  - `sunangle/` The `sunangle` package
    - `Cargo.toml`
    - `src/`
      - `lib.rs` Defines module structure, re-exports `SunangleApp`.
      - `main.rs` Program startup, `fn main()`
      - `ui/` Egui-based ui
      - `threed/` three_d based view
      - `egui_app.rs` The main application struct, `SunangleApp`.
        - Implements the  [`eframe::App`](
          https://docs.rs/eframe/latest/eframe/trait.App.html) trait.
          - Local save state logic
          - "Updates" the UI on every redraw
      - `tai.rs` Type representing International Atomic Time, TAI
      - `world_state.rs` A representation of the world, computed from a point in time
      - `view_state.rs` A representation of the view, camera position, etc.
      - `time/` Some old code for time, mostly using `chrono` instead
    - `Trunk.toml` Configures `trunk` build system for WASM
    - `www/`
      - `index.html` Main web page, but also controls `trunk` build
      - `assets/` used in building the web app
  - `rust-toolchain` Specifies what components are needed from `rustup`

## Learning about egui

Official egui docs: <https://docs.rs/egui>

A video introduction: [Getting started with Rust 🦀 2021: 7a. Building a GUI app in Rust [Part A]](https://www.youtube.com/watch?v=NtUkr_z7l84)

For inspiration, check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links to its source code.

## Updating egui

As of 2023, egui is in active development with frequent releases with breaking changes.
* [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.
* When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md).
