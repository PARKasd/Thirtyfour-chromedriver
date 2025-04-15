# 🦀 Rust + Thirtyfour: Auto ChromeDriver Installer

This Rust project automatically downloads the correct version of ChromeDriver based on the version of Chrome installed on your system. It uses the [`thirtyfour`](https://crates.io/crates/thirtyfour) crate for WebDriver automation, and integrates tools to fetch, unzip, and launch the matching driver.
I'm newbie in Rust so code is dirty.
## 🚀 Features

- Detect installed Chrome version (cross-platform support)
- Automatically download the matching ChromeDriver
- Unzip and configure the driver binary
- Launch Chrome using [`thirtyfour`](https://crates.io/crates/thirtyfour)

## 📦 Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
platform-dirs = "0.3.0"
reqwest = "0.12.15"
anyhow = "1.0.97"
indicatif = "0.17.11"
zip = "2.1.1"
zip-extensions = "0.7.0"
regex = "1.11.1"
log = "0.4.27"
thirtyfour = "0.35.0"
tokio = { version = "1.0.0", features = ["rt", "rt-multi-thread", "macros"] }
