use std::collections::HashMap;

use nom::IResult;

use crate::days::Day;

pub struct Day08;

const START_ID: &str = "AAA";
const END_ID: &str = "ZZZ";

#[derive(Debug, Clone)]
pub struct Data {
    pub direction: Vec<char>,
    pub redirection: HashMap<String, (String, String)>,
}

impl Day for Day08 {
    type Input = Data;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let mut data = Data {
            direction: Vec::new(),
            redirection: HashMap::new()
        };
        for element in input.split("\n\n").enumerate() {
            if element.0 == 0 {
                data.direction = element.1.chars().collect();
            } else {
                element.1.split('\n').for_each(|line| {
                    let line_split: Vec<&str> = line.split('=').collect();
                    let id = line_split[0].trim().to_string();
                    let without_brace: String = line_split[1].chars().filter(|x| *x != '(' && *x != ')').collect();
                    let redirection: Vec<String> = without_brace.split(',').map(|x| x.trim().to_string()).collect();
                    data.redirection.insert(id, (redirection[0].clone(), redirection[1].clone()));
                })
            }
        }
        Ok(("", data))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result = 0;
        let mut current_id: String = START_ID.to_string();
        let mut index_direction = 0;

        while current_id != END_ID.to_string() {
            result += 1;
            let direction = input.direction[index_direction];
            let current_tuple = input.redirection.get(&*current_id).unwrap();
            match direction {
                'L' => current_id = current_tuple.clone().0,
                'R' => current_id = current_tuple.clone().1,
                _ => panic!("not good direction")
            }
            if index_direction == input.direction.len() - 1 {
                index_direction = 0;
            } else {
                index_direction += 1;
            }
        }

        result
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let ids: Vec<String> = input.redirection.keys().filter(|x| x.ends_with('A')).map(|x| x.to_string()).collect();
        let mut best_value: Vec<usize> = Vec::new();
        let mut loop_to_best_value: Vec<usize> = Vec::new();
        let mut current_result = 0;

        for id in ids {
            let mut current_id = id;
            let mut index_direction = 0;
            let mut result = 0;
            while !current_id.ends_with('Z') {
                result += 1;
                let direction = input.direction[index_direction];
                let current_tuple = input.redirection.get(&*current_id).unwrap();
                match direction {
                    'L' => current_id = current_tuple.clone().0,
                    'R' => current_id = current_tuple.clone().1,
                    _ => panic!("not good direction")
                }
                if index_direction == input.direction.len() - 1 {
                    index_direction = 0;
                } else {
                    index_direction += 1;
                }
            }
            best_value.push(result);

            result = 0;
            while result == 0 || !current_id.ends_with('Z') {
                result += 1;
                let direction = input.direction[index_direction];
                let current_tuple = input.redirection.get(&*current_id).unwrap();
                match direction {
                    'L' => current_id = current_tuple.clone().0,
                    'R' => current_id = current_tuple.clone().1,
                    _ => panic!("not good direction")
                }
                if index_direction == input.direction.len() - 1 {
                    index_direction = 0;
                } else {
                    index_direction += 1;
                }
            }
            loop_to_best_value.push(result)
        }

        println!("{:?}", best_value);
        println!("{:?}", loop_to_best_value);

        current_result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let parsed = Day08::parse(input).unwrap().1;
        assert_eq!(Day08::part_1(&parsed), 2);
    }

    #[test]
    fn test_part1_2() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        let parsed = Day08::parse(input).unwrap().1;
        assert_eq!(Day08::part_1(&parsed), 6);
    }

    #[test]
    fn test_part2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let parsed = Day08::parse(input).unwrap().1;
        assert_eq!(Day08::part_2(&parsed), 6);
    }
}
