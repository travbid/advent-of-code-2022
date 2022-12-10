use std::io::BufRead;
use std::io::Read;
use std::io::Seek;

fn solve_part_one<R: Read>(f: R) -> Result<u32, String> {
    let mut total = 0;
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        let chars = line.chars().collect::<Vec<char>>();
        if chars.len() % 2 != 0 {
            return Err(format!(
                "Rucksack does not have an even number of items: {}",
                line
            ));
        }
        let (compartment1, compartment2) = chars.split_at(chars.len() / 2);
        let compartment1 = compartment1
            .to_owned()
            .into_iter()
            .collect::<std::collections::HashSet<char>>();
        for item in compartment2 {
            if compartment1.contains(item) {
                let priority = if *item >= 'a' && *item <= 'z' {
                    *item as u32 - 96
                } else if *item >= 'A' && *item <= 'Z' {
                    *item as u32 + 27 - 65
                } else {
                    return Err(format!("Invalid char: {}", item));
                };
                total += priority;
                break;
            }
        }
    }

    Ok(total)
}

fn solve_part_two<R: Read>(f: R) -> Result<u32, String> {
    let mut lines = Vec::new();
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        lines.push(line.chars().collect::<Vec<char>>())
    }

    let mut total = 0;
    for chunk in lines.chunks(3) {
        if chunk.len() != 3 {
            return Err(format!("lines is not multiple of 3"));
        }
        let first = chunk[0]
            .to_owned()
            .into_iter()
            .collect::<std::collections::HashSet<char>>();
        let second = chunk[1]
            .to_owned()
            .into_iter()
            .collect::<std::collections::HashSet<char>>();
        for item in &chunk[2] {
            if first.contains(&item) && second.contains(&item) {
                let priority = if *item >= 'a' && *item <= 'z' {
                    *item as u32 - 96
                } else if *item >= 'A' && *item <= 'Z' {
                    *item as u32 + 27 - 65
                } else {
                    return Err(format!("Invalid char: {}", item));
                };
                total += priority;
                break;
            }
        }
    }

    Ok(total)
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
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    let ans = solve_part_one(input.as_bytes())?;
    assert_eq!(ans, 157);

    let ans = solve_part_two(input.as_bytes())?;
    assert_eq!(ans, 70);

    Ok(())
}
