use std::fs::File;
use std::io::ErrorKind;

pub fn demonstrate_recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("Error creating the file: {e:?}"),
            },
            other_error => panic!("Erorr opening the file : {other_error:?}"),
        },
    };

    //  Shortcuts for Panic on Error: unwrap and expect
    let file_with_unwrap = File::open("random.txt").unwrap();
    let file_with_expect =
        File::open("random.txt").expect("random.txt should be included in this project");
}
