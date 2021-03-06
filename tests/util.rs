use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[macro_export]
macro_rules! assert_ok {
    ($expr:expr) => {{
        match $expr {
            Ok(_) => (),
            Err(err) => panic!("{}", err)
        }
    }};
}

pub fn resource_string_content(file: String) -> String {
    let mut content = String::new();
    let mut source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    source_path.push(if cfg!(windows) {
                         String::from("tests\\resources\\")
                     } else {
                         String::from("tests/resources/")
                     });
    source_path.push(file);

    match source_path.to_str() {
        Some(path) => match File::open(path) {
            Ok(mut file) => {
                file.read_to_string(&mut content).unwrap();
            }
            Err(error) => panic!("Error opening file {}: {}", path, error)
        },
        None => panic!("Error opening file: path can't be converted to string.")
    }

    return content;
}

pub fn valid_resource(file: &str) -> String {
    if cfg!(windows) {
        resource_string_content(format!("{}{}", "valid\\", file))
    } else {
        resource_string_content(format!("{}{}", "valid/", file))
    }
}
