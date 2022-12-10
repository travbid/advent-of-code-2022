use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Seek;

fn read_stacks<R: Read>(lines: &mut Lines<BufReader<R>>) -> Result<Vec<Vec<char>>, String> {
    let mut stacks = Vec::new();

    for line in lines {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };

        if line.is_empty() {
            break;
        }

        let line_chars = line.chars().collect::<Vec<char>>();
        if line_chars[1] == '1' {
            continue;
        }
        let num_stacks = ((line_chars.len() + 3) / 4) + 1;
        if num_stacks > stacks.len() {
            stacks.resize(num_stacks, Vec::new());
        }
        for (i, chunk) in line_chars.chunks(4).enumerate() {
            let c = chunk[1];
            if c != ' ' {
                if c < 'A' || c > 'Z' {
                    return Err(format!("Invalid char: {}", c));
                }
                stacks[i + 1].push(c);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
        for item in stack {
            print!("{} ", item);
        }
        println!();
    }
    Ok(stacks)
}

fn read_line(line: &str) -> Result<(u32, usize, usize), String> {
    let mut line = &line[5..];
    for (i, c) in line.char_indices() {
        if c == ' ' {
            let num_crates = match line[..i].parse::<u32>() {
                Ok(x) => x,
                Err(e) => return Err(format!("Could not parse crate number: {}, {}", line, e)),
            };

            line = &line[i + 6..];

            for (i, c) in line.char_indices() {
                if c == ' ' {
                    let stack_from = match line[..i].parse::<usize>() {
                        Ok(x) => x,
                        Err(e) => {
                            return Err(format!("Could not parse stack number: {}, {}", line, e))
                        }
                    };

                    line = &line[i + 4..];

                    let stack_to = match line.parse::<usize>() {
                        Ok(x) => x,
                        Err(e) => {
                            return Err(format!("Could not parse stack number: {}, {}", line, e))
                        }
                    };

                    return Ok((num_crates, stack_from, stack_to));
                }
            }

            break;
        }
    }

    Err(format!("Did not find all required info for line: {}", line))
}

fn solve_part_one<R: Read>(f: R) -> Result<String, String> {
    let mut lines = BufReader::new(f).lines();
    let mut stacks = read_stacks(&mut lines)?;

    for line in lines {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        let (num_crates, stack_from, stack_to) = read_line(&line)?;
        for _ in 0..num_crates {
            let top = match stacks[stack_from].last() {
                Some(x) => *x,
                None => return Err(format!("Error getting last element of stack: {}", line)),
            };
            stacks[stack_to].push(top);
            stacks[stack_from].pop();
        }
    }

    let mut result = String::new();
    for stack in stacks.iter().skip(1) {
        match stack.last() {
            Some(c) => result.push(*c),
            None => result.push(' '),
        }
    }
    Ok(result)
}

fn solve_part_two<R: Read>(f: R) -> Result<String, String> {
    let mut lines = BufReader::new(f).lines();
    let mut stacks = read_stacks(&mut lines)?;

    for line in lines {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        let (num_crates, stack_from, stack_to) = read_line(&line)?;

        let from_stack = stacks[stack_from].clone();
        stacks[stack_to].extend(&from_stack[from_stack.len() - num_crates as usize..]);
        for _ in 0..num_crates {
            stacks[stack_from].pop();
        }
    }

    let mut result = String::new();
    for stack in stacks.iter().skip(1) {
        match stack.last() {
            Some(c) => result.push(*c),
            None => result.push(' '),
        }
    }
    Ok(result)
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
    let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    let ans = solve_part_one(input.as_bytes())?;
    assert_eq!(ans, "CMZ");

    let ans = solve_part_two(input.as_bytes())?;
    assert_eq!(ans, "MCD");

    Ok(())
}
