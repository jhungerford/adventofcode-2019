use super::*;

#[test]
fn test_is_six_digit_number() {
    assert_eq!(true, is_six_digit_number(&123456));
    assert_eq!(false, is_six_digit_number(&12345));
    assert_eq!(false, is_six_digit_number(&1234567));
}

#[test]
fn test_is_in_range() {
    let five_to_ten = make_is_in_range(&5, &10);

    assert_eq!(true, five_to_ten(&5));
    assert_eq!(true, five_to_ten(&7));
    assert_eq!(true, five_to_ten(&10));

    assert_eq!(false, five_to_ten(&4));
    assert_eq!(false, five_to_ten(&11));
}

#[test]
fn test_is_adjacent_digit_same() {
    assert_eq!(true, is_adjacent_digit_same(&111111));
    assert_eq!(true, is_adjacent_digit_same(&112345));
    assert_eq!(true, is_adjacent_digit_same(&123455));
    assert_eq!(true, is_adjacent_digit_same(&111211));
    assert_eq!(true, is_adjacent_digit_same(&122345));
    assert_eq!(false, is_adjacent_digit_same(&121212));
    assert_eq!(false, is_adjacent_digit_same(&12345));
}

#[test]
fn test_is_increasing() {
    assert_eq!(true, is_increasing(&111111));
    assert_eq!(true, is_increasing(&12345));
    assert_eq!(true, is_increasing(&111223));
    assert_eq!(false, is_increasing(&111101));
    assert_eq!(false, is_increasing(&123245));
}

#[test]
fn test_num_valid() {
    assert_eq!(650, num_valid(100000, 123456));
}