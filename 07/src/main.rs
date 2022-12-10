use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Seek;

struct Dir {
    subdirs: BTreeMap<String, Dir>,
    files: BTreeMap<String, u32>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            subdirs: BTreeMap::new(),
            files: BTreeMap::new(),
        }
    }
    fn all_sizes(&self) -> (u32, Vec<u32>) {
        let mut self_size = 0;
        let mut child_sizes = Vec::new();
        for (_, file_size) in &self.files {
            self_size += file_size;
        }
        for (_, d) in &self.subdirs {
            let (subdir_size, subdir_children) = d.all_sizes();
            child_sizes.extend(subdir_children);
            child_sizes.push(subdir_size);
            self_size += subdir_size;
        }
        (self_size, child_sizes)
    }
}

fn read_dir<R: Read>(
    line_iter: &mut Lines<BufReader<R>>,
    file_tree: &mut Dir,
) -> Result<(), String> {
    if let Some(ls) = line_iter.next() {
        let ls = match ls {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        if ls != "$ ls" {
            return Err(format!("Expected \"$ ls\" command. Found {}", ls));
        }
        while let Some(next) = line_iter.next() {
            let line = match next {
                Ok(x) => x,
                Err(e) => return Err(format!("Could not read line: {}", e)),
            };
            if line.starts_with("dir ") {
                // Do nothing
            } else if line.starts_with("$ cd") {
                let dir_name = line.chars().skip(5).collect::<String>();
                if dir_name == ".." {
                    return Ok(());
                }
                // Don't try to re-add a directory that's already been visited
                if !file_tree.subdirs.contains_key(&dir_name) {
                    let mut new_dir = Dir::new();
                    read_dir(line_iter, &mut new_dir)?;
                    file_tree.subdirs.insert(dir_name, new_dir);
                }
            } else if line.starts_with("$") {
                return Err(format!("Unexpected command: {}", line));
            } else {
                let size_str = line.chars().take_while(|c| *c != ' ').collect::<String>();
                let size = match size_str.parse::<u32>() {
                    Ok(x) => x,
                    Err(e) => return Err(format!("Could not parse as u32: {}, {}", line, e)),
                };
                let name = line[size_str.len() + 1..].to_string();
                file_tree.files.insert(name, size);
            }
        }
    }

    Ok(())
}

fn solve_part_one<R: Read>(f: R) -> Result<u32, String> {
    let mut line_iter = BufReader::new(f).lines();
    if let Some(next) = line_iter.next() {
        let line = match next {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        if !line.starts_with("$ cd") {
            return Err(format!("First line was not \"$ cd /\""));
        }
        let mut file_tree = Dir::new();
        read_dir(&mut line_iter, &mut file_tree)?;
        let (_, child_sizes) = file_tree.all_sizes();
        let total = child_sizes.iter().fold(0, |accum, size| {
            if *size <= 100_000 {
                accum + *size
            } else {
                accum
            }
        });
        return Ok(total);
    } else {
        Err(format!("Could not read first line"))
    }
}

fn solve_part_two<R: Read>(f: R) -> Result<u32, String> {
    let mut line_iter = BufReader::new(f).lines();
    if let Some(next) = line_iter.next() {
        let line = match next {
            Ok(x) => x,
            Err(e) => return Err(format!("Could not read line: {}", e)),
        };
        if !line.starts_with("$ cd") {
            return Err(format!("First line was not \"$ cd /\""));
        }
        let mut file_tree = Dir::new();
        read_dir(&mut line_iter, &mut file_tree)?;
        let (root_size, mut child_sizes) = file_tree.all_sizes();
        child_sizes.sort();
        const MAX_SPACE_USAGE: u32 = 70_000_000 - 30_000_000;
        for size in child_sizes {
            if root_size - size <= MAX_SPACE_USAGE {
                return Ok(size);
            }
        }
        return Err(format!("Did not find a large enough directory to delete"));
    } else {
        Err(format!("Could not read first line"))
    }
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
    let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    let ans = solve_part_one(input.as_bytes())?;
    assert_eq!(ans, 95_437);

    let ans = solve_part_two(input.as_bytes())?;
    assert_eq!(ans, 24_933_642);

    Ok(())
}
