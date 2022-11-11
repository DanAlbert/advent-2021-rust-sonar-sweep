const MY_INPUT: &str = include_str!("inputs/problem3.txt");

fn parse_log_lines<'a, const N: usize>(
    lines: impl IntoIterator<Item = &'a str>,
    replace_ambiguous: bool,
) -> Result<[bool; N], String> {
    let mut iter = lines.into_iter().peekable();
    if iter.peek().is_none() {
        return Err("Results are undefined for an empty log".to_string());
    }

    let mut bit_sums = [0i32; N];
    for (lineno, line) in iter.enumerate() {
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

    if replace_ambiguous {
        bit_sums = bit_sums.map(|v| if v == 0 && replace_ambiguous { 1 } else { v })
    }

    if let Some(bad_bit) = bit_sums.iter().position(|v| v == &0) {
        Err(format!(
            "Results are undefined: column {} has no most common bit",
            bad_bit
        ))
    } else {
        Ok(bit_sums.map(|v| v > 0))
    }
}

fn parse_log<const N: usize>(log: &str) -> Result<[bool; N], String> {
    parse_log_lines(log.lines(), false)
}

struct LifeSupport {
    o2: u32,
    co2: u32,
}

#[derive(Clone, Copy, PartialEq)]
enum LifeSupportMode {
    O2,
    CO2,
}

fn life_support<const N: usize>(log: &str) -> Result<LifeSupport, String> {
    Ok(LifeSupport {
        o2: _life_support::<N>(&log.lines().collect(), LifeSupportMode::O2, 0)?,
        co2: _life_support::<N>(&log.lines().collect(), LifeSupportMode::CO2, 0)?,
    })
}

fn _life_support<const N: usize>(
    potentials: &Vec<&str>,
    mode: LifeSupportMode,
    idx: usize,
) -> Result<u32, String> {
    if potentials.is_empty() {
        return Err("Found no valid life support value".to_string());
    }
    if potentials.len() == 1 {
        return Ok(u32::from_str_radix(potentials[0], 2).unwrap());
    }
    if idx >= N {
        return Err("Found no unique life support value".to_string());
    }

    _life_support::<N>(
        &filter_life_support::<N>(potentials, mode, idx)?,
        mode,
        idx + 1,
    )
}

fn filter_life_support<'a, const N: usize>(
    potentials: &[&'a str],
    mode: LifeSupportMode,
    idx: usize,
) -> Result<Vec<&'a str>, String> {
    let most_common_bits = parse_log_lines::<N>(potentials.iter().cloned(), true)?;
    let seek = match mode {
        LifeSupportMode::O2 => {
            if most_common_bits[idx] {
                '1'
            } else {
                '0'
            }
        }
        LifeSupportMode::CO2 => {
            if most_common_bits[idx] {
                '0'
            } else {
                '1'
            }
        }
    };
    Ok(potentials
        .iter()
        .filter(|&s| s.chars().nth(idx).unwrap() == seek)
        .cloned()
        .collect())
}

#[test]
fn test_filter_life_support() {
    assert_eq!(
        vec!["11110", "10110", "10111", "10101", "11100", "10000", "11001"],
        filter_life_support::<5>(
            &[
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ],
            LifeSupportMode::O2,
            0
        )
        .unwrap()
    );

    assert_eq!(
        vec!["10110", "10111", "10101", "10000"],
        filter_life_support::<5>(
            &["11110", "10110", "10111", "10101", "11100", "10000", "11001"],
            LifeSupportMode::O2,
            1
        )
        .unwrap()
    );

    assert_eq!(
        vec!["10110", "10111", "10101"],
        filter_life_support::<5>(
            &["10110", "10111", "10101", "10000"],
            LifeSupportMode::O2,
            2
        )
        .unwrap()
    );

    assert_eq!(
        vec!["10110", "10111"],
        filter_life_support::<5>(&["10110", "10111", "10101"], LifeSupportMode::O2, 3).unwrap()
    );

    assert_eq!(
        vec!["10111"],
        filter_life_support::<5>(&["10110", "10111"], LifeSupportMode::O2, 4).unwrap()
    );

    assert_eq!(
        vec!["00100", "01111", "00111", "00010", "01010"],
        filter_life_support::<5>(
            &[
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ],
            LifeSupportMode::CO2,
            0
        )
        .unwrap()
    );

    assert_eq!(
        vec!["01111", "01010"],
        filter_life_support::<5>(
            &["00100", "01111", "00111", "00010", "01010"],
            LifeSupportMode::CO2,
            1
        )
        .unwrap()
    );

    assert_eq!(
        vec!["01010"],
        filter_life_support::<5>(&["01111", "01010"], LifeSupportMode::CO2, 2).unwrap()
    );
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

fn life_support_rating_from_log<const N: usize>(log: &str) -> Result<u32, String> {
    let l = life_support::<N>(log)?;
    Ok(l.o2 * l.co2)
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
    let life_support = life_support::<5>(log).unwrap();
    assert_eq!(23, life_support.o2);
    assert_eq!(10, life_support.co2);
}

fn main() {
    println!(
        "Power consumption: {}",
        power_consumption_from_log::<12>(MY_INPUT).unwrap()
    );
    println!(
        "Life support: {}",
        life_support_rating_from_log::<12>(MY_INPUT).unwrap()
    );
}
