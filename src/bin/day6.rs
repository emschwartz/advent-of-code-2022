use std::collections::HashSet;

fn main() {
    let input = include_str!("./day6.txt");
    println!("Part 1: {}", find_marker(input, 4).unwrap());
    println!("Part 2: {}", find_marker(input, 14).unwrap());
}

fn find_marker(input: &str, marker_len: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(marker_len)
        .position(|window| HashSet::<u8>::from_iter(window.iter().copied()).len() == marker_len)
        .map(|i| i + marker_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_packet_header() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker(input, 4), Some(7));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(input, 4), Some(5));

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(input, 4), Some(6));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(input, 4), Some(10));

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(input, 4), Some(11));
    }

    #[test]
    fn find_message_header() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker(input, 14), Some(19));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(input, 14), Some(29));
    }
}
