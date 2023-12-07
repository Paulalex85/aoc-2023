use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, u8};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;

use crate::days::Day;

pub struct Day02;

#[derive(Debug, PartialEq)]
pub enum Color {
    Blue,
    Green,
    Red,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<RevealedSet>,
}

#[derive(Debug, PartialEq)]
pub struct RevealedSet {
    pub cube_revealed: Vec<Cube>,
}

#[derive(Debug, PartialEq)]
pub struct Cube {
    pub color: Color,
    pub nb: usize,
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    map(
        tuple((
            u8,
            tag(" "),
            map(alphanumeric1, |c| match c {
                "blue" => Color::Blue,
                "green" => Color::Green,
                "red" => Color::Red,
                _ => panic!("Unknown color"),
            }),
        )),
        |(nb, _, color)| Cube {
            color,
            nb: nb as usize,
        },
    )(input)
}

fn parse_set(input: &str) -> IResult<&str, RevealedSet> {
    map(separated_list0(tag(", "), parse_cube), |vec_cube| {
        RevealedSet {
            cube_revealed: vec_cube,
        }
    })(input)
}

fn parse_revealed_set(input: &str) -> IResult<&str, Vec<RevealedSet>> {
    map(separated_list1(tag("; "), parse_set), |vec_set| vec_set)(input)
}

fn parse_id(input: &str) -> IResult<&str, usize> {
    map(tuple((tag("Game "), u8, tag(": "))), |(_, id, _)| {
        id as usize
    })(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (rest, info) = tuple((parse_id, parse_revealed_set))(input)?;

    let game = Game {
        id: info.0,
        sets: info.1,
    };
    Ok((rest, game))
}

impl Day for Day02 {
    type Input = Vec<Game>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending,parse_game)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|game| {
                game.sets.iter().all(|set| {
                    set.cube_revealed.iter().all(|cube| {
                        (cube.color == Color::Red && cube.nb <= 12)
                            || (cube.color == Color::Green && cube.nb <= 13)
                            || (cube.color == Color::Blue && cube.nb <= 14)
                    })
                })
            })
            .map(|game| game.id)
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut result: usize = 0;

        for game in input.iter() {
            let mut low_red: usize = 0;
            let mut low_green: usize = 0;
            let mut low_blue: usize = 0;

            for revealed_set in &game.sets {
                for cube in &revealed_set.cube_revealed {
                    match cube.color {
                        Color::Red => if cube.nb > low_red { low_red = cube.nb },
                        Color::Green => if cube.nb > low_green { low_green = cube.nb },
                        Color::Blue => if cube.nb > low_blue { low_blue = cube.nb },
                    }
                }
            }

            result += low_blue * low_red * low_green;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let parsed = Day02::parse(input).unwrap().1;
        assert_eq!(Day02::part_1(&parsed), 8);
    }

    #[test]
    fn test_part2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let parsed = Day02::parse(input).unwrap().1;
        assert_eq!(Day02::part_2(&parsed), 2286);
    }
}
