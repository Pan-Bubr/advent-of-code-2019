use std::collections::HashMap;

fn main() {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    let orbit_list = get_orbits();

    for (parent, child) in orbit_list {
        orbits.entry(parent).or_insert(Vec::new()).push(child);
    }

    let orbit_count = count_orbits(&orbits, 0, "COM");

    println!("{}", orbit_count);
}

fn get_orbits() -> Vec<(String, String)> {
    let input = std::fs::read_to_string("../inputs/6.txt").unwrap();
    return input
        .lines()
        .map(|line| {
            let mut name = line.split(")");
            return (
                String::from(name.next().unwrap()),
                String::from(name.next().unwrap()),
            );
        })
        .collect();
}

fn count_orbits(orbits: &HashMap<String, Vec<String>>, count: i32, key: &str) -> i32 {
    match orbits.get(key) {
        Some(orbiters) => {
            orbiters
                .iter()
                .map(|planet| count_orbits(&orbits, count + 1, planet))
                .sum::<i32>()
                + count
        }
        None => count,
    }
}
