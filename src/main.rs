const SEA_FLOOR: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

fn main() {
    let mut last_depth: Option<i32> = None;
    let mut increases = 0;
    for depth in SEA_FLOOR {
        if let Some(n) = last_depth {
            if n < *depth {
                increases += 1
            }
        }
        last_depth = Some(*depth);
    }
    println!("Increases {}", increases);
}
