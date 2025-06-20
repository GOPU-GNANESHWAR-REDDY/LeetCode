pub fn int_to_roman(mut num: i32) -> String {
    let value_symbols = vec![
        (1000, "M"),
        (900,  "CM"),
        (500,  "D"),
        (400,  "CD"),
        (100,  "C"),
        (90,   "XC"),
        (50,   "L"),
        (40,   "XL"),
        (10,   "X"),
        (9,    "IX"),
        (5,    "V"),
        (4,    "IV"),
        (1,    "I"),
    ];

    let mut result = String::new();

    for &(value, symbol) in &value_symbols {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
}

fn main() {
    let input = 1994;
    let roman = int_to_roman(input);
    println!("Roman numeral of {} is {}", input, roman); // Output: MCMXCIV
}
