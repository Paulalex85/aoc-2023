use std::collections::HashMap;

use nom::IResult;

use crate::days::Day;

pub struct Day03;

impl Day for Day03 {
    type Input = HashMap<(u32, u32), char>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let mut hash_map: HashMap<(u32, u32), char> = HashMap::new();
        input
            .split('\n')
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
        let mut result = 0;
        let mut temp_nb: String = String::from("");
        let max_x = input.keys().map(|x| x.0).max().unwrap() + 1;
        let max_y = input.keys().map(|x| x.1).max().unwrap() + 1;
        for y_index in 0..max_y {
            for x_index in 0..max_x {
                let c = match input.get(&(x_index, y_index)) {
                    Some(_x) => _x,
                    None => panic!("can t get at index {} {}", x_index, y_index)
                };
                if c.is_ascii_digit() {
                    temp_nb.push(*c);
                }
                let end_of_line: bool = x_index == max_x - 1;
                if !temp_nb.is_empty() && (c.is_ascii_punctuation() || end_of_line) {
                    let mut pos_to_check: Vec<(u32, u32)> = Vec::new();
                    let last_digit_position: u32 = if c.is_ascii_digit() { x_index } else { x_index - 1 };
                    let is_upper_bound: bool = y_index == 0;
                    let is_lower_bound: bool = y_index == max_y - 1;
                    if last_digit_position >= temp_nb.len() as u32 {
                        let before_digit_x = last_digit_position - temp_nb.len() as u32;
                        pos_to_check.push((before_digit_x, y_index));
                        if !is_upper_bound {
                            pos_to_check.push((before_digit_x, y_index - 1));
                        }
                        if !is_lower_bound {
                            pos_to_check.push((before_digit_x, y_index + 1));
                        }
                    }
                    if !end_of_line {
                        pos_to_check.push((x_index, y_index));
                        if !is_upper_bound {
                            pos_to_check.push((x_index, y_index - 1));
                        }
                        if !is_lower_bound {
                            pos_to_check.push((x_index, y_index + 1));
                        }
                    }
                    if !is_upper_bound {
                        temp_nb.chars().enumerate().for_each(|x| pos_to_check.push((last_digit_position - x.0 as u32, y_index - 1)));
                    }
                    if !is_lower_bound {
                        temp_nb.chars().enumerate().for_each(|x| pos_to_check.push((last_digit_position - x.0 as u32, y_index + 1)));
                    }

                    for pos in pos_to_check {
                        let current_c = input.get(&(pos.0, pos.1)).unwrap();
                        if current_c.is_ascii_punctuation() && *current_c != '.' {
                            result += temp_nb.parse::<usize>().unwrap();
                            break;
                        }
                    }
                    temp_nb = String::from("");
                }
            }
        }
        result
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut result: usize = 0;
        let max_x = input.keys().map(|x| x.0).max().unwrap() + 1;
        let max_y = input.keys().map(|x| x.1).max().unwrap() + 1;
        for y_index in 0..max_y {
            for x_index in 0..max_x {
                let c = match input.get(&(x_index, y_index)) {
                    Some(_x) => *_x,
                    None => panic!("can t get at index {} {}", x_index, y_index)
                };
                if c == '*' {
                    let mut pos_to_check: Vec<(u32, u32)> = Vec::new();

                    //check left
                    if is_in_bounds((x_index - 1) as i32, y_index as i32, max_x, max_y) && is_pos_digit(input, x_index - 1, y_index) {
                        pos_to_check.push((x_index - 1, y_index));
                    }

                    //check right
                    if is_in_bounds((x_index + 1) as i32, y_index as i32, max_x, max_y) && is_pos_digit(input, x_index + 1, y_index) {
                        pos_to_check.push((x_index + 1, y_index));
                    }

                    //check top
                    if is_in_bounds(x_index as i32, (y_index - 1) as i32, max_x, max_y) && is_pos_digit(input, x_index, y_index - 1) {
                        pos_to_check.push((x_index, y_index - 1));
                    } else {
                        //check top left
                        if is_in_bounds((x_index - 1) as i32, (y_index - 1) as i32, max_x, max_y) && is_pos_digit(input, x_index - 1, y_index - 1) {
                            pos_to_check.push((x_index - 1, y_index - 1));
                        }
                        //check top right
                        if is_in_bounds((x_index + 1) as i32, (y_index - 1) as i32, max_x, max_y) && is_pos_digit(input, x_index + 1, y_index - 1) {
                            pos_to_check.push((x_index + 1, y_index - 1));
                        }
                    }

                    //check bottom
                    if is_in_bounds(x_index as i32, (y_index + 1) as i32, max_x, max_y) && is_pos_digit(input, x_index, y_index + 1) {
                        pos_to_check.push((x_index, y_index + 1));
                    } else {
                        //check bottom left
                        if is_in_bounds((x_index - 1) as i32, (y_index + 1) as i32, max_x, max_y) && is_pos_digit(input, x_index - 1, y_index + 1) {
                            pos_to_check.push((x_index - 1, y_index + 1));
                        }
                        //check bottom right
                        if is_in_bounds((x_index + 1) as i32, (y_index + 1) as i32, max_x, max_y) && is_pos_digit(input, x_index + 1, y_index + 1) {
                            pos_to_check.push((x_index + 1, y_index + 1));
                        }
                    }

                    if pos_to_check.len() >= 2 {
                        let first_nb: usize = get_nb(input, pos_to_check.get(0).unwrap().0, pos_to_check.get(0).unwrap().1, max_x, max_y);
                        let second_nb: usize = get_nb(input, pos_to_check.get(1).unwrap().0, pos_to_check.get(1).unwrap().1, max_x, max_y);
                        result += first_nb * second_nb;
                    }
                }
            }
        }
        result
    }
}

fn is_in_bounds(x: i32, y: i32, max_x: u32, max_y: u32) -> bool {
    x < max_x as i32 && y < max_y as i32 && x >= 0 && y >= 0
}

fn is_pos_digit(input: &HashMap<(u32, u32), char>, x: u32, y: u32) -> bool {
    if input.get(&(x, y)).unwrap().is_ascii_digit() {
        return true;
    }
    false
}


fn get_nb(input: &HashMap<(u32, u32), char>, x: u32, y: u32, max_x: u32, max_y: u32) -> usize {
    let mut temp_nb: String = String::from("");
    temp_nb.push(*input.get(&(x, y)).unwrap());

    //left
    let mut x_check: u32 = x;
    loop {
        x_check = match x_check.checked_sub(1) {
            Some(_x) => _x,
            None => break
        };
        if is_pos_digit(input, x_check, y) {
            temp_nb.insert(0, *input.get(&(x_check, y)).unwrap());
        } else { break }
    }

    x_check = x;
    loop {
        x_check += 1;

        if is_in_bounds(x_check as i32, y as i32, max_x, max_y) && is_pos_digit(input, x_check, y) {
            temp_nb.push(*input.get(&(x_check, y)).unwrap());
        } else { break }
    }

    temp_nb.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let parsed = Day03::parse(input).unwrap().1;
        assert_eq!(Day03::part_1(&parsed), 4361);
    }

    #[test]
    fn test_part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let parsed = Day03::parse(input).unwrap().1;
        assert_eq!(Day03::part_2(&parsed), 467835);
    }
}
