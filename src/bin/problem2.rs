use advent::command::Command;
use advent::navigator::{AimingNav, Navigator, PartOneNav};
use advent::submarine::Submarine;

const MY_INPUT: &str = include_str!("inputs/problem2.txt");

fn destination_product(sub: &Submarine) -> i32 {
    sub.x * sub.depth
}

#[test]
fn test_sample_input() {
    let commands = Command::parse_each(vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ])
    .unwrap();

    {
        let mut sub = Submarine::new();
        assert_eq!(0, destination_product(&sub));

        PartOneNav::default().act_on_each(&mut sub, commands.iter());
        assert_eq!(150, destination_product(&sub));
    }

    {
        let mut sub = Submarine::new();
        AimingNav::default().act_on_each(&mut sub, commands.iter());
        assert_eq!(15, sub.x);
        assert_eq!(60, sub.depth);
        assert_eq!(900, destination_product(&sub));
    }
}

fn main() {
    let commands = Command::parse_each(MY_INPUT.lines()).unwrap();

    {
        let mut sub = Submarine::new();
        PartOneNav::default().act_on_each(&mut sub, commands.iter());
        println!("Part 1 result: {}", destination_product(&sub));
    }

    {
        let mut sub = Submarine::new();
        AimingNav::default().act_on_each(&mut sub, commands.iter());
        println!("Part 2 result: {}", destination_product(&sub));
    }
}
