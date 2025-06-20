use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        //Map Roman numerals to integers values
        let roman_map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ].iter().cloned().collect();
        
        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        let mut i = 0;

        while i < chars.len() {
            let current = roman_map[&chars[i]];

            //Check if there's a next character, and if subtractive notation applies
            if i +1 < chars.len() {
                let next = roman_map[&chars[i + 1]];

                if current < next {
                    total += next - current;
                    i +=2; //skip the next characyer
                    continue;
                }
            }

            //Otherwise, just add the current value
            total += current;
            i +=1;
        }
        total
    }
}

fn main() {
let roman = "MCMXCIV".to_string();
println!("Integer: {}", roman_to_int(roman_to_int(roman));
}