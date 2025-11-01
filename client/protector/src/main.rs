//protector client 

#![allow(unused_parens)] // do not warn about unused parentheses

mod settings;
use settings::settings as set;

//const SETTINGS_FILE: &str = "/etc/protector/conf/protector.conf";
const DEBUG: bool = true;
//only debug
const SETTINGS_FILE: &str = "protector.conf";

fn main() {
	println!("Starting Protector Client");
	println!("Loading settings from conf file");
	if(!set::load_settings(SETTINGS_FILE)){
		println!("error reading config file\nexiting...");
		std::process::exit(3);
	}
	if(DEBUG){	
		let b = set::get_settings_value_str("server_ip");
		let c = set::get_settings_value_int("server_port");
		let d = set::get_settings_value_str("server_secret");
		let e = set::get_settings_value_str("client_secret");
		println!("Server ip is {} and port is {}.\nAnd server_secret is {} and client_secret is {}",b,c,d,e);
	}
	println!("Finished, exiting...");
}
