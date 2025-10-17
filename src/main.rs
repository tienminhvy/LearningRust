fn main() {
    println!("Hello, world!");

    let inmutable: i32 = 5;
    println!("inmutable variable value: {inmutable}");

    // Cannot re-assign due to all variable are inmutable by default
    // inmutable = 7;

    let mut mutable: i32 = 8; // Need to mutate a variable? add 'mut' directive for it.
    println!("Before mutation: {mutable}");

    mutable = 11;
    println!("After mutation: {mutable}");

    // Defining constants

    const PI: f64 = 3.14;
    println!("The PI: {PI}");

    // Shadowing definition

    let x: i32 = 15;
    let x: i32 = x + 5;

    {
        let x: i32 = x * 20;
        println!("Block scoped shadowing variable: {x}");
    }

    println!("Shadowing variable: {}", x);

    // Another shadowing eg - shadowing is creating new variable with the same name but data type may different

    let spaces: &'static str = "  ";
    println!("spaces: {spaces}");
    
    let spaces: usize = spaces.len();
    println!("spaces: {spaces}");

    // Cannot do this
    /*
        let mut this: &'static str = "    ";
        this = this.len(); // error for mismatched type
    */

    // The Constants
    println!("I'm learning new language, it's called {LANGUAGE}");
    
    let n: i32 = 22;
    println!("The threshold is {THRESHOLD}");
    println!("{} is {}", n, if is_big(n) { "bigggg" } else { "smol" });

    // THRESHOLD = 5; // compile error due to constant cannot be modified
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 3;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}