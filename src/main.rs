fn main() {
    println!("Hello, world!");
    matching()
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
