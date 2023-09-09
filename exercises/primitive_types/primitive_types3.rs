// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// Introduction to arrays.

fn main() {
    // let a = ["Just like arrays in C!", "10", ":-)"];
    let a = ["Srimanth was here."; 100]; // Notice the semicolon in the middle. It create a static array of size 100.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
