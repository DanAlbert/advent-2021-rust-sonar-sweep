pub struct Submarine {
    pub x: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Submarine {
    pub(crate) fn aim(&mut self, delta: i32) {
        self.aim += delta;
    }

    pub(crate) fn dive(&mut self, delta: i32) {
        self.depth += delta;
    }

    pub(crate) fn ascend(&mut self, delta: i32) {
        self.depth -= delta;
    }

    pub(crate) fn forward(&mut self, delta: i32) {
        self.x += delta;
    }

    pub fn new() -> Submarine {
        Submarine {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Submarine::new()
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

#[test]
fn test_aim() {
    let mut sub = Submarine::new();
    assert_eq!(sub.aim, 0);

    sub.aim(1);
    assert_eq!(sub.aim, 1);

    sub.aim(2);
    assert_eq!(sub.aim, 3);

    sub.aim(-3);
    assert_eq!(sub.aim, 0);
}
