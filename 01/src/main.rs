use std::io::BufRead;
use std::io::Read;
use std::io::Seek;

fn solve_part_one<R: Read>(f: R) -> Result<u32, String> {
    let mut greatest = 0;
    let mut current_total = 0;
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        if line.is_empty() {
            if current_total > greatest {
                greatest = current_total
            }
            current_total = 0;
        } else {
            let num = match line.parse::<u32>() {
                Ok(x) => x,
                Err(e) => return Err(format!("Could not parse \"{}\" as i32: {}", line, e)),
            };
            current_total += num;
        }
    }
    Ok(greatest)
}

fn insert_ordered<T: PartialOrd + Copy + core::fmt::Debug>(list: &mut [T], item: T) {
    // Could be improved with a binary search
    for (index, value) in list.iter().enumerate() {
        if item > *value {
            for i in (index + 1..list.len()).rev() {
                list[i] = list[i - 1];
            }
            list[index] = item;
            break;
        }
    }
}

fn solve_part_two<R: Read>(f: R) -> Result<u32, String> {
    let mut greatest = [0, 0, 0];
    let mut current_total = 0;
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        if line.is_empty() {
            if current_total > greatest[2] {
                insert_ordered(&mut greatest, current_total);
            }
            current_total = 0;
        } else {
            let num = match line.parse::<u32>() {
                Ok(x) => x,
                Err(e) => return Err(format!("Could not parse \"{}\" as i32: {}", line, e)),
            };
            current_total += num;
        }
    }
    if current_total > greatest[2] {
        insert_ordered(&mut greatest, current_total);
    }

    let greatest_total = greatest
        .iter()
        .fold(0, |accumulator, element| accumulator + element);
    Ok(greatest_total)
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
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    let ans = solve_part_one(input.as_bytes())?;
    assert_eq!(ans, 24_000);

    let ans = solve_part_two(input.as_bytes())?;
    assert_eq!(ans, 45_000);

    Ok(())
}
