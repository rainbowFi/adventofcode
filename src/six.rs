use std::collections::HashMap;

pub fn run_a() {
    let file = std::fs::read_to_string("inputs/input_6.txt").expect("Unable to open file");
    let num_orbits = total_orbits(&file);
    println!("6A: num orbits {}", num_orbits);
}

pub fn run_b() {
    let file = std::fs::read_to_string("inputs/input_6.txt").expect("Unable to open file");
    let num_orbits = path_length(&file);
    println!("6B: num orbits {}", num_orbits);
}

fn total_orbits(orbit_map: &str) -> i32 {
    let orbit_map = read_map(orbit_map);
    let orbits = orbits_from_map(&orbit_map);
    let mut num_routes = 0;

    for link in orbit_map {
        num_routes += count_routes(&orbits, &link.1);
    }

    num_routes
}

fn path_length(orbit_map: &str) -> i32 {
    let orbit_map = read_map(orbit_map);
    let orbits = orbits_from_map(&orbit_map);

    let mut our_path = get_path(&orbits, String::from("YOU"), String::from("COM"));
    let mut santa_path = get_path(&orbits, String::from("SAN"), String::from("COM"));

    //println!("our_path {:?}, santa_path {:?}", our_path, santa_path);
    while our_path.last() == santa_path.last() {
        our_path.pop();
        santa_path.pop();
    }

    (our_path.len() + santa_path.len()) as i32
}

fn get_path(orbit_map: &HashMap<String, String>, start: String, end: String) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    if let Some(mut next_planet) = orbit_map.get(&start) {
        loop {
            path.push(next_planet.clone());
            let planet = orbit_map.get(next_planet).expect("Missing entry");
            if String::from(planet) == end {
                break;
            } else {
                next_planet = planet;
            }
        }
    }
    path
}

fn read_map(orbit_data: &str) -> Vec<(String, String)> {
    let orbit_map: Vec<_> = orbit_data
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split(')').collect();
            (words[0].to_string(), words[1].to_string())
        })
        .collect();
    //println!("{:?}", orbit_map);
    orbit_map
}

fn orbits_from_map(orbit_map: &Vec<(String, String)>) -> HashMap<String, String> {
    let mut mapped_links = HashMap::new();

    for link in orbit_map {
        mapped_links.insert(link.1.clone(), link.0.clone());
    }

    //println!("{:?}", mapped_links);

    mapped_links
}

fn count_routes(orbits: &HashMap<String, String>, start: &String) -> i32 {
    let mut num_routes = 0;

    if let Some(next_planet) = orbits.get(start) {
        num_routes += 1;
        num_routes += count_routes(orbits, next_planet);
    }

    num_routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_count() {
        assert_eq!(
            42,
            total_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
        )
    }

    #[test]
    fn test_path_length() {
        assert_eq!(
            4,
            path_length("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")
        )
    }
}
