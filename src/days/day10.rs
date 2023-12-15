use std::collections::HashMap;
use itertools::Itertools;
use nom::IResult;

use crate::days::Day;

pub struct Day10;

impl Day for Day10 {
    type Input = HashMap<(u32, u32), char>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let mut hash_map: HashMap<(u32, u32), char> = HashMap::new();
        input
            .split('\n')
            .rev()
            .enumerate()
            .for_each(|line| {
                line
                    .1
                    .chars()
                    .enumerate()
                    .for_each(|c| if let Some(_x) = hash_map
                        .insert((c.0 as u32, line.0 as u32), c.1) { panic!("should not erase {:?}", c) }
                    )
            }
            );
        Ok(("", hash_map))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let max_x = input.keys().map(|x| x.0).max().unwrap();
        let max_y = input.keys().map(|x| x.1).max().unwrap();
        let start_position: (u32, u32) = *input.iter().find(|x| *x.1 == 'S').unwrap().0;
        let mut paths: Vec<Vec<(u32, u32)>> = Vec::new();
        let mut paths_to_check: Vec<Vec<(u32, u32)>> = vec![];

        if is_in_bounds(start_position.0 as i32 + 1, start_position.1 as i32, max_x, max_y) {
            if let Some(x) = check_right(input.clone(), vec![start_position], start_position, (max_x, max_y)) {
                paths_to_check.push(vec![start_position, x]);
            }
        }

        if is_in_bounds(start_position.0 as i32 - 1, start_position.1 as i32, max_x, max_y) {
            if let Some(x) = check_left(input.clone(), vec![start_position], start_position, (max_x, max_y)) {
                paths_to_check.push(vec![start_position, x]);
            }
        }

        if is_in_bounds(start_position.0 as i32, start_position.1 as i32 + 1, max_x, max_y) {
            if let Some(x) = check_top(input.clone(), vec![start_position], start_position, (max_x, max_y)) {
                paths_to_check.push(vec![start_position, x]);
            }
        }

        if is_in_bounds(start_position.0 as i32, start_position.1 as i32 - 1, max_x, max_y) {
            if let Some(x) = check_bottom(input.clone(), vec![start_position], start_position, (max_x, max_y)) {
                paths_to_check.push(vec![start_position, x]);
            }
        }
        println!("path to check {:?}", paths_to_check);
        while !paths_to_check.is_empty() {
            let current_path: Vec<(u32, u32)> = paths_to_check.pop().unwrap();
            if *input.get(current_path.last().unwrap()).unwrap() == 'S' {
                paths.push(current_path);
            } else {
                let next_position: Option<(u32, u32)> = get_next_position(current_path.clone(), input.clone(), (max_x, max_y));
                if next_position.is_some() {
                    let mut new_path = current_path.clone();
                    new_path.push(next_position.unwrap());
                    paths_to_check.push(new_path);
                }
            }
        }
        println!("{}", paths.len());
        // println!("{:?}", paths);
        paths.iter().map(|x| x.len() - 1).max().unwrap().div_ceil(2)
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}

fn get_next_position(current_path: Vec<(u32, u32)>, input: HashMap<(u32, u32), char>, max: (u32, u32)) -> Option<(u32, u32)> {
    let x = current_path.last().unwrap().0;
    let y = current_path.last().unwrap().1;
    let current_char = input.get(&(x, y)).unwrap();
    match current_char {
        '|' => {
            if let Some(x) = check_top(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            if let Some(x) = check_bottom(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            return None
        },
        '-' => {
            if let Some(x) = check_left(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            if let Some(x) = check_right(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            return None
        },
        'L' => {
            if let Some(x) = check_top(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            if let Some(x) = check_right(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            return None
        },
        'J' => {
            if let Some(x) = check_top(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            if let Some(x) = check_left(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            return None
        },
        '7' => {
            if let Some(x) = check_bottom(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            if let Some(x) = check_left(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            return None
        },
        'F' => {
            if let Some(x) = check_bottom(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            if let Some(x) = check_right(input.clone(), current_path.clone(), (x, y), max) { return Some(x) }
            return None
        },
        '.' => println!("arrived a . case {} {}", x, y),
        'S' => println!("Loop"),
        _ => panic!("wrong {}", current_char)
    }
    None
}

fn check_top(input: HashMap<(u32, u32), char>, current_path: Vec<(u32, u32)>, current_position: (u32, u32), max: (u32, u32)) -> Option<(u32, u32)> {
    if let Some(x) = is_valid_link(input.clone(),
                                   current_path.clone(),
                                   vec!['|', '7', 'F', 'S'],
                                   current_position,
                                   (0, 1),
                                   max) {
        return Some(x)
    }
    None
}

fn check_left(input: HashMap<(u32, u32), char>, current_path: Vec<(u32, u32)>, current_position: (u32, u32), max: (u32, u32)) -> Option<(u32, u32)> {
    if let Some(x) = is_valid_link(input.clone(),
                                   current_path.clone(),
                                   vec!['-', 'L', 'F', 'S'],
                                   current_position,
                                   (-1, 0),
                                   max) {
        return Some(x)
    }
    None
}

fn check_bottom(input: HashMap<(u32, u32), char>, current_path: Vec<(u32, u32)>, current_position: (u32, u32), max: (u32, u32)) -> Option<(u32, u32)> {
    if let Some(x) = is_valid_link(input.clone(),
                                   current_path.clone(),
                                   vec!['|', 'L', 'J', 'S'],
                                   current_position,
                                   (0, -1),
                                   max) {
        return Some(x)
    }
    None
}

fn check_right(input: HashMap<(u32, u32), char>, current_path: Vec<(u32, u32)>, current_position: (u32, u32), max: (u32, u32)) -> Option<(u32, u32)> {
    if let Some(x) = is_valid_link(input.clone(),
                                   current_path.clone(),
                                   vec!['-', '7', 'J', 'S'],
                                   current_position,
                                   (1, 0),
                                   max) {
        return Some(x)
    }
    None
}

fn is_valid_link(input: HashMap<(u32, u32), char>, current_path: Vec<(u32, u32)>, valid_next_char: Vec<char>, current_position: (u32, u32), modification_position: (i32, i32), max: (u32, u32)) -> Option<(u32, u32)> {
    if is_in_bounds(current_position.0 as i32 + modification_position.0, current_position.1 as i32 + modification_position.1, max.0, max.1) {
        let next_position: (u32, u32) = ((current_position.0 as i32 + modification_position.0) as u32, (current_position.1 as i32 + modification_position.1) as u32);
        let next_char = *input.get(&next_position).unwrap();
        if (!is_in_path(next_position.0, next_position.1, current_path.clone()) && valid_next_char.contains(&next_char))
            || (next_char == 'S' && current_path.len() > 2) {
            return Some(next_position);
        }
    }

    None
}

fn get_adjacent_position(x_max: u32, y_max: u32, current_path: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let x = current_path.last().unwrap().0;
    let y = current_path.last().unwrap().1;
    let mut result: Vec<(u32, u32)> = Vec::new();
    let modification: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for modif in modification {
        let next_position: (i32, i32) = ((x as i32) + modif.0, (y as i32) + modif.1);
        if is_in_bounds(next_position.0, next_position.1, x_max, y_max) && !is_in_path(next_position.0 as u32, next_position.1 as u32, current_path.clone()) {
            result.push((next_position.0 as u32, next_position.1 as u32))
        }
    }

    result
}

fn is_in_path(x: u32, y: u32, current_path: Vec<(u32, u32)>) -> bool {
    return current_path.iter().contains(&(x, y));
}

fn is_in_bounds(x: i32, y: i32, max_x: u32, max_y: u32) -> bool {
    x <= max_x as i32 && y <= max_y as i32 && x >= 0 && y >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

        let parsed = Day10::parse(input).unwrap().1;
        assert_eq!(Day10::part_1(&parsed), 8);
    }
}

