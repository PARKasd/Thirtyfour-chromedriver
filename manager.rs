use std::fs;
use std::fs::File;
use std::path::PathBuf;
use crate::dw_module::*;

pub async fn download_chromedriver(client: &reqwest::Client, dw_link: String) -> anyhow::Result<()> {
    let version_info: String;
    let (chrome_exe, chromedriver_exe) = get_file_names();
    let dw_name : String;
    if cfg!(target_os = "windows") {

        if cfg!(target_pointer_width = "64") {
            dw_name = "chromedriver-win64".to_string();
        }
        else{
            dw_name = "chromedriver-win32".to_string();
        }

    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            dw_name = "chromedriver-mac-x64".to_string();
        }
        else {
            dw_name = "chromedriver-mac-arm64".to_string();
        }
    } else{
        dw_name = "chromedriver-linux64".to_string();
    }

    let driver_path = get_cache_dir().join(PathBuf::from(dw_name).with_extension("zip"));
    let response = client.get(&dw_link).send().await?;
    let file = File::create(&driver_path)?;

    write_file(
        &file,
        response,
        format!("Downloading Chromedriver ({})", &dw_link),
    )
        .await?;

    println!("Extracting Chromedriver...");
    zip_extract(&driver_path, &get_cache_dir()).unwrap();

    println!("Completed Chromedriver Download ({})", &dw_link);

    // Delete zip file
    fs::remove_file(&driver_path).unwrap();

    Ok(())
}
impl Handler {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn launch_chromedriver(
        &mut self,
        capabilities: &mut ChromeCapabilities,
        port: &str,
    ) -> Result<process::Child, anyhow::Error> {
        self.client = reqwest::Client::new();
        let chrome_exe: PathBuf;
        let chromedriver_exe: PathBuf;
        let (chrome_exe_name, chromedriver_exe_name) = dw_module.get_file_names();
        if !self.package_downloaded() {
            let (chrome_path, driver_path) = self.download_chromedriver().await?;
            chrome_exe = chrome_path.join(chrome_exe_name);
            chromedriver_exe = driver_path.join(chromedriver_exe_name);
        }
    }
}