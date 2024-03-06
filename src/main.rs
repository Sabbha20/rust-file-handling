
use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};

fn main() {
    let mut file = File::create("src/myfile.txt").expect("create failed");
    file.write_all("Hi World!\nThis is Sabbha.".as_bytes()).expect("write failed");

    let mut file2 = OpenOptions::new().append(true)
        .open("src/example.txt")
        .expect("cannot open file");
    file2.write_all("Added content from code.\nNow the 2nd line.".as_bytes()).expect("Append failed");

    let mut file3 = File::open("src/example.txt").unwrap();
    let mut content = String::new();
    file3.read_to_string(&mut content).unwrap();
    println!("{}", content);

    remove_file("src/myfile.txt").expect("Delete failed");

}



