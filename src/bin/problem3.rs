const MY_INPUT: &str = include_str!("inputs/problem3.txt");

fn parse_log<const N: usize>(log: &str) -> Result<[bool; N], String> {
    if log.is_empty() {
        return Err("Results are undefined for an empty log".to_string());
    }

    let mut bit_sums = [0i32; N];
    for (lineno, line) in log.lines().enumerate() {
        if line.len() != N {
            return Err(format!(
                "Line {} has invalid length {} (must be {})",
                lineno,
                line.len(),
                N
            ));
        }
        for (idx, char) in line.chars().enumerate() {
            match char {
                '0' => bit_sums[idx] -= 1,
                '1' => bit_sums[idx] += 1,
                _ => {
                    return Err(format!(
                        "Invalid bit in log at line {} column {}: {}",
                        lineno, idx, char
                    ))
                }
            }
        }
    }

    if let Some(bad_bit) = bit_sums.iter().position(|v| v == &0) {
        Err(format!(
            "Results are undefined: column {} has no most common bit",
            bad_bit
        ))
    } else {
        Ok(bit_sums.map(|v| {
            println!();
            v > 0
        }))
    }
}

#[test]
fn test_parse_log() {
    assert_eq!(
        Err("Results are undefined for an empty log".to_string()),
        parse_log::<5>("")
    );
    assert_eq!(
        Err("Line 0 has invalid length 4 (must be 5)".to_string()),
        parse_log::<5>("0000")
    );
    assert_eq!(
        Err("Invalid bit in log at line 0 column 4: a".to_string()),
        parse_log::<5>("0000a")
    );
    assert_eq!(Ok([false, false, false, false, false]), parse_log("00000"));
    assert_eq!(Ok([true, false, true, false, true]), parse_log("10101"));
    assert_eq!(
        Err("Results are undefined: column 0 has no most common bit".to_string()),
        parse_log::<5>("10000\n00000")
    );
    assert_eq!(Ok([true]), parse_log("1\n1"));
    assert_eq!(
        Ok([true, false, false, false, true]),
        parse_log("00001\n10000\n11111")
    );
}

fn bit_array_to_int<const N: usize>(bits: &[bool; N]) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + ((*v as u32) << i))
}

fn gamma_rate<const N: usize>(most_common_bits: &[bool; N]) -> u32 {
    bit_array_to_int(most_common_bits)
}

#[test]
fn test_gamma_rate() {
    assert_eq!(0, gamma_rate(&[false, false, false, false, false]));
    assert_eq!(0b1, gamma_rate(&[false, false, false, false, true]));
    assert_eq!(0b11111, gamma_rate(&[true, true, true, true, true]));
}

fn epsilon_rate<const N: usize>(most_common_bits: &[bool; N]) -> u32 {
    bit_array_to_int(&most_common_bits.map(|v| !v))
}

#[test]
fn test_epsilon_rate() {
    assert_eq!(0b11111, epsilon_rate(&[false, false, false, false, false]));
    assert_eq!(0b11110, epsilon_rate(&[false, false, false, false, true]));
    assert_eq!(0, epsilon_rate(&[true, true, true, true, true]));
}

fn power_consumption_from_log<const N: usize>(log: &str) -> Result<u32, String> {
    let most_common_bits = parse_log::<N>(log)?;
    Ok(gamma_rate(&most_common_bits) * epsilon_rate(&most_common_bits))
}

#[test]
fn test_sample_input() {
    let log = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
    let most_common_bits = parse_log::<5>(log).unwrap();
    assert_eq!([true, false, true, true, false], most_common_bits);
    assert_eq!(22, gamma_rate(&most_common_bits));
    assert_eq!(9, epsilon_rate(&most_common_bits));
    assert_eq!(Ok(198), power_consumption_from_log::<5>(log));
}

fn main() {
    println!(
        "Power consumption: {}",
        power_consumption_from_log::<12>(MY_INPUT).unwrap()
    );
}
