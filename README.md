# Rust + Thirtyfour: Auto ChromeDriver Installer

This Rust project automatically downloads the correct version of ChromeDriver based on the version of Chrome installed on your system. It uses the [`thirtyfour`](https://crates.io/crates/thirtyfour) crate for WebDriver automation, and integrates tools to fetch, unzip, and launch the matching driver.
I'm newbie in Rust so code is dirty.
## 🚀 Features

- Detect installed Chrome version (cross-platform support)
- Automatically download the matching ChromeDriver
- Unzip and configure the driver binary
- Launch Chrome using [`thirtyfour`](https://crates.io/crates/thirtyfour)
- Manage Old ChromeDrivers

## 📦 Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
Thirtyfour-chromedriver = "0.1.5"
```

## Code Example

```rust
use thirtyfour::prelude::*;

// Require the Handler
use thirtyfour_chromedriver::{manager::Handler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create Chrome capabilities
    let mut caps = DesiredCapabilities::chrome(); 

    // Launch chromedriver on port 9515 
    let mut chromedriver = Handler::new()
        .launch_chromedriver(&mut caps, "9515")
        .await?;

    // Connect to chrome on the same port
    let driver = WebDriver::new("http://localhost:9515", caps).await?; 

    // Close the proccess after tasks are finished
    chromedriver.kill()?;

    Ok(())
}
```