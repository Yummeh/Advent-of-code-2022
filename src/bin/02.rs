fn win(points: &mut i32) {
    *points += 6;
}

fn draw(points: &mut i32) {
    *points += 3;
}

fn loss(points: &mut i32) {
    // points += 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut final_score = 0;

    for line in input.split("\n") {
        let mut is_your_shape = false;

        let mut you = "";
        let mut opponent = "";

        for letter in line.split(' ') {
            if is_your_shape {
                you = letter;
            } else {
                opponent = letter;
            }

            is_your_shape = !is_your_shape;
        }

        // Can fail if a character has been removed from the file, or if another empty line was added.
        let you = you.chars().nth(0).unwrap();
        let opponent = opponent.chars().nth(0).unwrap();

        let battle_score = calc_battle_score(you, opponent);

        final_score += battle_score;
    }
    return Some(final_score as u32);

    None
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn calc_battle_score(you: char, opponent: char) -> i32 {
    let mut battle_score = 0;

    let you_shape = match you {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => {
            panic!("AAAAAAA WHAT IS THIS SHAAPEEE: {}", opponent)
        }
    };

    let opponent_shape = match opponent {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => {
            panic!("AAAAAAA WHAT IS THIS SHAAPEEE: {}", opponent)
        }
    };

    match you_shape {
        Shape::Rock => match opponent_shape {
            Shape::Rock => draw(&mut battle_score),
            Shape::Paper => loss(&mut battle_score),
            Shape::Scissors => win(&mut battle_score),
        },
        Shape::Paper => match opponent_shape {
            Shape::Rock => win(&mut battle_score),
            Shape::Paper => draw(&mut battle_score),
            Shape::Scissors => loss(&mut battle_score),
        },
        Shape::Scissors => match opponent_shape {
            Shape::Rock => loss(&mut battle_score),
            Shape::Paper => win(&mut battle_score),
            Shape::Scissors => draw(&mut battle_score),
        },
    }

    match you_shape {
        Shape::Rock => battle_score += 1,
        Shape::Paper => battle_score += 2,
        Shape::Scissors => battle_score += 3,
    }

    battle_score
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut final_score = 0;

    for line in input.split("\n") {
        let mut is_your_shape = false;

        let mut state = "";
        let mut opponent = "";

        for letter in line.split(' ') {
            if is_your_shape {
                state = letter;
            } else {
                opponent = letter;
            }

            is_your_shape = !is_your_shape;
        }

        // Can fail if a character has been removed from the file, or if another empty line was added.
        let state = state.chars().nth(0).unwrap();
        let opponent = opponent.chars().nth(0).unwrap();

        let result = calc_part_2_battle_score(opponent, state);

        final_score += result;
    }
    return Some(final_score as u32);
}

#[derive(Debug)]
enum State {
    Win,
    Draw,
    Lose,
}

/* I */
fn calc_part_2_battle_score(opponent: char, required_state: char) -> i32 {
    let mut battle_score = 0;

    let opponent_shape = match opponent {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => {
            panic!("AAAAAAA WHAT IS THIS SHAAPEEE: {}", opponent)
        }
    };

    let required_state = match required_state {
        'X' => State::Lose,
        'Y' => State::Draw,
        'Z' => State::Win,
        _ => {
            panic!("AAAAAAA WHAT IS THIS STATEEEE: {}", required_state)
        }
    };

    // Return the shape that is required to have the desired state in rock paper scissors
    let you_shape = match opponent_shape {
        Shape::Rock => match required_state {
            State::Lose => Shape::Scissors,
            State::Draw => Shape::Rock,
            State::Win => Shape::Paper,
        },
        Shape::Paper => match required_state {
            State::Lose => Shape::Rock,
            State::Draw => Shape::Paper,
            State::Win => Shape::Scissors,
        },
        Shape::Scissors => match required_state {
            State::Lose => Shape::Paper,
            State::Draw => Shape::Scissors,
            State::Win => Shape::Rock,
        },
    };

    // Calculate match score
    match you_shape {
        Shape::Rock => match opponent_shape {
            Shape::Rock => draw(&mut battle_score),
            Shape::Paper => loss(&mut battle_score),
            Shape::Scissors => win(&mut battle_score),
        },
        Shape::Paper => match opponent_shape {
            Shape::Rock => win(&mut battle_score),
            Shape::Paper => draw(&mut battle_score),
            Shape::Scissors => loss(&mut battle_score),
        },
        Shape::Scissors => match opponent_shape {
            Shape::Rock => loss(&mut battle_score),
            Shape::Paper => win(&mut battle_score),
            Shape::Scissors => draw(&mut battle_score),
        },
    }

    // Calculate shape score
    match you_shape {
        Shape::Rock => battle_score += 1,
        Shape::Paper => battle_score += 2,
        Shape::Scissors => battle_score += 3,
    }

    battle_score
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
