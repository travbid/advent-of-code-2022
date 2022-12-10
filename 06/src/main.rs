use std::io::Read;
use std::io::Seek;

fn solve_part_one(input: &str) -> usize {
    let mut prev_chars = input.chars().take(4).collect::<Vec<char>>();
    for c in input.chars().skip(4) {
        let mut set = std::collections::HashSet::new();
        set.insert(prev_chars[prev_chars.len() - 1]);
        set.insert(prev_chars[prev_chars.len() - 2]);
        set.insert(prev_chars[prev_chars.len() - 3]);
        set.insert(prev_chars[prev_chars.len() - 4]);
        if set.len() == 4 {
            return prev_chars.len();
        }
        prev_chars.push(c);
    }

    prev_chars.len()
}

fn solve_part_two(input: &str) -> usize {
    const MARKER_LEN: usize = 14;
    let mut prev_chars = input.chars().take(MARKER_LEN).collect::<Vec<char>>();
    for c in input.chars().skip(MARKER_LEN) {
        let mut set = std::collections::HashSet::new();
        for i in 1..=MARKER_LEN {
            set.insert(prev_chars[prev_chars.len() - i]);
        }
        if set.len() == MARKER_LEN {
            return prev_chars.len();
        }
        prev_chars.push(c);
    }

    prev_chars.len()
}

fn main() -> Result<(), String> {
    let mut f = match std::fs::File::open("./input.txt") {
        Ok(f) => f,
        Err(e) => return Err(format!("Error opening input.txt: {}", e)),
    };

    let mut buf = String::new();
    if let Err(e) = f.read_to_string(&mut buf) {
        return Err(format!("Error reading file to string: {}", e));
    }

    println!("Part One: {}", solve_part_one(&buf));
    if let Err(e) = f.rewind() {
        return Err(format!("Could not rewind file: {}", e));
    }
    println!("Part Two: {}", solve_part_two(&buf));
    Ok(())
}

#[test]
fn test_input() -> Result<(), String> {
    let inputs = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    for input in inputs {
        let ans = solve_part_one(input.0);
        assert_eq!(ans, input.1);
    }

    for input in inputs {
        let ans = solve_part_two(input.0);
        assert_eq!(ans, input.2);
    }

    Ok(())
}
