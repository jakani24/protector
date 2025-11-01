use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;

fn main() {
    // Predefined values
    let app_path = "/etc/protector/app/protector";  
    let service_name = "protector";         
    let working_dir = "/etc/protector";     
    let log_file = "/var/log/protector.log";

    // Step 1: Create folder structure
    let folders = ["/etc/protector/app", "/etc/protector/conf"];
    for folder in folders.iter() {
        if !Path::new(folder).exists() {
            match fs::create_dir_all(folder) {
                Ok(_) => { 
			println!("Created folder: {}", folder);
			if let Err(e) = fs::set_permissions(folder, fs::Permissions::from_mode(0o700)) {
                	        eprintln!("Failed to set permissions for {}: {}", folder, e);
                	} else {
                		println!("Set permissions 700 for {}", folder);
                	}                
		}
		Err(e) => eprintln!("Failed to create folder {}: {}", folder, e),
            }
        }
    }

    // Step 2: Create empty log file if it doesn't exist
    if !Path::new(log_file).exists() {
        match File::create(log_file) {
            Ok(_) => println!("Created log file: {}", log_file),
            Err(e) => eprintln!("Failed to create log file {}: {}", log_file, e),
        }
    }

    // Step 3: Create systemd service content
    let service_content = format!(
        r#"[Unit]
Description={0} system-wide service
After=network.target

[Service]
Type=simple
ExecStart={1}
Restart=on-failure
WorkingDirectory={2}
StandardOutput=append:{3}
StandardError=append:{3}

[Install]
WantedBy=multi-user.target
"#,
        service_name, app_path, working_dir, log_file
    );

    let service_path = format!("/etc/systemd/system/{}.service", service_name);

    // Step 4: Write the service file
    if Path::new("/etc/systemd/system/").exists() {
        match File::create(&service_path) {
            Ok(mut file) => {
                file.write_all(service_content.as_bytes())
                    .expect("Failed to write service file");
                println!("System-wide service file created at {}", service_path);

                // Step 5: Reload systemd, enable, and start the service
                let reload = Command::new("systemctl")
                    .arg("daemon-reload")
                    .status()
                    .expect("Failed to reload systemd");
                if reload.success() {
                    println!("systemd reloaded successfully.");
                }

                let enable = Command::new("systemctl")
                    .arg("enable")
                    .arg(service_name)
                    .status()
                    .expect("Failed to enable service");
                if enable.success() {
                    println!("Service {} enabled to start on boot.", service_name);
                }

                let start = Command::new("systemctl")
                    .arg("start")
                    .arg(service_name)
                    .status()
                    .expect("Failed to start service");
                if start.success() {
                    println!("Service {} started successfully.", service_name);
                } else {
                    eprintln!("Failed to start service {}.", service_name);
                }
            }
            Err(e) => eprintln!("Failed to create service file: {}", e),
        }
    } else {
        println!("Directory /etc/systemd/system/ does not exist. Cannot install");
    }
}
