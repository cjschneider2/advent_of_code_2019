fn main() {
    let input = include_str!("../input.txt");
    let total_loop = using_loop(input);
    let total_iter = using_iter(input);

    assert_eq!(total_loop, total_iter);
    println!("part1: {}", total_iter);
}

// bonus benchmark tests
fn using_loop(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mass: i32 = line.parse().expect("not a valid integer");
        total += part1(mass)
    }
    total
}

fn using_iter(input: &str) -> i32 {
    input.lines().into_iter()
        .map(|line| line.parse::<i32>().expect("not a valid integer"))
        .fold(0, |acc, mass| acc + part1(mass))
}

/// Converts input mass to required fuel units
fn part1(input: i32) -> i32 {
    /* your code here */
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(part1(12), 2);
    }

    #[test]
    fn test_14() {
        assert_eq!(part1(14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(part1(1969), 654);
    }

    #[test]
    fn test_100756() {
        assert_eq!(part1(100756), 33583);
    }
}
