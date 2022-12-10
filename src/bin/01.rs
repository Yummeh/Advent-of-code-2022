use std::num;

pub fn part_one(input: &str) -> Option<u32> {
    // print!("{}", input);
    let all_words = input.split("\n\n");

    let mut highest_sum = 0;

    for grouped_words in all_words {
        let mut sum_of_elve = 0;

        for word in grouped_words.split("\n") {
            let number = word.trim().parse::<i32>();

            match number {
                Ok(value) => {
                    sum_of_elve += value;
                }
                Err(e) => {}
            }
        }
        if sum_of_elve > highest_sum {
            highest_sum = sum_of_elve;
        }
    }

    return Some(highest_sum as u32);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_words = input.split("\n\n");

    let mut spots = [0, 0, 0];
    let mut highest_sum = 0;

    for grouped_words in all_words {
        let mut sum_of_elve = 0;

        for word in grouped_words.split("\n") {
            let number = word.trim().parse::<i32>();

            match number {
                Ok(value) => {
                    sum_of_elve += value;
                }
                Err(e) => {}
            }
        }

        if sum_of_elve > highest_sum {
            highest_sum = sum_of_elve;
        }

        set_spots(&mut spots, sum_of_elve);
    }

    let result: i32 = spots.iter().sum();

    return Some(result as u32);

    None
}

// If a higher value is found it replaces the spot and pushes everything a spot down.
fn set_spots(spots: &mut [i32; 3], value: i32) {
    if value <= spots[2] {
        return;
    }

    if value <= spots[1] {
        spots[2] = value;
        return;
    }
    if value <= spots[0] {
        spots[2] = spots[1];
        spots[1] = value;
        return;
    }

    // print!(s)
    spots[2] = spots[1];
    spots[1] = spots[0];
    spots[0] = value;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
