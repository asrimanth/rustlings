// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// Add missing function argument

fn main() {
    call_me(9);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Srimanth was here. Ring! Call number {}", i + 1);
    }
}
