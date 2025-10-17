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

    // Data types
    data_types();
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 3;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn data_types() {
    // let guess_me = "99".parse().expect("Not a Number"); // will not work due to explicit type definition is required for this case
    let guess_me : i32 = "99".parse().expect("Not a Number"); // worked
    println!("I guess {guess_me}");

    // i for integer, u for unsigned integer
    // e.g. i32 - u32
    // isize - usize => architecture dependent

    // Integer literals
    let literal_1: i32 = 12_345;
    let literal_2: i32 = 0xff;
    let literal_3: i32 = 0o77;
    let literal_4: i32 = 0b1111_0000;
    let literal_5: u8 = b'A';

    println!("Literals: {} {} {} {} {}", literal_1, literal_2, literal_3, literal_4, literal_5);

    // Floating-points types
    // f32 or f64, all are signed

    let n_1: f64 = 32.05;
    let n_2: f32 = 32.05;

    println!("Floats: {n_1} {n_2}");

    // Numeric Operations

    let sum: i32 = 5 + 10;
    let difference: f64 = 192.35 - 912.16;
    let product: i32 = 52 * 3;
    let quotient: f64 = 56.7 / 23.2;
    let truncated: i32 = -5 / 3;

    let remainder: i32 = 93 % 4;

    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");

    // The boolean type

    let me = true;
    let him: bool = false;

    println!("he is {him}, I am {me}");

    // Character type

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");

    // Compound types

    let tup: (i32, f64, &'static str, char) = (15, 23.2, "test", 'n');
    let (x, y, z, k) = tup;
    
    println!("{x}, {y}, {z}, {k}");
    println!("{tup:?}");
}