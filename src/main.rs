fn main() {
    println!("Hello, world!");
    matching();
    destructuring_tuple();
}

fn matching() {
    let number: i32 = 99;

    println!("Current number is {}", number);

    match number {
        1 => {
            println!("It's one!");
        },
        3 | 5 | 7 | 9 | 11 => println!("A prime value"),
        13..19 => println!("In range 13..19, not including 19"),
        23..=29 => println!("In range 23..29"),
        _ => println!("The default case") // cannot be missed
    };

    let boolean: bool = false;

    let binary: i32 = match boolean {
        true => 1,
        false => 0
    };

    println!("{} -> {}", boolean, binary);
}

fn destructuring_tuple() {
    let triple = (1, 1, -2);
    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}