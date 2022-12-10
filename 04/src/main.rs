use core::fmt::Formatter;
use std::fmt::Display;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;

struct Range {
    begin: u32,
    end: u32,
}

impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.begin <= other.begin && other.end <= self.end
    }
    fn partially_contains(&self, other: &Range) -> bool {
        self.begin <= other.begin && other.begin <= self.end
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} - {}", self.begin, self.end)
    }
}

fn get_range(range_bgn: &str, range_end: &str) -> Result<Range, String> {
    let begin = match range_bgn.parse::<u32>() {
        Ok(x) => x,
        Err(e) => {
            return Err(format!(
                "Could not parse str as integer: {}, {}",
                range_bgn, e
            ))
        }
    };
    let end = match range_end.parse::<u32>() {
        Ok(x) => x,
        Err(e) => {
            return Err(format!(
                "Could not parse str as integer: {}, {}",
                range_bgn, e
            ))
        }
    };

    if begin > end {
        return Err(format!("Invalid range: {} - {}", begin, end));
    }

    Ok(Range { begin, end })
}

fn parse_ranges(line: &str) -> Result<(Range, Range), String> {
    let parts = line.split(',').collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(format!("Line did not have two parts: {}", line));
    }
    let range1 = parts[0].split('-').collect::<Vec<&str>>();
    if range1.len() != 2 {
        return Err(format!("Invalid range: {}, {:?}", parts[0], range1));
    }
    let range2 = parts[1].split('-').collect::<Vec<&str>>();
    if range2.len() != 2 {
        return Err(format!("Invalid range: {}", parts[1]));
    }
    let range1 = get_range(range1[0], range1[1])?;
    let range2 = get_range(range2[0], range2[1])?;
    Ok((range1, range2))
}

fn solve_part_one<R: Read>(f: R) -> Result<u32, String> {
    let mut total = 0;
    for line in BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };

        let (range1, range2) = parse_ranges(&line)?;
        if range1.fully_contains(&range2) || range2.fully_contains(&range1) {
            total += 1;
        }
    }

    Ok(total)
}

fn solve_part_two<R: Read>(f: R) -> Result<u32, String> {
    let mut total = 0;
    for line in BufReader::new(f).lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };

        let (range1, range2) = parse_ranges(&line)?;
        if range1.partially_contains(&range2) || range2.partially_contains(&range1) {
            total += 1;
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
    let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    let ans = solve_part_one(input.as_bytes())?;
    assert_eq!(ans, 2);

    let ans = solve_part_two(input.as_bytes())?;
    assert_eq!(ans, 4);

    Ok(())
}
