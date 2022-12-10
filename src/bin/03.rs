use std::collections::{HashMap, HashSet};

struct Letter {
    location: i32,
}

// Should be O(1)
pub fn part_one(input: &str) -> Option<u32> {
    // Link letters to values, idk if this is the optimal way
    let mut letters = ('a'..='z').into_iter().collect::<Vec<char>>();
    let mut upper_letters = ('A'..='Z').into_iter().collect::<Vec<char>>();

    letters.append(&mut upper_letters);
    let mut letters_hashmap: HashMap<&char, i32> = HashMap::new();
    for (index, letter) in letters.iter().enumerate() {
        letters_hashmap.insert(letter, index as i32);
    }

    let mut duplicate_letters = Vec::new();

    // Get rows
    for row in input.split("\n") {
        let (letters_left, letters_right) = row.split_at(row.chars().count() / 2);

        let mut right_letters_hashmap: HashMap<char, i32> = HashMap::new();

        for letter_right in letters_right.chars() {
            if right_letters_hashmap.contains_key(&letter_right) {
                // panic!("HUHHHHHHHH: {} {}", letter_right, row)
            }

            right_letters_hashmap.insert(letter_right, 1);
        }

        for letter_left in letters_left.chars() {
            if right_letters_hashmap.contains_key(&letter_left) {
                // println!("Found one")
                duplicate_letters.push(letter_left);
            }
        }
    }

    let mut total_score: i32 = 0;

    for letter in duplicate_letters {
        let value = letters_hashmap.get(&letter).unwrap();
        total_score += *value;
    }

    println!("{:?}", total_score);

    return Some(total_score as u32);
    //  ('a'..='z').into_iter().collect::<Vec<char>>()

    // Track which letters exist multiple times

    // // Put every letter in a hashmap,
    // let mut hashmapRightLetters: HashMap<&str, i32> = HashMap::new();

    // let letter = "k";

    // let result = letters.contains_key(letter);
    // if !result {
    //     letters.insert(letter, 1);
    // } else {
    //     // Found a second one
    // }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
