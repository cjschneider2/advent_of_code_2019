fn main() {
    let input = include_str!("../input.txt");

    let total = part1(input);
    println!("part1: {}", total);

    let total = part2(input);
    println!("part2: {}", total);
}

fn part1(input: &str) -> i32 {
    input.lines().into_iter()
        .map(|line| line.parse::<i32>().expect("not a valid integer"))
        .fold(0, mass_to_fuel)
}

fn part2(input: &str) -> i32 {
    input.lines().into_iter()
        .map(|line| line.parse::<i32>().expect("not a valid integer"))
        .fold(0, mass_to_fuel_with_fuel)
}

/// Converts input mass to required fuel units
fn mass_to_fuel(acc: i32, mass: i32) -> i32 {
    acc + ((mass/ 3) - 2)
}

/// Converts input mass to required fuel units, also taking into account the required
/// fuel for the fuel itself
fn mass_to_fuel_with_fuel(acc: i32, mass: i32) -> i32 {
    let mut total = 0;
    let mut fuel_mass = mass_to_fuel(0, mass);
    while fuel_mass > 0 {
        total += fuel_mass;
        fuel_mass = mass_to_fuel(0, fuel_mass);
    }
    acc + total
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(mass_to_fuel(0, 12), 2);
    }

    #[test]
    fn test_14() {
        assert_eq!(mass_to_fuel(0, 14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(mass_to_fuel(0, 1969), 654);
    }

    #[test]
    fn test_100756() {
        assert_eq!(mass_to_fuel(0, 100756), 33583);
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(mass_to_fuel_with_fuel(0, 14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(mass_to_fuel_with_fuel(0, 1969), 966);
    }

    #[test]
    fn test_100756() {
        assert_eq!(mass_to_fuel_with_fuel(0, 100756), 50346);
    }
}
