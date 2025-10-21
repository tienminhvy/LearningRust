fn main() {
    println!("Hello, world!");

    // The if expression

    let number : i32 = 3;
    if number > 3 { // the condition must be a boolean type
        println!("yea")
    } else {
        println!("nay")
    }

    if number % 2 == 0 {
        println!("yub");
    } else if true { // conditional branching
        println!("yeb");
    } else {
        println!("nah")
    }

    // if in a let statement
    let me: &'static str = if number > 3 { "you" } else { "me" };
    println!("me is {me}");

    // This one will not work due to incompatible data types
    // let me = if number > 3 { 143 } else { "143" };

    // this is a loop
    loop {
        println!("looping through...");
        break
    }

    // returning the value from a loop
    let mut counter: i32 = 0;
    let result = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 8;
        }
    };
    println!("The result is: {result}");

    // labelling a loop
    let mut count: i32 = 0;
    'counting: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if (count == 2) { // can put the condition inside the parentheses pair
                break 'counting;
            }
            remaining -= 1;
        }

        count += 1;
    }

    // looping with the while loop

    let mut number = 10;
    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("--End loop");

    // looping with the for loop
    let a: [i32; 7] = [1, 0, 9, 0, 2, 0, 1];
    for element in a {
        println!("element: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
