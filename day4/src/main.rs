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

/// Given a number, returns whether two adjacent digits are the same.
/// Adjacent digits are considered in pairs - `122234` returns false since `222` isn't a pair of digits, but
/// `111145` returns true since `1111` is two pairs of ones.
fn is_adjacent_digit_same_pairs(num: &u32) -> bool {
    // Generate a Vec<num_in_a_row: u32>, where num_in_a_row represents the number of repeated digits.
    let mut num_iter = *num;
    let mut previous_i = num_iter % 10;
    num_iter /= 10;

    let mut num_i = 1;
    let mut nums_in_a_row = vec![];
    
    while num_iter > 0 {
        let i = num_iter % 10; // Rightmost digit - order doesn't matter.
        
        if i == previous_i {
            num_i += 1;
        } else {
            nums_in_a_row.push(num_i);
            previous_i = i;
            num_i = 1;
        }

        num_iter /= 10; // Discard the digit we examined
    }

    nums_in_a_row.push(num_i);

    // Check that there's at least one exact pair
    nums_in_a_row.into_iter().any(|num_in_a_row| num_in_a_row == 2)
}

/// Given a number returns whether digits increase or stay the same going left to right
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

/// Given a range, returns the number of different passwords that meet the criteria with strict matching rules.
fn strict_num_valid(min: u32, max: u32) -> u32 {
    let is_in_range = make_is_in_range(&min, &max);

    (min..max + 1).into_par_iter()
        .filter(|num| is_six_digit_number(num) 
                        && is_in_range(num)
                        && is_adjacent_digit_same(num)
                        && is_adjacent_digit_same_pairs(num)
                        && is_increasing(num))
        .count() as u32
}

#[cfg(test)]
mod test;

fn main() {
    println!("Part 1: {}", num_valid(147981, 691423));
    println!("Part 2: {}", strict_num_valid(147981, 691423));
}
