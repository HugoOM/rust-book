use std::fs::File;
use std::{io, io::Read};

fn main() {
    // let f = match File::open("/home/hugo/rust_book/chapter_9/panic/hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let f = File::open("/home/hugo/rust_book/chapter_9/panic/hello.txt");
    println!("{:?}", f);

    let some_file = File::open("/home/hugo/rust_book/chapter_9/panic/hello.txt").unwrap();
    println!("{:?}", some_file);

    // File::open("file_that_does_not_exist.txt").unwrap(); // Calls OOTB panick
    // File::open("file_that_does_not_exist.txt").expect("Some Custom Error Message For Panic");
    println!("{:?}", read_file_content().unwrap());
    println!("{:?}", read_with_questionmark_operator().unwrap());

    test_func().unwrap();
}

fn read_file_content() -> Result<Vec<u8>, io::Error> {
    let path: &str = "/home/hugo/rust_book/chapter_9/panic/hello.txt";

    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content: Vec<u8> = Vec::new();

    match f.read_to_end(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

fn read_with_questionmark_operator() -> Result<Vec<u8>, io::Error> {
    let path: &str = "/home/hugo/rust_book/chapter_9/panic/hello.txt";
    let mut content: Vec<u8> = Vec::new();
    // let mut f = File::open(path)?;
    // f.read_to_end(&mut content)?;

    File::open(path)?.read_to_end(&mut content)?;
    Ok(content)
}

fn test_func() -> Result<File, io::Error> {
    let invalid_path: &str = "/home/hugo/rust_book/chapter_9//hello.txt";
    // The below works and will bubble up the error even when wrapped in an Ok() ... need to further investigate
    // Ok(File::open(invalid_path)?)
    File::open(invalid_path)
}
