const MY_INPUT: &str = include_str!("input");

fn count_increases(depths: &[i32]) -> i32 {
    let mut last_depth: Option<i32> = None;
    let mut increases = 0;
    for depth in depths {
        if let Some(n) = last_depth {
            if n < *depth {
                increases += 1
            }
        }
        last_depth = Some(*depth);
    }
    increases
}

#[test]
fn test_count_increases() {
    assert_eq!(
        7,
        count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    )
}

fn main() {
    let floor: Vec<i32> = MY_INPUT
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!("Increases {}", count_increases(&floor));
}
