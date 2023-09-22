// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// Either 1 mutable reference, or multiple non-mutable references.
// References must be valid. These steps are to avoid race condition.

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // y's reference ends here.
    let z = &mut x;
    *z += 1000;
    // If a value is added to y and z, it is added to x.
    assert_eq!(x, 1200);
}
