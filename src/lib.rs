// src/lib.rs

use std::process::Command;
use std::io::{self, Read};

pub struct ADB {
    bin: String,
}

impl ADB {
    pub const BIN_LINUX: &'static str = "adb";
    pub const BIN_DARWIN: &'static str = "adb-darwin";
    pub const BIN_WINDOWS: &'static str = "adb.exe";

    pub fn new(bin_path: &str) -> Self {
        let bin = match std::env::consts::OS {
            "windows" => format!("{}\\{}", bin_path, Self::BIN_WINDOWS),
            "macos" => format!("{}/{}", bin_path, Self::BIN_DARWIN),
            _ => format!("{}/{}", bin_path, Self::BIN_LINUX),
        };
        ADB { bin }
    }

    fn exec_shell(&self, command: &str) -> Result<String, io::Error> {
        let output = Command::new(&self.bin)
            .arg(command)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)),
            ))
        }
    }

    pub fn run_adb(&self, command: &str) -> Result<String, io::Error> {
        self.exec_shell(command)
    }

    pub fn refresh_device_list(&self) -> Result<Vec<String>, io::Error> {
        let result = self.run_adb("devices -l")?;
        let devices: Vec<String> = result
            .lines()
            .skip(1) // Skip the header line
            .map(|line| line.trim().to_string())
            .collect();
        Ok(devices)
    }

    pub fn start_server(&self) -> Result<(), io::Error> {
        self.run_adb("start-server")?;
        Ok(())
    }

    pub fn kill_server(&self, force: bool) -> Result<(), io::Error> {
        if force {
            if std::env::consts::OS != "windows" {
                eprintln!("Force termination is not implemented on non-Windows systems, fallback to normal.");
            } else {
                self.exec_shell(&format!("taskkill /f /im {}", self.bin))?;
            }
        } else {
            self.run_adb("kill-server")?;
        }
        Ok(())
    }

   
   
    pub fn get_screenshot_png(&self, device: &str) -> Result<Vec<u8>, io::Error> {
        let output = self.run_adb(&format!("{} exec-out screencap -p", device))?;
        Ok(output.into_bytes())
    }
}
