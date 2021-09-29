#! [allow(unused_variables)]

use std::fmt;
use std::fmt::Display;

use rand::prelude::*;


// Three-slash comments can be rendered as documentation!
/// This type is kind of like an actual file
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

// We can write Functions to work with our File type
/// "Opens" a file, by rolling a d20 and returning an error for critical failures
fn open(file: File) -> Result<File, String> {
    if one_in(20) {
        let error = String::from("Critical Failure!");
        return Err(error)
    }
    Ok(file)
}

/// "Closes" a file, by rolling a d20 and returning an error for critical failures
fn close(file: File) -> Result<File, String> {
    if one_in(20) {
        let error = String::from("Critical Failure!");
        return Err(error)
    }
    Ok(file)
}

/// Reads bytes and appends them to a buffer (Vec<u8>)
/// And returns the number of bytes read
fn read(file: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = file.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

// Or we can implement methods for our File type
impl File {
    /// File::new()
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// blah blah blah
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }

    /// length
    fn len(&self) -> usize {
        self.data.len()
    }

    /// name
    fn name(&self) -> String {
        self.name.clone()
    }

}
// Traits can also be implemented for a struct. (Display is a common trait)
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

// Enums provide a way to represent multiple known variants
// eg Status, Conditions, Event Types
#[derive(Debug)]
enum FileState {
    Open,
    Closed
}
// Traits can also be implemented for enums
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

// Traits btw are like an interface - a set of functions
// which must be implemented for a struct in order to say
// the struct implements the trait.

/// Read trait. One function called trait_read that does exactly what the
/// other read functions in this file do but is named differently so you
/// can tell where the trait is being used in main()
trait Read{
    fn trait_read(
        self: &Self,
        save_to: &mut Vec<u8>
    ) -> Result<usize, String>;
}

impl Read for File {
    fn trait_read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
        state: FileState::Closed
    };

    let mut f2 = File {
        name: String::from("f2.txt"),
        data: vec![114, 117, 115, 116, 33],
        state: FileState::Closed
    };

    let mut buffer: Vec<u8> = vec![];

    f2 = open(f2).unwrap();
    let filesize = read(&f2, &mut buffer);
    f2 = close(f2).unwrap();

    let filename = &f2.name;
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", filename, filesize);
    println!("{}", text);

    let f3 = File::new("f3.txt");
    let filename = &f3.name;
    let filesize = f3.data.len();
    println!("{:?}", f3);
    println!("{} is {} bytes long", filename, filesize);

    let f4_data: Vec<u8> = vec![114, 117, 115, 116 , 33];
    let mut f4 = File::new_with_data("f4.txt", &f4_data);

    f4 = open(f4).unwrap();
    let mut buffer: Vec<u8> = vec![];
    let filesize = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();
    let filename = &f4.name;
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", filename, filesize);
    println!("{}", text);

    let mut buffer: Vec<u8> = vec![];
    let mut f5 = File::new_with_data("f5.txt", &f4_data);
    let filesize = f5.trait_read(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&buffer);
    println!("{} is {} bytes long: {}", f5.name(), f5.len(), text);

}

// RNG Function
fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}
