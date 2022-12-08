fn main() {
    let input = include_str!("day8.txt");
    println!("Part 1: {}", count_visible(input));
    println!("Part 2: {}", most_scenic_tree(input));
}

fn count_visible(input: &str) -> usize {
    let visibility_map = create_visibility_map(input);
    visibility_map.iter().filter(|&&v| v == 1).count()
}

fn parse_heights(input: &str) -> (Vec<u8>, usize, usize) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let heights: Vec<u8> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        // Calculate the height by subtracting the ASCII value of '0'
        .map(|c| c as u8 - b'0')
        .collect();
    (heights, rows, cols)
}

fn create_visibility_map(input: &str) -> Vec<u8> {
    let (heights, rows, cols) = parse_heights(input);
    let mut visibility_map = vec![0; heights.len()];

    // Apply visibility works from the top and left so we will
    // go through it twice, reversing the input the second time,
    // to get the visibility from all four directions.
    let forward = heights.iter().copied().zip(visibility_map.iter_mut());
    apply_visibility(forward, rows, cols);
    let backward = heights
        .iter()
        .copied()
        .rev()
        .zip(visibility_map.iter_mut().rev());
    apply_visibility(backward, rows, cols);

    visibility_map
}

fn apply_visibility<'a>(
    height_and_visibility: impl Iterator<Item = (u8, &'a mut u8)>,
    rows: usize,
    cols: usize,
) {
    // Keep track of the max values we've seen in 2 directions
    let mut max_from_top = vec![0; cols];
    let mut max_from_left = vec![0; rows];

    for (index, (height, visibility)) in height_and_visibility.enumerate() {
        let row = index / cols;
        let col = index % cols;

        if height > max_from_top[col] {
            max_from_top[col] = height;
            *visibility = 1;
        }

        if height > max_from_left[row] {
            max_from_left[row] = height;
            *visibility = 1;
        }

        if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
            *visibility = 1;
        }
    }
}

fn most_scenic_tree(input: &str) -> usize {
    let (heights, rows, cols) = parse_heights(input);
    let mut scenery_scores = vec![1; heights.len()];

    let trees: Vec<Vec<u8>> = heights.chunks(cols).map(|c| c.to_vec()).collect();

    for i in 0..heights.len() {
        let row = i / cols;
        let col = i % cols;

        if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
            scenery_scores[i] = 0;
            continue;
        }
        let height = trees[row][col];

        // Naive solution

        let mut above = 0;
        for r in (0..row).rev() {
            above += 1;
            if trees[r][col] >= height {
                break;
            }
        }

        let mut below = 0;
        for r in row + 1..rows {
            below += 1;
            if trees[r][col] >= height {
                break;
            }
        }

        let mut left = 0;
        for c in (0..col).rev() {
            left += 1;
            if trees[row][c] >= height {
                break;
            }
        }

        let mut right = 0;
        for c in col + 1..cols {
            right += 1;
            if trees[row][c] >= height {
                break;
            }
        }

        scenery_scores[i] = above * below * left * right;
    }

    scenery_scores.into_iter().max().unwrap()
}

fn print_map(values: &[u8], cols: usize) {
    for (index, value) in values.iter().enumerate() {
        if index % cols == 0 {
            println!();
        }
        print!("{:2} ", value);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn counting_visible() {
        assert_eq!(count_visible(EXAMPLE), 21);
    }

    #[test]
    fn finding_scenic_tree() {
        assert_eq!(most_scenic_tree(EXAMPLE), 8);
    }
}
