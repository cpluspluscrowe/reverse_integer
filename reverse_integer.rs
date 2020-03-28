fn main(){
    let number = 353;
    let answer = is_palindrome(number);
    println!("Is {0} a palindrome? Answer: {1}", number, answer);

    let number = 355;
    let answer = is_palindrome(number);
    println!("Is {0} a palindrome? Answer: {1}", number, answer);    
}

pub fn reverse(x: i32) -> i32 {
    let building = x.to_string().chars().rev().collect::<String>();
    return building.parse::<i32>().unwrap();
}

pub fn is_palindrome(x: i32) -> bool {
    let reversed = reverse(x);
    return x == reversed;
}
