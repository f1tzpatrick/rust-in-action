use std::ops::{Add};
use std::time::{Duration};

// note: this function will not compile, since we use the '+' operator 
// with two generic type variables. It is not possible to use this function
// with types that do not implement the trait std::ops::Add
// 
// fn add<T>(i: T, j: T) -> T {
//     i + j
// }

// We can add a trait bound on T, saying that T must implement Add and the output
// of that operation is type T
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
  i + j
}

fn main() {
  let floats = add(1.2, 3.4);
  let ints = add(10, 20);
  let durations = add(Duration::new(5, 0), Duration::new(10, 0));

  println!("{}", floats);
  println!("{}", ints);
  println!("{:?}", durations);

}