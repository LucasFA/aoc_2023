use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("./input");

    println!("Part one: {}", part1(INPUT));
    println!("Part two: {}", part2(INPUT));
}

fn part1(s: &str) -> u32 {
    s.split_ascii_whitespace().map(parse_line_part1).sum()
}

fn parse_line_part1(s: &str) -> u32 {
    let it = s.chars().filter(|c| c.is_numeric());

    let first = it.clone().next().unwrap();
    let last = it.last().unwrap();

    let charfinal = format!("{}{}", first, last);
    return charfinal.parse::<u32>().unwrap();
}

fn part2(s: &str) -> u32 {
    s.split_ascii_whitespace().map(parse_line_part2).sum()
}

fn parse_line_part2(s: &str) -> u32 {
    let dict: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // let r = Regex::new(r"^(?:(\d)?([A-Za-z])?)+$").unwrap(); // original
    let r = Regex::new(r"(\d)|(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut result = Vec::new();

    for cap in r.captures_iter(s) {
        let caps = cap
            .iter()
            .skip(1)
            .filter(|c| c.is_some())
            .map(|c| c.unwrap().as_str())
            .next()
            .unwrap();

        if caps.len() == 1 {
            result.push(
                caps.parse::<u32>()
                    .expect("Could not parse single character into u8"),
            );
        } else {
            result.push(1 + dict.iter().position(|&number| number == caps).unwrap() as u32);
        }
    }

    let dig1 = result[0] as u32 * 10 as u32;
    let dig2 = *result.last().unwrap() as u32;
    dig1 + dig2
}

#[cfg(test)]
mod test {
    mod part1 {
        use crate::*;
        #[test]
        fn full_example() {
            let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

            let result = part1(input);
            assert_eq!(result, 142);
        }

        #[test]
        fn simple() {
            let input = "1abc2";
            let result = part1(input);

            assert_eq!(result, 12);
        }
    }

    mod part2 {
        use crate::*;
        #[test]
        fn full_example() {
            let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
            let result = part2(input);

            assert_eq!(result, 281);
        }

        #[test]
        fn first() {
            let input = "two1nine";
            let result = part2(input);
            assert_eq!(result, 29)
        }

        #[test]
        fn overlapping() {
            let input = "zoneight234";
            let result = part2(input);
            assert_eq!(result, 14)
        }
        #[test]
        fn overlapping2() {
            let input = "xtwone3four";
            let result = part2(input);
            assert_eq!(result, 24)
        }

        #[test]
        fn edge_cases_man() {
            let input = "one234fiveight";
            let result = part2(input);
            assert_eq!(result, 18)
        }
    }
}
