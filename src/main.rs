
use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    let mut file = File::create("src/myfile.txt").expect("create failed");
    file.write_all("Hi World!\nThis is Sabbha.".as_bytes()).expect("write failed");

    let mut file2 = OpenOptions::new().append(true)
        .open("src/example.txt")
        .expect("cannot open file");
    file2.write_all("Added content from code.\n".as_bytes()).expect("Append failed")
}



