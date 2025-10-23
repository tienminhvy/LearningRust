fn main() {
    has_same_digits(String::from("3902"));
    has_same_digits(String::from("34789"));
    has_same_digits(String::from("12345"));
}

fn has_same_digits(s: String) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    
    let mut i: usize = 0;
    let mut tmp_chars: Vec<char> = vec![];
    while chars.len() > 2 {

        let n: u32 = (to_digit(chars[i]) + to_digit(chars[i + 1])) % 10;
        let new_char: char = to_char(n);
        tmp_chars.push(new_char);

        i += 1;
        if i >= chars.len() - 1 {
            chars = tmp_chars.clone();
            tmp_chars = vec![];
            i = 0;
        }
    }

    let result = if chars[0] == chars[1] { true } else { false };
    println!("The result of input '{s}' is {result}");
    return result;
}

fn to_digit(ch: char) -> u32 {
    ch.to_digit(10).unwrap()
}

fn to_char(n: u32) -> char {
    char::from_digit(n, 10).unwrap()
}
