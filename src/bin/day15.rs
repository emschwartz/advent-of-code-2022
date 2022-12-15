fn main() {
    let input = include_str!("day15.txt");
    println!("Part 1: {}", spots_without_sensors(input, 2000000).len());
    println!(
        "Part 2: {}",
        distress_signal_tuning_frequency(input, 4000000)
    );
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    nearest_beacon: Point,
    distance_to_beacon: u32,
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at x=").unwrap();
            let (sensor_x, line) = line.split_once(',').unwrap();
            let line = line.strip_prefix(" y=").unwrap();
            let (sensor_y, line) = line.split_once(':').unwrap();
            let line = line.strip_prefix(" closest beacon is at x=").unwrap();
            let (beacon_x, line) = line.split_once(',').unwrap();
            let beacon_y = line.strip_prefix(" y=").unwrap();
            let sensor = Point {
                x: sensor_x.parse().unwrap(),
                y: sensor_y.parse().unwrap(),
            };
            let beacon = Point {
                x: beacon_x.parse().unwrap(),
                y: beacon_y.parse().unwrap(),
            };

            Sensor {
                distance_to_beacon: sensor.manhattan_distance(&beacon),
                location: sensor,
                nearest_beacon: beacon,
            }
        })
        .collect()
}

fn spots_without_sensors(input: &str, row: i32) -> Vec<i32> {
    let sensors = parse_input(input);
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    for sensor in &sensors {
        min_x = min_x.min(sensor.location.x - sensor.distance_to_beacon as i32);
        max_x = max_x.max(sensor.location.x + sensor.distance_to_beacon as i32);
    }

    let mut spots_without_sensors = Vec::new();
    let mut x = min_x;
    while x <= max_x {
        let point = Point { x, y: row };
        for sensor in &sensors {
            if sensor.nearest_beacon == point {
                break;
            }
            let within_range =
                sensor.location.manhattan_distance(&point) <= sensor.distance_to_beacon;
            if within_range {
                spots_without_sensors.push(x);
                break;
            }
        }
        x += 1;
    }
    spots_without_sensors
}

fn distress_signal_tuning_frequency(input: &str, max_coord: usize) -> u64 {
    let sensors = parse_input(input);

    for row in 0..max_coord {
        let mut ranges = Vec::new();

        // For each sensor, calculate the range of x values in this row that the
        // sensor "covers"
        for sensor in &sensors {
            let max_dist_from_x: i32 =
                sensor.distance_to_beacon as i32 - sensor.location.y.abs_diff(row as i32) as i32;
            if max_dist_from_x < 0 {
                continue;
            }
            let range = (
                (sensor.location.x - max_dist_from_x).max(0),
                (sensor.location.x + max_dist_from_x).min(max_coord as i32),
            );
            if max_dist_from_x >= 0 {
                ranges.push(range);
            }
        }

        // Combind the ranges to find the spot that isn't covered by the ranges
        ranges.sort_by_key(|(ref min, _)| *min);
        let mut ranges = ranges.into_iter();
        let (min, mut max) = ranges.next().unwrap();
        for (from, to) in ranges {
            if from <= max {
                max = max.max(to);
            } else if from == max + 2 {
                // Found the spot
                return (max as u64 + 1) * 4000000 + row as u64;
            } else {
                panic!("{min} {max} {from} {to}")
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn example_part1() {
        assert_eq!(spots_without_sensors(EXAMPLE, 10).len(), 26);
    }

    #[test]
    fn example_part2() {
        assert_eq!(distress_signal_tuning_frequency(EXAMPLE, 20), 56000011);
    }
}
