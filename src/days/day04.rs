use nom::IResult;

use crate::days::Day;

pub struct Day04;

impl Day for Day04 {
    type Input = Vec<(Vec<u32>, Vec<u32>)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let mut result: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();

        for game in input.split('\n') {
            let mut game_vec: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());

            game.split('|').enumerate().for_each(|part| {
                let vec_nb: Vec<u32> = part.1.trim().split_whitespace().filter_map(|nb| nb.parse::<u32>().ok()).collect();
                if part.0 == 0 {
                    game_vec.0 = vec_nb
                } else {
                    game_vec.1 = vec_nb
                }
            });


            result.push(game_vec);
        }

        Ok(("", result))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|game| {
                let count_value: usize = game.0
                                             .iter()
                                             .filter(|&nb|
                                                 game.1.contains(nb)
                                             ).count();

                if count_value > 0 {
                    //2^(x-1)
                    2_usize.pow((count_value - 1) as u32)
                } else {
                    0
                }
            }).sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut winning_cards: Vec<usize> = Vec::new();
        input
            .iter()
            .enumerate()
            .for_each(|game| {
                let count_value: usize = game.1.0
                                             .iter()
                                             .filter(|&nb|
                                                 game.1.1.contains(nb)
                                             ).count();

                increment_winning_card(&mut winning_cards, game.0, 1);
                let nb_to_add = winning_cards[game.0];
                for i in 1..=count_value {
                    increment_winning_card(&mut winning_cards, game.0 + i, nb_to_add);
                }
            });
        winning_cards.iter().sum()
    }
}

fn increment_winning_card(winning_cards: &mut Vec<usize>, index: usize, times: usize) {
    while winning_cards.len() <= index {
        winning_cards.push(0);
    }
    winning_cards[index] += times;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let parsed = Day04::parse(input).unwrap().1;
        assert_eq!(Day04::part_1(&parsed), 13);
    }

    #[test]
    fn test_part2() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let parsed = Day04::parse(input).unwrap().1;
        assert_eq!(Day04::part_2(&parsed), 30);
    }
}