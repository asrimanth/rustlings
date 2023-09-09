// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// Shadowing and variable scopes

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Srimanth was here. Spell a Number : {}", number);
    {
        let number = 3; // don't rename this variable
        println!("Srimanth was here. Number plus two is : {}", number + 2);
    }
}
