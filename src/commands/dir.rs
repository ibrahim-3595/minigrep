use std::fs;

pub fn cmd_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cmd_ls() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        println!("{}/", path.file_name().unwrap().to_string_lossy());
                    } else {
                        println!("{}", path.file_name().unwrap().to_string_lossy());
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cmd_mkdir(name: &str) {
    match fs::create_dir(name) {
        Ok(_) => println!("Directory '{}' created.", name),
        Err(e) => println!("Failed to create directory '{}': {}", name, e),
    }
}
pub fn cmd_cd(path: &str) {
    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => println!("Failed to change directory to '{}': {}", path, e),
    }
}