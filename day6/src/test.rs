use super::*;

#[test]
fn test_parse_line() {
    assert_eq!(("COM", "B"), parse_line("COM)B"));
    assert_eq!(("B", "C"), parse_line("B)C"));
}

#[test]
fn test_parse_orbits() {
    let lines = vec![
        "COM)B",
        "B)C",
        "C)D",
        "D)E",
        "E)F",
        "B)G",
        "G)H",
        "D)I",
        "E)J",
        "J)K",
        "K)L",
    ];

    let mut expected = HashMap::new();
    expected.insert("B", "COM");
    expected.insert("C", "B");
    expected.insert("D", "C");
    expected.insert("E", "D");
    expected.insert("F", "E");
    expected.insert("G", "B");
    expected.insert("H", "G");
    expected.insert("I", "D");
    expected.insert("J", "E");
    expected.insert("K", "J");
    expected.insert("L", "K");

    assert_eq!(expected, parse_orbits(&lines));
}

#[test]
fn test_total_orbits() {
    let lines = vec![
        "COM)B",
        "B)C",
        "C)D",
        "D)E",
        "E)F",
        "B)G",
        "G)H",
        "D)I",
        "E)J",
        "J)K",
        "K)L",
    ];

    let orbit_map = parse_orbits(&lines);
    assert_eq!(42, total_orbits(&orbit_map));
}

#[test]
fn test_fewest_transfers() {
    let lines = vec![
            "COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L",
            "K)YOU",
            "I)SAN",
    ];

    let orbit_map = parse_orbits(&lines);
    assert_eq!(4, fewest_transfers(&orbit_map, "YOU", "SAN"));
}