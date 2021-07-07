fn main() {
    let a = 10;
    let b = 20;
    let result = add_with_lifetimes(&a, &b);
    println!("{} + {} = {}", a, b, result);

}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
