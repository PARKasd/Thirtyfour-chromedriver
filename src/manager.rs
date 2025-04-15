use std::{fs, process};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use platform_dirs::AppDirs;
use regex::Regex;
use zip_extensions::zip_extract;
use thirtyfour::{ChromeCapabilities, ChromiumLikeCapabilities};

pub fn get_cache_dir() -> PathBuf {
    let cache_dir = get_cached_dir();

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).unwrap();
    }

    cache_dir
}
pub fn get_cached_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some("TFCD"), true).unwrap();
    app_dirs.cache_dir
}



pub fn get_file_names() -> (String, String) {

    let chrome_exe: String;
    let chromedriver_exe: String;

    if cfg!(target_os = "windows") {
        chrome_exe = "C:/Program Files/Google/Chrome/Application/chrome.exe".to_string();
        chromedriver_exe = "chromedriver.exe".to_string();
    } else if cfg!(target_os = "macos") {
        chrome_exe = "/Applications/Google\\ Chrome.app/Contents/MacOS/Google\\ Chrome".to_string();
        chromedriver_exe = "chromedriver".to_string();
    } else {
        chrome_exe = "chrome".to_string();
        chromedriver_exe = "chromedriver".to_string();
    }

    (chrome_exe, chromedriver_exe)
}

pub async fn create_progressbar(length: u64, msg: String) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-")
    );
    pb.set_message(msg);

    pb
}

pub async fn write_file(
    mut file: &File,
    mut response: reqwest::Response,
    msg: String,
) -> anyhow::Result<()> {
    let file_size = response.content_length().unwrap_or(0);
    let progress_bar = create_progressbar(file_size, msg).await;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;

        let increment = chunk.len() as u64; // Convert to MB
        progress_bar.inc(increment);
    }

    file.flush()?;

    Ok(())
}

pub async fn get_dw_link(chrome_version:String) -> String {
    let dw_link: String;

    if cfg!(target_os = "windows") {
        if cfg!(target_pointer_width = "64") {
            dw_link = format!("https://storage.googleapis.com/chrome-for-testing-public/{}{}", chrome_version, "/win64/chromedriver-win64.zip").to_string();
        }
        else{
            dw_link = format!("https://storage.googleapis.com/chrome-for-testing-public/{}{}", chrome_version, "/win32/chromedriver-win32.zip").to_string();
        }

    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            dw_link = format!("https://storage.googleapis.com/chrome-for-testing-public/{}{}", chrome_version, "/mac-x64/chromedriver-mac-x64.zip").to_string();
        }
        else {
            dw_link = format!("https://storage.googleapis.com/chrome-for-testing-public/{}{}", chrome_version, "/mac-arm64/chromedriver-mac-arm64.zip").to_string();
        }
    } else{
        dw_link = format!("https://storage.googleapis.com/chrome-for-testing-public/{}{}", chrome_version, "/linux64/chromedriver-linux64.zip").to_string();
    }
    dw_link
}

pub async fn get_version_info() -> String {
    let version_info: String;
    let (chrome_exe, chromedriver_exe) = get_file_names();
    if cfg!(target_os = "windows") {
        let ps_command = format!(
            "(Get-Item '{}').VersionInfo.ProductVersion",
            chrome_exe
        );
        let output = Command::new("powershell")
            .args(["-Command", &ps_command])
            .output()
            .expect("PowerShell execution failed");
        version_info = String::from_utf8_lossy(&output.stdout).trim().to_string();
    } else if cfg!(target_os = "macos") {
        let terminal_command = "--version".to_string();
        let output = Command::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
            .args(["-Command", &terminal_command])
            .output()
            .expect("Google Chrome not installed");
        version_info = String::from_utf8_lossy(&output.stdout).trim().to_string().split(" ").nth(2).unwrap().to_string();
    } else {
        let output = Command::new("google-chrome --version").output().expect("Google chrome --version failed");
        version_info = String::from_utf8_lossy(&output.stdout).trim().to_string().split(" ").nth(2).unwrap().to_string();
    }
    let re = Regex::new(r"^([^.]+\.)+[^.]+$").unwrap();

    if re.is_match(&version_info) {
        version_info
    } else {
        panic!("Google Chrome not installed or version format incorrect");
    }

}
pub fn dw_name() -> String {
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
    dw_name
}
pub async fn download_chromedriver(client: &reqwest::Client, dw_link: String) -> anyhow::Result<()> {

    let version_info: String;
    let (chrome_exe, chromedriver_exe) = get_file_names();
    

    let driver_path = get_cache_dir().join(PathBuf::from(dw_name()).with_extension("zip"));
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
pub struct Handler {
    client: reqwest::Client,
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Handler {

    pub fn new() -> Self {
        Self::default()
    }
    fn package_downloaded(&self) -> bool {
        let (chrome_path, driver_path) = get_file_names();

        if Path::new(&driver_path).exists() {
            return true;
        }
        false
    }


    pub async fn launch_chromedriver(
        &mut self,
        capabilities: &mut ChromeCapabilities,
        port: &str,
    ) -> Result<process::Child, anyhow::Error> {
        self.client = reqwest::Client::new();

        let chrome_exe: PathBuf;
        let chromedriver_exe: PathBuf;

        let (chrome_exe_name, chromedriver_exe_name) = get_file_names();

        if !self.package_downloaded() {
            let dw_links = get_dw_link(get_version_info().await).await;
            download_chromedriver(&self.client, dw_links).await.expect("Failed to Download Chromedriver!");
        }
        
        let (default_chrome_path, default_driver_path) = get_file_names();
        chrome_exe = default_chrome_path.into();
        chromedriver_exe = PathBuf::from(get_cache_dir()).join(dw_name()).join(chromedriver_exe_name);
        println!("{}",chromedriver_exe.to_string_lossy());
        
        capabilities.set_binary(chrome_exe.to_str().unwrap())?;

        let chromedriver_exe = chromedriver_exe.to_str().unwrap();

        let mut command = Command::new(chromedriver_exe);
        let mut command = command
            .arg(format!("--port={}", port));
        Ok(command.spawn()?)
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use thirtyfour::prelude::*;
    use crate::manager::Handler;

    #[tokio::test]
    async fn test_launch_chromedriver() -> anyhow::Result<()>{
        let mut caps = DesiredCapabilities::chrome();

        caps.set_headless()?;
        caps.set_no_sandbox()?;
        caps.set_disable_gpu()?;
        let mut chromedriver = Handler::new()
            .launch_chromedriver(&mut caps, "3000")
            .await?;

        println!("Launched Chromedriver");

        let driver = WebDriver::new("http://localhost:3000", caps).await?;
        driver.goto("https://www.gimkit.com/join").await?;

        thread::sleep(Duration::from_secs(10));

        driver.quit().await?;
        chromedriver.kill()?;

        Ok(())
    }
}