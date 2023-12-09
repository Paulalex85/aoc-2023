use nom::IResult;

use crate::days::Day;

pub struct Day09;

impl Day for Day09 {
    type Input = Vec<Vec<i32>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let result = input.split('\n').map(|line|
            line.split_whitespace().map(|nb| nb.parse::<i32>().unwrap()).collect()
        ).collect();

        Ok(("", result))
    }


    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result: i32 = 0;

        for line in input {
            let mut tree: Vec<Vec<i32>> = vec![line.clone()];

            let mut current_last = tree.last().unwrap().clone();
            while current_last.iter().sum::<i32>() != 0 || current_last.len() > 1 {
                let mut new_entry = Vec::new();
                for index in 1..current_last.len() {
                    new_entry.push(current_last[index] - current_last[index - 1]);
                }
                tree.push(new_entry);
                current_last = tree.last().unwrap().clone();
            }

            result += tree.iter().map(|row| row.last().unwrap()).sum::<i32>();
        }
        result as usize
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut result: i32 = 0;

        for line in input {
            let mut tree: Vec<Vec<i32>> = vec![line.clone()];

            let mut current_last = tree.last().unwrap().clone();
            while current_last.iter().sum::<i32>() != 0 || current_last.len() > 1 {
                let mut new_entry = Vec::new();
                for index in 1..current_last.len() {
                    new_entry.push(current_last[index] - current_last[index - 1]);
                }
                tree.push(new_entry);
                current_last = tree.last().unwrap().clone();
            }

            tree.reverse();
            result += (1..tree.len()).fold(0, |sum, x| tree[x].first().unwrap() - sum);
        }
        result as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
-1 -2 -3 -4 "#;

        let parsed = Day09::parse(input).unwrap().1;
        assert_eq!(Day09::part_1(&parsed), 109);
    }

    #[test]
    fn test_part2() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

        let parsed = Day09::parse(input).unwrap().1;
        assert_eq!(Day09::part_2(&parsed), 2);
    }
}