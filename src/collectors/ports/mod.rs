use anyhow::Result;
use crate::types::PortInfo;

pub trait PortScanner {
    fn scan(&mut self) -> Result<Vec<PortInfo>>;
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

pub fn create_scanner() -> Box<dyn PortScanner> {
    #[cfg(target_os = "windows")]
    { Box::new(windows::WindowsPortScanner) }
    #[cfg(target_os = "linux")]
    { Box::new(linux::LinuxPortScanner) }
    #[cfg(target_os = "macos")]
    { Box::new(macos::MacosPortScanner) }
}
