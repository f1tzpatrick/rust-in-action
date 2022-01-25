fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:16b} {}", a, a);
    println!("b: {:16b} {}", b, b);

    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
      std::mem::transmute(a)
    };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe {
      std::mem::transmute(frankentype)
    };
    println!("{}", b);
    assert_eq!(a, b);

    let mut i: u16 = 0;
    print!("{}..", i);

    // This loop causes i to overflow (and the program to panic)
    // loop {
    //     i += 1000;
    //     print!("{}..", i);
    //     if i % 10000 == 0 {
    //         print!{"\n"}
    //     }
    // }

    let zero: u16 = 0b0000_0000_0000_0000;
    let one:  u16 = 0b0000_0000_0000_0001;
    let two:  u16 = 0b0000_0000_0000_0010;
    // ...
    let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{}, {}, {}", sixtyfivethousand_533, sixtyfivethousand_534, sixtyfivethousand_535);
}