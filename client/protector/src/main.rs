//protector client 

#![allow(unused_parens)] // do not warn about unused parentheses

mod settings;
use settings::settings as set;
mod log;
use log::log as logger;

//const SETTINGS_FILE: &str = "/etc/protector/conf/protector.conf";
const DEBUG: bool = true;
//only debug
const SETTINGS_FILE: &str = "protector.conf";


//all stdout will be redirected into /var/log/protector.log if started as a service.
fn main() {
	logger::log("Starting Protector Client");
	logger::log("Loading settings from conf file");
	if(!set::load_settings(SETTINGS_FILE)){
		logger::log("error reading config file\nexiting...");
		std::process::exit(3);
	}
	if(DEBUG){	
		let b = set::get_settings_value_str("server_ip");
		let c = set::get_settings_value_int("server_port");
		logger::log(&format!("Server ip is {} and port is {}",b,c));
	}
	logger::log("Finished, exiting...");
}
