use serde::{Deserialize, Serialize};
use std::process::Command;
use std::net::TcpStream;
use std::time::Duration;
use std::io::Write;
use chrono::Local;

// Global mock mode flag removed

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterCheckResult {
    pub connected: bool,
    pub printers: Vec<PrinterInfo>,
    pub default_printer: Option<String>,
    pub error: Option<String>,
}


/// Check for connected printers using system commands
#[tauri::command]
pub fn check_printer_connection(
    printer_ip: Option<String>,
    printer_port: Option<String>,
) -> PrinterCheckResult {

    #[cfg(target_os = "linux")]
    {
        check_printers_linux(printer_ip, printer_port)
    }
    
    #[cfg(target_os = "macos")]
    {
        check_printers_macos(printer_ip, printer_port)
    }
    
    #[cfg(target_os = "windows")]
    {
        check_printers_windows(printer_ip, printer_port)
    }
}

fn check_network_printer(ip: &str, port: &str) -> Option<PrinterInfo> {
    let address = format!("{}:{}", ip, port);
    // Try to connect with a short timeout (e.g., 2 seconds)
    match TcpStream::connect_timeout(
        &address.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap()),
        Duration::from_secs(2),
    ) {
        Ok(_) => Some(PrinterInfo {
            name: format!("Network Printer ({})", address),
            is_default: true,
            status: "Online".to_string(),
        }),
        Err(_) => None,
    }
}

#[cfg(target_os = "linux")]
fn check_printers_linux(ip: Option<String>, port: Option<String>) -> PrinterCheckResult {
    // Use lpstat to check for printers on Linux
    let output = Command::new("lpstat")
        .args(["-p", "-d"])
        .output();
    
    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stderr = String::from_utf8_lossy(&result.stderr);
            
            if !result.status.success() && stderr.contains("No destinations added") {
                // If network printer is configured, we still want to show it even if local lpstat fails or shows nothing
                let mut printers: Vec<PrinterInfo> = vec![];
                if let (Some(ref ip), Some(ref port)) = (&ip, &port) {
                    if !ip.is_empty() && !port.is_empty() {
                         if let Some(printer) = check_network_printer(ip, port) {
                            printers.push(printer);
                        }
                    }
                }

                if printers.is_empty() {
                    return PrinterCheckResult {
                        connected: false,
                        printers: vec![],
                        default_printer: None,
                        error: Some("ไม่พบเครื่องพิมพ์ที่เชื่อมต่อ".to_string()),
                    };
                } else {
                     return PrinterCheckResult {
                        connected: true,
                        printers,
                        default_printer: None,
                        error: None,
                    };
                }
            }
            
            let mut printers: Vec<PrinterInfo> = vec![];
            let mut default_printer: Option<String> = None;

            // Check for network printer if configured
            if let (Some(ref ip), Some(ref port)) = (&ip, &port) {
                if !ip.is_empty() && !port.is_empty() {
                    if let Some(printer) = check_network_printer(ip, port) {
                        printers.push(printer);
                    }
                }
            }
            
            for line in stdout.lines() {
                // Parse printer lines: "printer PrinterName is idle..."
                if line.starts_with("printer ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[1].to_string();
                        let status = if line.contains("idle") {
                            "พร้อมใช้งาน".to_string()
                        } else if line.contains("disabled") {
                            "ปิดใช้งาน".to_string()
                        } else {
                            "กำลังทำงาน".to_string()
                        };
                        
                        printers.push(PrinterInfo {
                            name: name.clone(),
                            is_default: false,
                            status,
                        });
                    }
                }
                // Parse default printer line: "system default destination: PrinterName"
                if line.contains("system default destination:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        default_printer = Some(parts[1].trim().to_string());
                    }
                }
            }
            
            // Mark default printer
            if let Some(ref default_name) = default_printer {
                for printer in &mut printers {
                    if &printer.name == default_name {
                        printer.is_default = true;
                    }
                }
            }
            
            PrinterCheckResult {
                connected: !printers.is_empty(),
                printers,
                default_printer,
                error: None,
            }
        }
        Err(e) => PrinterCheckResult {
            connected: false,
            printers: vec![],
            default_printer: None,
            error: Some(format!("ไม่สามารถตรวจสอบเครื่องพิมพ์ได้: {}", e)),
        },
    }
}

#[cfg(target_os = "macos")]
fn check_printers_macos(ip: Option<String>, port: Option<String>) -> PrinterCheckResult {
    // Use lpstat on macOS as well
    let output = Command::new("lpstat")
        .args(["-p", "-d"])
        .output();
    
    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            
            let mut printers: Vec<PrinterInfo> = vec![];
            let mut default_printer: Option<String> = None;

            // Check for network printer if configured
            if let (Some(ref ip), Some(ref port)) = (&ip, &port) {
                if !ip.is_empty() && !port.is_empty() {
                    if let Some(printer) = check_network_printer(ip, port) {
                        printers.push(printer);
                    }
                }
            }
            
            for line in stdout.lines() {
                if line.starts_with("printer ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[1].to_string();
                        printers.push(PrinterInfo {
                            name: name.clone(),
                            is_default: false,
                            status: "พร้อมใช้งาน".to_string(),
                        });
                    }
                }
                if line.contains("system default destination:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        default_printer = Some(parts[1].trim().to_string());
                    }
                }
            }
            
            if let Some(ref default_name) = default_printer {
                for printer in &mut printers {
                    if &printer.name == default_name {
                        printer.is_default = true;
                    }
                }
            }
            
            PrinterCheckResult {
                connected: !printers.is_empty(),
                printers,
                default_printer,
                error: None,
            }
        }
        Err(e) => PrinterCheckResult {
            connected: false,
            printers: vec![],
            default_printer: None,
            error: Some(format!("ไม่สามารถตรวจสอบเครื่องพิมพ์ได้: {}", e)),
        },
    }
}

#[cfg(target_os = "windows")]
fn check_printers_windows(ip: Option<String>, port: Option<String>) -> PrinterCheckResult {
    // Use wmic to list printers on Windows
    let output = Command::new("wmic")
        .args(["printer", "get", "name,default,status"])
        .output();
    
    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            
            let mut printers: Vec<PrinterInfo> = vec![];
            let mut default_printer: Option<String> = None;

            // Check for network printer if configured
            if let (Some(ref ip), Some(ref port)) = (&ip, &port) {
                if !ip.is_empty() && !port.is_empty() {
                    if let Some(printer) = check_network_printer(ip, port) {
                        printers.push(printer);
                    }
                }
            }
            
            for line in stdout.lines().skip(1) {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                // Parse Windows WMI output
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let is_default = parts[0] == "TRUE";
                    let name = parts[1..parts.len()-1].join(" ");
                    
                    if is_default {
                        default_printer = Some(name.clone());
                    }
                    
                    printers.push(PrinterInfo {
                        name,
                        is_default,
                        status: "พร้อมใช้งาน".to_string(),
                    });
                }
            }
            
            PrinterCheckResult {
                connected: !printers.is_empty(),
                printers,
                default_printer,
                error: None,
            }
        }
        Err(e) => PrinterCheckResult {
            connected: false,
            printers: vec![],
            default_printer: None,
            error: Some(format!("ไม่สามารถตรวจสอบเครื่องพิมพ์ได้: {}", e)),
        },
    }
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPrintResult {
    pub success: bool,
    pub message: String,
}

/// Send a test print to the default printer or network printer
#[tauri::command]
pub fn test_print(
    printer_ip: Option<String>,
    printer_port: Option<String>,
) -> TestPrintResult {
    // Prioritize network printer if configured
    if let (Some(ip), Some(port)) = (printer_ip, printer_port) {
        if !ip.is_empty() && !port.is_empty() {
            return test_print_network(&ip, &port);
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        test_print_unix()
    }
    
    #[cfg(target_os = "windows")]
    {
        test_print_windows()
    }
}

fn test_print_network(ip: &str, port: &str) -> TestPrintResult {
    let address = format!("{}:{}", ip, port);
    
    // Connect to printer
    let mut stream = match TcpStream::connect(&address) {
        Ok(s) => s,
        Err(e) => return TestPrintResult {
            success: false,
            message: format!("ไม่สามารถเชื่อมต่อเครื่องพิมพ์ {}: {}", address, e),
        },
    };

    // ESC/POS commands
    // ESC @ = Initialize
    // ESC a 1 = Center align
    // ESC a 0 = Left align
    // GS V 66 0 = Cut paper
    
    let now = Local::now();
    let date_str = now.format("%d/%m/%Y").to_string();
    let time_str = now.format("%H:%M:%S").to_string();

    let content = format!(
        "\x1B@\x1Ba\x01\
        ================================\n\
        POT CAFE\n\
        Test Print Receipt\n\
        ================================\n\n\
        Printer Connection Test\n\
        Status: SUCCESSFUL\n\n\
        Date: {}\n\
        Time: {}\n\n\
        ================================\n\
        --- Test Complete ---\n\
        ================================\n\n\n\n\
        \x1D\x56\x42\x00", // Cut paper
        date_str, time_str
    );

    match stream.write_all(content.as_bytes()) {
        Ok(_) => TestPrintResult {
            success: true,
            message: format!("ส่งข้อมูลไปยัง {} สำเร็จ", address),
        },
        Err(e) => TestPrintResult {
            success: false,
            message: format!("ส่งข้อมูลล้มเหลว: {}", e),
        },
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_print_unix() -> TestPrintResult {
    // Get current date and time
    let now = Local::now();
    let date_str = now.format("%d/%m/%Y").to_string();
    let time_str = now.format("%H:%M:%S").to_string();
    
    // Create a simple test receipt text
    let test_content = format!(r#"
================================
        ☕ POT CAFE
      Test Print Receipt
================================

Printer Connection Test
Status: SUCCESSFUL

Date: {}
Time: {}

================================
      --- Test Complete ---
================================
"#, date_str, time_str);

    // Use lp command to print on Unix systems
    let output = Command::new("echo")
        .arg(&test_content)
        .output();
    
    match output {
        Ok(_) => {
            // Try to send to lp
            let print_result = Command::new("sh")
                .args(["-c", &format!("echo '{}' | lp", test_content)])
                .output();
            
            match print_result {
                Ok(result) => {
                    if result.status.success() {
                        TestPrintResult {
                            success: true,
                            message: "ส่งงานพิมพ์ทดสอบสำเร็จ".to_string(),
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&result.stderr);
                        TestPrintResult {
                            success: false,
                            message: format!("ไม่สามารถพิมพ์ได้: {}", stderr),
                        }
                    }
                }
                Err(e) => TestPrintResult {
                    success: false,
                    message: format!("ไม่สามารถพิมพ์ได้: {}", e),
                },
            }
        }
        Err(e) => TestPrintResult {
            success: false,
            message: format!("ไม่สามารถสร้างเอกสารทดสอบได้: {}", e),
        },
    }
}

#[cfg(target_os = "windows")]
fn test_print_windows() -> TestPrintResult {
    // On Windows, we'll create a temp file and print it
    use std::fs::File;
    use std::io::Write;
    use std::env;
    
    let test_content = r#"
================================
        POT CAFE
      Test Print Receipt
================================

Printer Connection Test
Status: SUCCESSFUL

================================
      --- Test Complete ---
================================
"#;

    let temp_dir = env::temp_dir();
    let temp_file = temp_dir.join("pot_cafe_test_print.txt");
    
    match File::create(&temp_file) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(test_content.as_bytes()) {
                return TestPrintResult {
                    success: false,
                    message: format!("ไม่สามารถสร้างไฟล์ทดสอบได้: {}", e),
                };
            }
            
            // Use notepad /p to print on Windows
            let print_result = Command::new("notepad")
                .args(["/p", temp_file.to_str().unwrap()])
                .output();
            
            match print_result {
                Ok(result) => {
                    if result.status.success() {
                        TestPrintResult {
                            success: true,
                            message: "ส่งงานพิมพ์ทดสอบสำเร็จ".to_string(),
                        }
                    } else {
                        TestPrintResult {
                            success: false,
                            message: "ไม่สามารถพิมพ์ได้".to_string(),
                        }
                    }
                }
                Err(e) => TestPrintResult {
                    success: false,
                    message: format!("ไม่สามารถพิมพ์ได้: {}", e),
                },
            }
        }
        Err(e) => TestPrintResult {
            success: false,
            message: format!("ไม่สามารถสร้างไฟล์ทดสอบได้: {}", e),
        },
    }
}

