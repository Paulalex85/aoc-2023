use std::cmp::Ordering;

use nom::IResult;

use crate::days::Day;

pub struct Day07;

impl Day for Day07 {
    type Input = Vec<(Vec<char>, usize)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let result = input.split('\n').map(|line| {
            let x: Vec<&str> = line.split_whitespace().collect();
            (x[0].chars().collect(), x[1].parse().unwrap())
        }).collect();

        Ok(("", result))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_result(input, false)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        get_result(input, true)
    }
}

fn get_result(input: &Vec<(Vec<char>, usize)>, joker: bool) -> usize {
    let mut ranking: Vec<(Vec<char>, usize)> = Vec::new();
    for hand in input {
        let mut inserted = false;
        for hand_in_ranking in ranking.iter().enumerate() {
            if is_a_better_than_b(hand.clone().0, hand_in_ranking.1.clone().0, joker) {
                ranking.insert(hand_in_ranking.0, hand.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            ranking.push(hand.clone());
        }
    }

    ranking.iter().enumerate().map(|x| (ranking.len() - x.0) * x.1.clone().1).sum()
}

fn is_a_better_than_b(hand_a: Vec<char>, hand_b: Vec<char>, joker: bool) -> bool {
    let a_identical = get_identical_card(hand_a.clone(), joker);
    let b_identical = get_identical_card(hand_b.clone(), joker);

    let value_j: u32 = if joker { 1 } else { 11 };

    match a_identical.cmp(&b_identical) {
        Ordering::Greater => return true,
        Ordering::Less => return false,
        Ordering::Equal => {
            for i in 0..hand_a.len() {
                match get_card_value(hand_a[i], value_j).cmp(&get_card_value(hand_b[i], value_j)) {
                    Ordering::Greater => return true,
                    Ordering::Less => return false,
                    Ordering::Equal => {},
                }
            }
        }
    }
    false
}

fn get_identical_card(hand: Vec<char>, joker: bool) -> usize {
    let mut identical_tuple = (0, 0);
    let mut hand_clone = hand.clone();
    let mut current_len = hand_clone.len();
    let mut nb_joker = 0;
    let mut result = 0;
    while current_len > 0 {
        let char_to_check = hand_clone[0];

        hand_clone.retain(|x| *x != char_to_check);
        let nb_found = current_len - hand_clone.len();
        if joker && char_to_check == 'J' {
            nb_joker = nb_found;
        } else if identical_tuple.0 <= nb_found {
            identical_tuple = (nb_found, identical_tuple.0);
        } else if identical_tuple.1 <= nb_found {
            identical_tuple.1 = nb_found;
        }
        current_len = hand_clone.len()
    }

    if identical_tuple.0 == 5 {
        result = 6;
    } else if identical_tuple.0 == 4 {
        result = 5;
    } else if identical_tuple.0 == 3 && identical_tuple.1 == 2 {
        result = 4;
    } else if identical_tuple.0 == 3 {
        result = 3;
    } else if identical_tuple.0 == 2 && identical_tuple.1 == 2 {
        result = 2;
    } else if identical_tuple.0 == 2 {
        result = 1;
    }
    if joker {
        if identical_tuple.0 + nb_joker == 5 {
            result = 6
        } else if identical_tuple.0 + nb_joker == 4 {
            result = 5
        } else if identical_tuple.0 + identical_tuple.1 + nb_joker == 5 {
            result = 4;
        } else if identical_tuple.0 + nb_joker == 3 {
            result = 3;
        } else if identical_tuple.0 + identical_tuple.1 + nb_joker == 4 {
            result = 2;
        } else if identical_tuple.0 + nb_joker == 2 {
            result = 1;
        } else {
            result = nb_joker;
        }
    }
    result
}

fn get_card_value(card: char, value_j: u32) -> u32 {
    if card.is_ascii_digit() {
        card.to_digit(10).unwrap()
    } else {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => value_j,
            'T' => 10,
            _ => { panic!("error parsing card") }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let parsed = Day07::parse(input).unwrap().1;
        assert_eq!(Day07::part_1(&parsed), 6440);
    }

    #[test]
    fn test_part1_fullhouse() {
        let input = r#"22327 97
22323 888"#;

        let parsed = Day07::parse(input).unwrap().1;
        assert_eq!(Day07::part_1(&parsed), 1873);
    }

    #[test]
    fn test_part2() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let parsed = Day07::parse(input).unwrap().1;
        assert_eq!(Day07::part_2(&parsed), 5905);
    }
}
