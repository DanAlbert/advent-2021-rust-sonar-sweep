use crate::command;

pub struct Submarine {
    pub x: i32,
    pub depth: i32,
}

impl Submarine {
    fn dive(&mut self, delta: i32) {
        self.depth += delta;
    }

    fn ascend(&mut self, delta: i32) {
        self.depth -= delta;
    }

    fn forward(&mut self, delta: i32) {
        self.x += delta;
    }

    pub fn new() -> Submarine {
        Submarine { x: 0, depth: 0 }
    }

    pub fn act_on(&mut self, commands: impl IntoIterator<Item = command::Command>) {
        for command in commands {
            match command.action {
                command::Action::UP => self.ascend(command.value),
                command::Action::DOWN => self.dive(command.value),
                command::Action::FORWARD => self.forward(command.value),
            }
        }
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_dive() {
    let mut sub = Submarine::new();
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 0);

    sub.dive(1);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 1);

    sub.dive(2);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 3);

    sub.dive(-3);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 0);
}

#[test]
fn test_ascend() {
    let mut sub = Submarine::new();
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 0);

    sub.ascend(1);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, -1);

    sub.ascend(2);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, -3);

    sub.ascend(-3);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 0);
}

#[test]
fn test_forward() {
    let mut sub = Submarine::new();
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 0);

    sub.forward(1);
    assert_eq!(sub.x, 1);
    assert_eq!(sub.depth, 0);

    sub.forward(2);
    assert_eq!(sub.x, 3);
    assert_eq!(sub.depth, 0);

    sub.forward(-3);
    assert_eq!(sub.x, 0);
    assert_eq!(sub.depth, 0);
}
