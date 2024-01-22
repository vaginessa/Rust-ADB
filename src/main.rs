// src/main.rs

use rust_adb::ADB; 

fn main() {
    let adb = ADB::new("/path/to/adb"); 
    match adb.refresh_device_list() {
        Ok(devices) => {
            println!("Connected devices:");
            for device in devices {
                println!("{}", device);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    
    match adb.get_screenshot_png("device_id_or_serial") {
        Ok(screenshot) => {
           
            println!("Screenshot received: {} bytes", screenshot.len());
        }
        Err(err) => eprintln!("Error getting screenshot: {}", err),
    }
}
