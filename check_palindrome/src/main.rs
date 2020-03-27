fn main() {
    println!("{}", check_palindrome(String::from("aabbaa")));
}

fn check_palindrome(input: String) -> bool {
    let input = input.as_bytes();
    let len = input.len() / 2;
    for i in 0..len {
        if input[i] != input[input.len() - 1 - i] {
            return false;
        }
    }
    true
}
