const MY_INPUT: &str = include_str!("inputs/problem1.txt");

fn count_increases(depths: impl IntoIterator<Item = i32>) -> i32 {
    let mut last_depth: Option<i32> = None;
    let mut increases = 0;
    for depth in depths {
        if let Some(n) = last_depth {
            if n < depth {
                increases += 1
            }
        }
        last_depth = Some(depth);
    }
    increases
}

fn count_windowed_increases(depths: &[i32]) -> i32 {
    count_increases(depths.windows(3).map(|w| w.iter().sum()))
}

#[test]
fn test_sample_input() {
    let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, count_increases(depths));
    assert_eq!(5, count_windowed_increases(&depths))
}

fn parse_depths() -> Vec<i32> {
    MY_INPUT
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn part_one() -> i32 {
    count_increases(parse_depths())
}

fn part_two() -> i32 {
    count_windowed_increases(&parse_depths())
}

fn main() {
    println!("Increases {}", part_one());
    println!("Windowed increases {}", part_two());
}
