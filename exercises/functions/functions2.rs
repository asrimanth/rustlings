// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// Loops in Rust using the for keyword.

fn main() {
    call_me(9);
}

fn call_me(num:i16) {
    for i in 0..num {
        println!("Srimanth was here. Ring! Call number {}", i + 1);
    }
}
