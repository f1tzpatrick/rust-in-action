#! [allow(unused_variables)]

use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// We can write Functions to work with our File type
fn open(file: File) -> Result<File, String> {
    if one_in(20) {
        let error = String::from("Critical Failure!");
        return Err(error)
    }
    Ok(file)
}

fn close(file: File) -> Result<File, String> {
    if one_in(20) {
        let error = String::from("Critical Failure!");
        return Err(error)
    }
    Ok(file)
}

#[allow(dead_code)]
fn read(file: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = file.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

// Or we can implement methods for our File type
#[allow(dead_code)]
impl File {
    fn new(name: &str) -> File {
        File { name: String::from(name), data: Vec::new() }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
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
    };

    let mut f2 = File {
        name: String::from("f2.txt"),
        data: vec![114, 117, 115, 116, 33],
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


}
