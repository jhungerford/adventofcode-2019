use rayon::prelude::*;

/// Given a number, returns whether it has six digits.
fn is_six_digit_number(num: &u32) -> bool {
    num >= &100000 && num <= &999999
}

/// Given a minimum and maximum range value, returns a predicate that checks whether a number falls in the range.
fn make_is_in_range<'a>(min: &'a u32, max: &'a u32) -> impl Fn(&u32) -> bool + 'a {
    move |num| num >= min && num <= max
}

/// Given a number, returns whether two adjacent digits are the same.
fn is_adjacent_digit_same(num: &u32) -> bool {
    let num_str = num.to_string();
    let num_bytes = num_str.as_bytes();
    
    // Compare a digit to the previous for equality (skipping the first since the second digit compares it).
    // Only a single pair of digits needs to match.
    num_bytes.into_iter().enumerate().skip(1).any(|(i, c)| c == &num_bytes[i - 1])
}

// Given a number returns whether digits increase or stay the same going left to right
fn is_increasing(num: &u32) -> bool{
    let num_str = num.to_string();
    let num_bytes = num_str.as_bytes();
    
    // Compare a digit to the previous to ensure it's increasing (skipping the first since the second digit compares it).
    // All of the pairs of digits need to match
    num_bytes.into_iter().enumerate().skip(1).all(|(i, c)| c >= &num_bytes[i - 1])
}

/// Given a range, returns the number of different passwords that meet the criteria.
fn num_valid(min: u32, max: u32) -> u32 {
    let is_in_range = make_is_in_range(&min, &max);

    (min..max + 1).into_par_iter()
        .filter(|num| is_six_digit_number(num) 
                        && is_in_range(num)
                        && is_adjacent_digit_same(num)
                        && is_increasing(num))
        .count() as u32
}

#[cfg(test)]
mod test;

fn main() {
    println!("Part 1: {}", num_valid(147981, 691423))
}
