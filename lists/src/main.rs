fn main() {

    // arrays - fixed size group stored on the stack
    //          must contain the same type
    //          cannot be resized
    //          arrays of different size are technically different types
    let ints = [1, 2, 3]; // type can be inferred
    let floats: [f32; 3] = [1.0, 2.0, 3.0]; // or specified
    let zeros = [0; 3]; // initialize array with single value ([0, 0, 0])

    // let numbers = [ints, floats, zeros]; // Array members must be the same type
    let numbers = [ints, zeros];


    // Slices are dynamically sized array-like objects stored on the heap
    //   - their size is not known at compile time.
    //   - Like arrays, these cannot be resized
    //   - Can act as a "view" on arrays - fast RO access without memory copy

    let slice = &floats[0..1];

    // Vectors Vec<T> are growable lists of type <T>
    // Commonly used in Rust programs
    // Not as efficient as arrays but can change size

    let mut high_scores: Vec<(&str, i8)> = Vec::new();

    // push(): Append to back
    // pop(): Remove from back (item is returned for assignments)
    // insert(index: usize, element: T)
    // remove(index: usize): Remove the item at index and returns it
    high_scores.push(("Sam", 75));
    high_scores.push(("Max", 80));
    high_scores.push(("Doomguy", 100));
    high_scores.push(("Jon", 40));
    high_scores.push(("Tyrion", 101));

    // Vectors have a few methods to sort with
    // sort_by_key lets us run a function on T, and sorts by the result
    // In this case the function returns the second item from the score tuple
    high_scores.sort_by_key(|score| score.1);
    // Values are sorted low to high. The vector can be reversed too:
    high_scores.reverse();
    
    println!("High Scores!");
    for score in high_scores{
        println!("{}: {}", score.0, score.1);
    }

    
}
