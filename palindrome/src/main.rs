use std::io;

fn palindrome(s: &str) -> bool {
    let sanitize: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();

    sanitize == sanitize.chars().rev().collect::<String>()
}

fn main() {
    println!("Type a word");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();
    if palindrome(word){
        println!("{word} is a palindrome");
    }else{
    println!("Its not a Palindrome");
    }
}
