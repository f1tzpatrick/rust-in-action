use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap();

    // Create a buffered reader to avoid making excess direct file I/O accesses
    let reader = BufReader::new(f);

    // Files can be looped through line
    //   let mut line = String::new();
    //   loop {
    //       let len = reader.read_line(&mut line).unwrap();
    //     if len == 0 {
    //       break
    //     }
    //     println!("{} ({} bytes long)", line, len);
    //     line.truncate(0);
    //   }

    // or iterated over with BufReader lines()
    for line in reader.lines() {
        let line = line.unwrap(); // shadow assign line: Result<String> to line: String
        println!("{} ({} bytes long)", line, line.len());
    }
}
