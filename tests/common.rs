use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn valid_resource_content(dirs: &[&str], file: &str) -> String {
    resource_content(true, dirs, file)
}

pub fn valid_resource_path(dirs: &[&str], file: &str) -> String { resource_path(true, dirs, file) }

#[allow(dead_code)]
pub fn invalid_resource_content(dirs: &[&str], file: &str) -> String {
    resource_content(false, dirs, file)
}

#[allow(dead_code)]
pub fn invalid_resource_path(dirs: &[&str], file: &str) -> String {
    resource_path(false, dirs, file)
}

fn resource_content(valid: bool, subdirs: &[&str], file: &str) -> String {
    match File::open(resource_path(valid, subdirs, file)) {
        Ok(mut path) => {
            let mut content = String::new();
            match path.read_to_string(&mut content) {
                Ok(_) => content,
                Err(err) => panic!("Error while reading file contents: {}.", err)
            }
        }
        Err(err) => panic!("Error while opening file while reading resource contents: {}.", err)
    }
}

fn resource_path(valid: bool, subdirs: &[&str], file: &str) -> String {
    let mut source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("resources")
        .join(if valid { "valid" } else { "invalid" });

    for dir in subdirs {
        source_path = source_path.join(dir);
    }

    source_path = source_path.join(file);
    String::from(source_path.to_string_lossy())
}

pub fn check_valid_resource_exists_and_delete(subdirs: &[&str], file: &str) -> bool {
    remove(&valid_resource_path(subdirs, file))
}

#[allow(dead_code)]
pub fn check_invalid_resource_exists_and_delete(subdirs: &[&str], file: &str) -> bool {
    remove(&invalid_resource_path(subdirs, file))
}

fn remove(path_string: &String) -> bool {
    let path = Path::new(&path_string);
    if !path.exists() {
        return false;
    }

    match fs::remove_file(path) {
        Ok(_) => true,
        Err(err) => panic!("Error while removing file: {}.", err)
    }
}