use std::fs;
use std::sync::RwLock;
use once_cell::sync::Lazy;

pub struct Settings {
    pub server_ip: String,
    pub server_port: i64,
    pub client_secret: String,
    pub server_secret: String,
}

pub static SETTINGS: Lazy<RwLock<Settings>> = Lazy::new(|| {
    RwLock::new(Settings {
        server_ip: String::new(),
        server_port: 0,
	client_secret: String::new(),
	server_secret: String::new(),
    })
});

pub fn load_settings(path: &str) -> bool {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false, //file error
    };

    let mut settings = SETTINGS.write().unwrap();

    for line in contents.lines() {
        let line = line.trim();
        if (line.is_empty() || line.starts_with('#') || line.starts_with("//") || line.starts_with("[")) {
            continue; //skip comments
        }

        let (key, value) = match line.split_once(':') {
            Some(kv) => kv,
            None => return false, // malformed line, should be name: value
        };

        let key = key.trim();
        let value = value.trim();

        match key { //can be used with a lot more settings key&values
            "server_ip" => settings.server_ip = value.to_string(),
	    "client_secret" => settings.client_secret = value.to_string(),
	    "server_secret" => settings.server_secret = value.to_string(),
            "server_port" => {
                match value.parse::<i64>() {
                    Ok(port) => settings.server_port = port,
                    Err(_) => return false, // invalid port number
                }
            }
            _ => continue, //return false, // unknown key
        }
    }

    return true;
}

pub fn get_settings_value_int(name: &str) -> i64 {
	let settings = SETTINGS.read().unwrap();
	match name{
		"server_port" => return settings.server_port,
		_ => return 0,
	}
}

pub fn get_settings_value_str(name: &str) -> String {
	let settings = SETTINGS.read().unwrap();
	match name{
		"server_ip" => return settings.server_ip.clone(),
		"server_secret" => return settings.server_secret.clone(),
		"client_secret" => return settings.client_secret.clone(),
		_ => return "err_not_found".to_string(),
	}
}
