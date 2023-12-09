use nom::IResult;

use crate::days::Day;

pub struct Day06;

#[derive(Debug, Clone)]
pub struct Data {
    pub time: Vec<usize>,
    pub distance: Vec<usize>,
}

impl Day for Day06 {
    type Input = Data;

    fn parse(_input: &str) -> IResult<&str, Self::Input> {
        // let data = Data {
        //     time: vec![58, 81, 96, 76],
        //     distance: vec![434, 1041, 2219, 1218]
        // };
        let data = Data {
            time: vec![7, 15, 30],
            distance: vec![9, 40, 200]
        };
        Ok(("", data))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result: usize = 0;

        input.time.iter().enumerate().for_each(|time| {
            let mut nb_better = 0;
            for i in 0..*time.1 {
                let play = i * (time.1 - i);
                if play > input.distance[time.0] {
                    nb_better += 1;
                }
            }
            if result == 0 {
                result = nb_better;
            } else {
                result *= nb_better;
            }
        });

        result
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        const TIME: usize = 58819676;
        const DISTANCE: usize = 434104122191218;

        let mut result: usize = 0;

        for i in 0..TIME {
            let play = i * (TIME - i);
            if play > DISTANCE {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let parsed = Day06::parse(input).unwrap().1;
        assert_eq!(Day06::part_1(&parsed), 288);
    }
}