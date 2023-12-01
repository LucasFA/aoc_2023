use std::iter::zip;

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

fn parse_line_part2(s: &str) -> u32 {
    const DICTIONARY: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sstring = s.to_string();
    for (idx, num_name) in zip(1..=9, DICTIONARY) {
        dbg!(idx.clone());
        dbg!(num_name.clone());
        dbg!(sstring.clone());

        sstring = sstring.replace(num_name, idx.to_string().as_str());
    }

    dbg!(&sstring);
    parse_line_part1(sstring.as_str())
}

fn part2(s: &str) -> u32 {
    s.split_ascii_whitespace().map(parse_line_part2).sum()
}
#[cfg(test)]
mod test {
    mod part1 {
        use crate::*;
        #[test]
        fn full_part1_example() {
            let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

            let result = part1(input);
            assert_eq!(result, 142);
        }

        #[test]
        fn individual1() {
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
    }
}
