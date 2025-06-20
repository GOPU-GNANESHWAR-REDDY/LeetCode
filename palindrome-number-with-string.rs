impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //Negativenumber can't be palindromes
       if x < 0 {
        return false;
       } 
       let s = x.to_string(); //Convert number to string
       let reversed: String = s.chars().rev().collect(); //Reverse the string

       s == reversed //Compare origina and reversed
    }
}

/*fn main() {
    let test1 = 121;
    let test2 = -121;
    let test3 = 10;

    println!("{}", is_palindrome(test1)); // true
    println!("{}", is_palindrome(test2)); // false
    println!("{}", is_palindrome(test3)); // false
}
*/