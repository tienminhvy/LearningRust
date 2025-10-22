fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_param(55555);
    another_function_with_params(5, "pounds");

    // statement and expression
    
    // Cannot do this
    // let x = (let y = 5);

    let x: i32 = {
        let y: i32 = 5;
        y + 1
    };
    println!("{x} is a number");

    let value: &str = function_with_returning_value();
    println!("{value}");

    let x = plus_one(5);
    println!("Value of x is {x}");
}

fn another_function() {
    println!("!dlrow, olleH");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is {x}");
}

fn another_function_with_params(x: i32, label: &str) {
    println!("The value of x is {x} {label}");
}

fn function_with_returning_value() -> &'static str {
    return "Lorem ipsum dolor sit amet...";
}

fn plus_one(x: i32) -> i32 {
    // this works
    x + 1
    // but this is not
    // x + 1;
}
