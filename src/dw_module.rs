use std::{env::consts, fs, fs::File, io::Write};
use std::any::{type_name, type_name_of_val};
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::anyhow;
use platform_dirs::AppDirs;
use indicatif::{ProgressBar, ProgressStyle};
use zip_extensions::zip_extract;
use regex::Regex;
use log::error;
use tokio::runtime::Handle;

pub struct Handler {
    client: reqwest::Client,
    platform: String,
}
fn get_cached_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some("chromedriver-manager"), true).unwrap();
    app_dirs.cache_dir
}
pub struct DriverDownload {
    pub platform: String,
    pub url: String,
}
fn get_file_names() -> (String, String) {

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
impl Handler {

    
    pub fn get_cache_dir() -> PathBuf {
        let cache_dir = get_cached_dir();

        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).unwrap();
        }

        cache_dir
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
        let progress_bar = Self::create_progressbar(file_size, msg).await;

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
    fn package_downloaded(&self) -> bool {
        let (chrome_path, driver_path) = get_file_names();

        if Path::new(&driver_path).exists() {
            return true;
        }
        false
    }
}