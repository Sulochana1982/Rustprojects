

//https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
//https://www.programiz.com/rust/file-handling
use std::fs;

fn main() {
    let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
    println!("{}", data);
}



use std::fs;

fn main() {
    let data = fs::read("/etc/hosts").expect("Unable to read file");
    println!("{}", data.len());
}




use std::fs;

fn main() {
    let data = "Some data!";
    fs::write("/tmp/foo", data).expect("Unable to write file");
}




use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut f = File::open("/etc/hosts").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
}




use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = Vec::new();
    let mut f = File::open("/etc/hosts").expect("Unable to open file");
    f.read_to_end(&mut data).expect("Unable to read data");
    println!("{}", data.len());
}




use std::fs::File;
use std::io::Write;

fn main() {
    let data = "Some data!";
    let mut f = File::create("/tmp/foo").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}




use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut data = String::new();
    let f = File::open("/etc/hosts").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
}



use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let data = "Some data!";
    let f = File::create("/tmp/foo").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}




use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("/etc/hosts").expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("Line: {}", line);
    }
}



use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let data = "Some data!\n";
    let mut f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("/tmp/foo")
        .expect("Unable to open file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}



use std::io::{BufWriter, Write};
use std::fs::OpenOptions;

fn main() {
    let data = "Some data!\n";
    let f = OpenOptions::new()
        .append(true)
        .open("/tmp/foo")
        .expect("Unable to open file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}