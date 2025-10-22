fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_param(55555);
}

fn another_function() {
    println!("!dlrow, olleH");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is {x}");
}
