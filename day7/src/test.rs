use super::*;

#[test]
fn test_permutations() {
    let values = [1,2,3];
    let actual: Vec<Vec<i32>> = permutations(&values).collect();
    let expected = vec![
        vec![1,2,3],
        vec![1,3,2],
        vec![3,1,2],
        vec![3,2,1],
        vec![2,3,1],
        vec![2,1,3],
    ];

    assert_eq!(expected, actual);
}

#[test]
fn test_chain_output() {
    assert_eq!(43210, chain_output(
        &vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], 
        vec![4,3,2,1,0]));
    assert_eq!(54321, chain_output(
        &vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], 
        vec![0,1,2,3,4]));
    assert_eq!(65210, chain_output(
        &vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], 
        vec![1,0,4,3,2]));
}

#[test]
fn test_max_output() {
    assert_eq!(43210, max_output(&vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]));
}