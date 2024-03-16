use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let get_file_result = File::open("x.txt");
    let mut file = match get_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("x.txt") {
                Ok(fc) => fc,
                Err(_) => panic!("Problem creating file !"),
            },
            _ => panic!("Can't handle this shit !"),
        },
    };
    println!("{:?}", file);
    let content = read_content_from_file(&mut file);
    println!("{}", content.expect("content error !"));
    println!("{}", get_last_char().unwrap());
}

fn read_content_from_file(file: &mut File) -> Result<String, Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_last_char() -> Option<char> {
    let mut text = String::new();
    File::open("x.txt")
        .expect("Error Openning File !")
        .read_to_string(&mut text)
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                panic!("Not found !");
            } else {
                panic!("Stop !");
            }
        });
    text.lines().last()?.chars().last()
}
