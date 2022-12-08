use std::collections::HashMap;
use std::path::{Path, PathBuf};

const HARD_DRIVE_SIZE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;

fn main() {
    let input = include_str!("./day7.txt");
    println!("Part 1: {}", total_size_of_small_directories(input));
    println!("Part 2: {}", smallest_directory_to_delete(input));
}

fn total_size_of_small_directories(input: &str) -> u32 {
    let directories = disk_usage(input);
    directories.values().filter(|size| **size < 100000).sum()
}

fn smallest_directory_to_delete(input: &str) -> u32 {
    let directories = disk_usage(input);
    let mut sizes: Vec<u32> = directories.values().copied().collect();
    sizes.sort();

    let total_used = directories[Path::new("/")];
    let available_space = HARD_DRIVE_SIZE - total_used;
    let to_delete = UPDATE_SIZE.saturating_sub(available_space);

    sizes.into_iter().find(|size| *size > to_delete).unwrap()
}

fn disk_usage(input: &str) -> HashMap<PathBuf, u32> {
    let mut path = PathBuf::new();
    let mut directories = HashMap::new();

    for line in input.lines() {
        if let Some(directory) = line.strip_prefix("$ cd ") {
            if directory == ".." {
                path.pop();
            } else {
                path.push(directory);
                directories.insert(path.clone(), 0);
            }
        } else if let Some(size) = line
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<u32>().ok())
        {
            // Add the file size to the current directory and all its ancestors.
            for directory in path.ancestors() {
                *directories.get_mut(directory).unwrap() += size;
            }
        }
        // Ignore other lines
    }

    directories
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "$ cd /
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
7214296 k";

    #[test]
    fn mapping_directories() {
        let directories = disk_usage(EXAMPLE);
        assert_eq!(directories[Path::new("/")], 48381165);
        assert_eq!(directories[Path::new("/a/e")], 584);
        assert_eq!(directories[Path::new("/a")], 94853);
        assert_eq!(directories[Path::new("/d")], 24933642);
    }

    #[test]
    fn directory_to_delete() {
        assert_eq!(smallest_directory_to_delete(EXAMPLE), 24933642);
    }
}
