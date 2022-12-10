use std::io::BufRead;
use std::io::Read;
use std::io::Seek;

fn solve_part_one<R: Read>(f: R) -> Result<u32, String> {
    let mut score = 0;
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        let split: Vec<&str> = line.split_whitespace().collect();
        if split.len() != 2 {
            return Err(format!("Line does not contain 2 elements: {}", line));
        }
        if split[1] == "X" {
            score += 1;
            if split[0] == "A" {
                score += 3;
            } else if split[0] == "B" {
                score += 0;
            } else if split[0] == "C" {
                score += 6;
            } else {
                return Err(format!("Not A, B, or C: {}", split[0]));
            }
        } else if split[1] == "Y" {
            score += 2;
            if split[0] == "A" {
                score += 6;
            } else if split[0] == "B" {
                score += 3;
            } else if split[0] == "C" {
                score += 0;
            } else {
                return Err(format!("Not A, B, or C: {}", split[0]));
            }
        } else if split[1] == "Z" {
            score += 3;
            if split[0] == "A" {
                score += 0;
            } else if split[0] == "B" {
                score += 6;
            } else if split[0] == "C" {
                score += 3;
            } else {
                return Err(format!("Not A, B, or C: {}", split[0]));
            }
        } else {
            return Err(format!("Not X, Y, or Z: {}", split[1]));
        }
    }
    Ok(score)
}

fn solve_part_two<R: Read>(f: R) -> Result<u32, String> {
    let mut score = 0;
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        let split: Vec<&str> = line.split_whitespace().collect();
        if split.len() != 2 {
            return Err(format!("Line does not contain 2 elements: {}", line));
        }
        if split[1] == "X" {
            // Lose
            score += 0;
            if split[0] == "A" {
                // Rock
                score += 3; //Scissors
            } else if split[0] == "B" {
                // Paper
                score += 1; //Rock
            } else if split[0] == "C" {
                // Scissors
                score += 2; //Paper
            } else {
                return Err(format!("Not A, B, or C: {}", split[0]));
            }
        } else if split[1] == "Y" {
            //Draw
            score += 3;
            if split[0] == "A" {
                score += 1;
            } else if split[0] == "B" {
                score += 2;
            } else if split[0] == "C" {
                score += 3;
            } else {
                return Err(format!("Not A, B, or C: {}", split[0]));
            }
        } else if split[1] == "Z" {
            //Win
            score += 6;
            if split[0] == "A" {
                score += 2;
            } else if split[0] == "B" {
                score += 3;
            } else if split[0] == "C" {
                score += 1;
            } else {
                return Err(format!("Not A, B, or C: {}", split[0]));
            }
        } else {
            return Err(format!("Not X, Y, or Z: {}", split[1]));
        }
    }
    Ok(score)
}

fn main() -> Result<(), String> {
    let mut f = match std::fs::File::open("./input.txt") {
        Ok(f) => f,
        Err(e) => return Err(format!("Error opening input.txt: {}", e)),
    };

    println!("Part One: {}", solve_part_one(&f)?);
    if let Err(e) = f.rewind() {
        return Err(format!("Could not rewind file: {}", e));
    }
    println!("Part Two: {}", solve_part_two(&f)?);
    Ok(())
}

#[test]
fn test_input() -> Result<(), String> {
    let input = r#"A Y
B X
C Z
"#;

    let ans = solve_part_one(input.as_bytes())?;
    assert_eq!(ans, 15);

    let ans = solve_part_two(input.as_bytes())?;
    assert_eq!(ans, 12);

    Ok(())
}
