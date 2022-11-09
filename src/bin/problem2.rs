use advent::command::Command;
use advent::submarine::Submarine;

const MY_INPUT: &str = include_str!("inputs/problem2.txt");

fn destination_product(sub: &Submarine) -> i32 {
    sub.x * sub.depth
}

#[test]
fn test_destination_product() {
    let commands = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let mut sub = Submarine::new();
    assert_eq!(0, destination_product(&sub));

    sub.act_on(Command::parse_each(commands).unwrap());
    assert_eq!(150, destination_product(&sub));
}

fn main() {
    let mut sub = Submarine::new();
    let commands = Command::parse_each(MY_INPUT.lines()).unwrap();
    sub.act_on(commands);
    println!("{}", destination_product(&sub));
}
