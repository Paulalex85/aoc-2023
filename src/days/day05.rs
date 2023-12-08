use nom::IResult;

use crate::days::Day;

pub struct Day05;

#[derive(Debug, Clone)]
pub struct Data {
    pub seeds: Vec<usize>,
    pub redirection: Vec<Vec<(usize, usize, usize)>>
}

impl Day for Day05 {
    type Input = Data;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let mut data = Data {
            seeds: Vec::new(),
            redirection: Vec::new()
        };
        for element in input.split("\n\n") {
            let title: Vec<&str> = element.split(':').collect();
            if title[0].contains("seeds") {
                data.seeds = title[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
            } else {
                data.redirection.push(title[1].trim().split('\n').map(|x| {
                    let list: Vec<usize> = x.split_whitespace().map(|y| y.parse().unwrap()).collect();
                    (list[0], list[1], list[2])
                }).collect())
            }
        }
        Ok(("", data))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.seeds.iter().map(
            |seed|
                {
                    let mut current_value = *seed;
                    for remap in input.redirection.clone() {
                        current_value = get_remap(remap, current_value);
                    }
                    current_value
                }
        ).min().unwrap()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut new_seeds: Vec<(usize, usize)> = Vec::new();
        let mut result: usize = 0;
        // let mut new_seeds: Vec<usize> = Vec::new();
        for i in 0..input.seeds.len() / 2 {
            new_seeds.push((input.seeds[i * 2], input.seeds[i * 2] + input.seeds[i * 2 + 1]));
        }
        println!("{:?}", new_seeds[0]);

        println!("{}", new_seeds.len());

        let redirection_rev = input.redirection.clone();
        loop {
            let mut current_value = result;
            for remap in redirection_rev.iter().rev() {
                current_value = get_remap_reverse(remap, current_value);
            }
            if is_in_seed(new_seeds.clone(), current_value) {
                break;
            } else {
                result += 1;
            }
        }
        result
    }
}

fn is_in_seed(seeds: Vec<(usize, usize)>, value: usize) -> bool {
    for seed_range in seeds {
        if value >= seed_range.0 && value < seed_range.1 {
            return true;
        }
    }
    false
}

fn get_remap(remap_vec: Vec<(usize, usize, usize)>, value: usize) -> usize {
    for remap in remap_vec {
        if remap.1 <= value && remap.1 + remap.2 > value {
            return remap.0 + value - remap.1;
        }
    }

    value
}

fn get_remap_reverse(remap_vec: &Vec<(usize, usize, usize)>, value: usize) -> usize {
    for remap in remap_vec {
        if remap.0 <= value && remap.0 + remap.2 > value {
            return remap.1 + value - remap.0;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let parsed = Day05::parse(input).unwrap().1;
        assert_eq!(Day05::part_1(&parsed), 35);
    }

    #[test]
    fn test_part2() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let parsed = Day05::parse(input).unwrap().1;
        assert_eq!(Day05::part_2(&parsed), 46);
    }
}
