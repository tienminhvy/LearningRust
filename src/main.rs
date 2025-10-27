fn main() {
    println!("Hello, world!");
    matching();
    destructuring_tuple();
    destructuring_array();
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

fn destructuring_array() {
    // Try changing the values in the array, or make it a slice!
    let array: [i32; 3] = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
