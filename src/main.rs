fn main() {
    println!("Hello, world!");

    let inmutable: i32 = 5;
    println!("inmutable variable value: {}", inmutable);

    // Cannot re-assign due to all variable are inmutable by default
    // inmutable = 7;

    let mut mutable: i32 = 8; // Need to mutate a variable? add 'mut' directive for it.
    println!("Before mutation: {}", mutable);

    mutable = 11;
    println!("After mutation: {}", mutable);
}
