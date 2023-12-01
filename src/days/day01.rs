use std::char::from_digit;
use nom::character::complete::{line_ending, not_line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list0;

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<String>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, map(not_line_ending, |s: &str| s.to_string()))(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result: usize = 0;
        for i in input {
            let first_number: char = i.chars().find(|c| c.is_numeric()).unwrap();
            let second_number: char = i.chars().rev().find(|c| c.is_numeric()).unwrap();
            result += two_char_to_number(&first_number, &second_number);
        }
        result
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let number_string: &[&str; 9] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let mut result = 0;
        for i in input {
            let mut first_number: char = '0';
            let mut second_number: char = '0';
            for j in 0..i.len() {
                if i.chars().nth(j).unwrap().is_numeric() {
                    first_number = i.chars().nth(j).unwrap();
                    break;
                } else if number_string.iter().any(|x| i[..=j].contains(x)) {
                    first_number = from_digit((number_string.iter().position(|&r| i.get(0..=j).unwrap().contains(r)).unwrap() + 1) as u32, 10).unwrap();
                    break;
                }
            }
            for j in (0..i.len()).rev() {
                if i.chars().nth(j).unwrap().is_numeric() {
                    second_number = i.chars().nth(j).unwrap();
                    break;
                } else if number_string.iter().any(|x| i.get(j..).unwrap().contains(x)) {
                    second_number = from_digit((number_string.iter().position(|&r|i.get(j..).unwrap().contains(r)).unwrap() + 1) as u32, 10).unwrap();
                    break;
                }
            }
            if first_number == '0' || second_number == '0' {
                panic!("Could not find two numbers in {}", i)
            }
            result += two_char_to_number(&first_number, &second_number);
        }

        result
    }
}

fn two_char_to_number(char_one: &char, char_two: &char) -> usize {
    (char_one.to_string() + &char_two.to_string()).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let parsed = Day01::parse(input).unwrap().1;
        assert_eq!(Day01::part_1(&parsed), 142);
    }

    #[test]
    fn test_part2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        let parsed = Day01::parse(input).unwrap().1;
        assert_eq!(Day01::part_2(&parsed), 281);
    }
}