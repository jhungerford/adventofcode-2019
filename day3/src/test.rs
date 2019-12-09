use super::*;

#[test]
fn closest_intersection_ex1() {
    let segments1 = parse_segments(&"R8,U5,L5,D3");
    let segments2 = parse_segments(&"U7,R6,D4,L4");

    assert_eq!(6, closest_intersection(&segments1, &segments2));
}

#[test]
fn closest_intersection_ex2() {
    let segments1 = parse_segments(&"R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let segments2 = parse_segments(&"U62,R66,U55,R34,D71,R55,D58,R83");

    assert_eq!(159, closest_intersection(&segments1, &segments2));
}

#[test]
fn closest_intersection_ex3() {
    let segments1 = parse_segments(&"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let segments2 = parse_segments(&"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

    assert_eq!(135, closest_intersection(&segments1, &segments2));
}

#[test]
fn first_intersection_ex1() {
    let segments1 = parse_segments(&"R8,U5,L5,D3");
    let segments2 = parse_segments(&"U7,R6,D4,L4");

    assert_eq!(30, first_intersection(&segments1, &segments2));
}

#[test]
fn first_intersection_ex2() {
    let segments1 = parse_segments(&"R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let segments2 = parse_segments(&"U62,R66,U55,R34,D71,R55,D58,R83");

    assert_eq!(610, first_intersection(&segments1, &segments2));
}

#[test]
fn first_intersection_ex3() {
    let segments1 = parse_segments(&"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let segments2 = parse_segments(&"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

    assert_eq!(410, first_intersection(&segments1, &segments2));
}
