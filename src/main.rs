use std::collections::HashMap;
use std::time::Instant;

fn main() {
    //let s: String = String::from("III"); // 3
    //let s: String = String::from("LVIII"); // 58
    let s: String = String::from("MCMXCIV"); // 1994
                                             //let s: String = String::from("IV");
                                             //println!("{}", roman_to_int(s));
    let now = Instant::now();
    println!("{}", roman_to_int(s));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

// valid numerals
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
//
// valid groups
// IV           4
// IX           9
// XL           40
// XC           90
// CD           400
// CM           900
fn roman_to_int(s: String) -> i32 {
    let char_maps = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut output: i32 = 0;
    let rev_string = s.chars().rev().collect::<String>(); // reversed original string
    let mut index = 0;

    // while does not get to the end
    while index < rev_string.chars().count() {
        // get current character
        let current_char = rev_string.chars().nth(index).unwrap();

        // sum to the output
        output += char_maps.get(&current_char).unwrap();

        // to prevent seg fault
        if index + 1 < rev_string.chars().count() {
            // get next char
            let next_char = rev_string.chars().nth(index + 1).unwrap();
            // if the next character is smaller than the current one, it should be subtracted
            if char_maps.get(&next_char) < char_maps.get(&current_char) {
                output -= char_maps.get(&next_char).unwrap();
                index += 1;
            }
        }
        index += 1;
    }
    return output;
}
