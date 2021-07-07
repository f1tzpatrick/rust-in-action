use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

fn discount(day_of_month: u8) -> f32 {
    // variables can be assigned to conditionals
    let amount = if day_of_month % 2 == 0 { 0.50 } else { 0.00 };

    // or a match
    let amount = match day_of_month {
        01 | 15 => 0.50,
        _ => 0.0,
    };
    println!("Your discount is {}%", amount);
    // expression value returned without using `return`
    amount
}

fn main() {
    discount(10);

    // loop forever with `loop`
    loop {
        println!("infinite loop!\n break");
        break;
    }

    // loop on condition with `while`
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What's the password?");
        word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
    }
    println!("You got it!");

    // for with range iterator (end inclusive)
    for i in 0..=10 {
        println!("i = {}", i);
    }

    // Read a string and convert it to a u8
    println!("Guess a number:");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: u8 = x.trim().parse().unwrap();
    // match on that u8
    match x {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4..=10 => println!("Somewhere between 4 and 11"), // exclusive range not supported here
        25 | 50 | 75 | 100 => println!("quarters!"),      // "or" values
        _ => println!("Something!"),
    }

    // match with pattern matching

    let dice1 = dice_roll(6);
    let dice2 = dice_roll(6);

    println!("Rolled {} and {}.", dice1, dice2);
    match (dice1, dice2) {
        (1, 1) => println!("Snake Eyes! roll again."),
        (5, _) | (_, 5) => {
            // matches can run more than one line
            println!("You rolled a 5!");
            println!("Drink!");
        }
        _ => println!("Move forward {} places", { dice1 + dice2 }),
    }

    // While, and some examples of std::time
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

    // Breaking from nested loops
    // label a loop with '<label>: loop...
    'outer: for x in 1..10 {
        for y in 0..100 {
            for z in 0..1000 {
                if x * y * z > 10_000 {
                    break 'outer;
                }
            }
        }
    }

    println!("Done!")
}

fn dice_roll(num_sides: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=num_sides)
}
