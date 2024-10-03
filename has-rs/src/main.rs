use std::env;
use std::process::Command;
use lazy_static::lazy_static;
use regex::Regex;
use which::which;

// tested with: cargo run tere ls node npm nope git java rustc rustup cargo
fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() > 1 {
		for app in args.iter().skip(1) {
			let v = get_version(app);
			if v.len() > 0 {
				println!("✔ {} {}", app, v);
			} else {
				println!("✘ {}", app);
			}
		}
	} else {
		println!("has-rs v0.1.1\n\
		---------------------------------------------\n\
		Written by Joachim de Fourestier (joedf)\n\
		Released under the MIT License\n\
		https://github.com/joedf/has-rs\n\
		\n\
		Based on https://github.com/kdabir/has");
	}
}

fn get_version(app: &str) -> String {
	// accounts for PATHEXT on MS Windows!
	let resolved_path = match which(app) {
		Ok(result) => result.into_os_string().into_string().unwrap(),
		Err(_) => String::new()
	};

	// println!("{}", resolved_path);

	if resolved_path.len() <= 0 {
		return String::new()
	}

	let mut command_exists = false;

	lazy_static! { static ref RE: Regex = Regex::new(r"(?i)(\d+\.|v){1,3}\d+").unwrap(); }

	for arg in ["--version", "-version", "-V", "-v", "version"] {
		let output = try_get_version(resolved_path.to_string(), arg);

		// println!("{}", output);

		if output.len() > 0 {
			let version = match RE.captures(&output) {
				Some(x) => x.get(0).unwrap().as_str().to_string().replace("v",""),
				None => String::new()
			};

			if version.len() > 0 {
				return version
			}

			command_exists = true;
		}
	}

	if command_exists {
		return "n/a".to_string()
	} else {
		return String::new()
	}
}

fn try_get_version(app: String, arg: &str) -> String {
	let output = match Command::new(app)
		.arg(arg)
		.output() {
			Ok(s) => format!("{} {}",
				String::from_utf8(s.stdout).unwrap(),
				String::from_utf8(s.stderr).unwrap()),
			Err(_) => return String::new()
	};

	return output
}