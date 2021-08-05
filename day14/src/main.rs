use std::collections::HashMap;

use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct MaterialCount<'a> {
    material: &'a str,
    count: u64,
}

impl MaterialCount<'_> {
    fn of(count: u64, material: &str) -> MaterialCount {
        MaterialCount {material, count}
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Reactions<'a> {
    reactions: HashMap<MaterialCount<'a>, Vec<MaterialCount<'a>>>
}

impl Reactions<'_> {
    /// Converts the given lines into a map of material count -> the material counts required to produce it.
    fn parse(lines: Vec<&str>) -> Reactions {
        let pattern = Regex::new(r"(\d+) ([A-Z]+)").unwrap();

        let mut map = HashMap::new();

        for line in lines {
            let materials: Vec<MaterialCount> = pattern.captures_iter(line).map(|cap| MaterialCount::of(
                cap[1].parse::<u64>().unwrap(),
                cap.get(2).unwrap().as_str()
            )).collect();

            map.insert(materials[materials.len() - 1], materials[0..materials.len() - 1].to_vec());
        }

        Reactions {
            reactions: map
        }
    }

    /// Returns the amount of ore that's required to produce the given amount of fuel.
    fn ore_required(&self, fuel: u64) -> u64 {
        // Start with fuel, push requirements onto stack
        let mut needs = HashMap::<&str, u64>::new();
        let mut leftovers = HashMap::<&str, u64>::new();
        let mut ore_produced = 0;

        needs.insert("FUEL", fuel);

        while !needs.is_empty() {
            let making_material = *needs.keys().next().unwrap();
            let mut making_count = needs.remove(making_material).unwrap();

            // Check if there's already leftovers.
            match leftovers.get(making_material) {
                None => {},
                Some(&leftover_count) if leftover_count > making_count => {
                    leftovers.insert(making_material, leftover_count - making_count);
                    continue;
                },
                Some(&leftover_count) if leftover_count == making_count => {
                    leftovers.remove(making_material);
                    continue;
                },
                Some(&leftover_count) => {
                    leftovers.remove(making_material);
                    making_count -= leftover_count;
                }
            }

            // Find the right reaction to use.
            let (reaction_result, reaction_requirements) = self.reactions.iter()
                .find(|(produced, _inputs)| produced.material == making_material)
                .unwrap();

            // Figure out how many times to run the reaction to produce enough material.
            let times = (making_count as f64 / reaction_result.count as f64).ceil() as u64;

            // Run the reaction - add the requirements to need
            for input in reaction_requirements {
                if input.material == "ORE" {
                    ore_produced += input.count * times;
                } else {
                    needs.insert(input.material, needs.get(input.material).unwrap_or(&0) + input.count * times);
                }
            }

            // Store any unused produced material in leftovers.
            let leftover_count = reaction_result.count * times - making_count;
            if leftover_count > 0 {
                leftovers.insert(making_material, leftovers.get(making_material).unwrap_or(&0) + leftover_count);
            }
        }

        ore_produced
    }

    /// Returns the amount of fuel that can be produced by the given amount of ore.
    fn fuel_produced(&self, ore: u64) -> u64 {
        let one_fuel_ore = self.ore_required(1);
        let mut min_fuel = ore / one_fuel_ore;
        let mut max_fuel = min_fuel * 2;

        while self.ore_required(max_fuel) < ore {
            max_fuel += min_fuel;
        }

        let mut split = min_fuel + (max_fuel - min_fuel) / 2;
        while self.ore_required(split) > ore || self.ore_required(split + 1) < ore {
            let ore_required = self.ore_required(split);

            if ore_required > ore {
                max_fuel = split;
            } else {
                min_fuel = split;
            }

            split = min_fuel + (max_fuel - min_fuel) / 2;
        }

        split
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec![
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ];

        let mut expected = HashMap::new();
        expected.insert(MaterialCount::of(10, "A"), vec![MaterialCount::of(10, "ORE")]);
        expected.insert(MaterialCount::of(1, "B"), vec![MaterialCount::of(1, "ORE")]);
        expected.insert(MaterialCount::of(1, "C"), vec![MaterialCount::of(7, "A"), MaterialCount::of(1, "B")]);
        expected.insert(MaterialCount::of(1, "D"), vec![MaterialCount::of(7, "A"), MaterialCount::of(1, "C")]);
        expected.insert(MaterialCount::of(1, "E"), vec![MaterialCount::of(7, "A"), MaterialCount::of(1, "D")]);
        expected.insert(MaterialCount::of(1, "FUEL"), vec![MaterialCount::of(7, "A"), MaterialCount::of(1, "E")]);

        assert_eq!(Reactions::parse(lines), Reactions {reactions: expected});
    }

    #[test]
    fn test_ex1() {
        let lines = vec![
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ];

        let reactions = Reactions::parse(lines);
        assert_eq!(reactions.ore_required(1), 31);
    }

    #[test]
    fn test_ex2() {
        let lines = vec![
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ];

        let reactions = Reactions::parse(lines);
        assert_eq!(reactions.ore_required(1), 165);
    }

    #[test]
    fn test_ex3() {
        let lines = vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ];

        let reactions = Reactions::parse(lines);
        assert_eq!(reactions.ore_required(1), 13312);
        assert_eq!(reactions.fuel_produced(1000000000000), 82892753);
    }

    #[test]
    fn test_ex4() {
        let lines = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ];

        let reactions = Reactions::parse(lines);
        assert_eq!(reactions.ore_required(1), 180697);
        assert_eq!(reactions.fuel_produced(1000000000000), 5586022);
    }

    #[test]
    fn test_ex5() {
        let lines = vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ];

        let reactions = Reactions::parse(lines);
        assert_eq!(reactions.ore_required(1), 2210736);
        assert_eq!(reactions.fuel_produced(1000000000000), 460664);
    }
}

/// Converts the given file into a vector of lines.
fn file_to_lines(name: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(name).expect("File does not exist.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Error parsing line."))
        .collect()
}

fn main() {
    let read_lines = file_to_lines("input.txt");
    let lines: Vec<&str> = read_lines.iter().map(|s| s as &str).collect();

    let reactions = Reactions::parse(lines);
    println!("Part 1: {}", reactions.ore_required(1));
    println!("Part 2: {}", reactions.fuel_produced(1000000000000));
}
