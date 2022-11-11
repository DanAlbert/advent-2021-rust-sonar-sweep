use crate::command::{Action, Command};
use crate::submarine::Submarine;

pub trait Navigator {
    fn act_on(&self, submarine: &mut Submarine, command: &Command);

    fn act_on_each<'a>(
        &self,
        submarine: &mut Submarine,
        commands: impl Iterator<Item = &'a Command>,
    ) {
        commands.for_each(|c| self.act_on(submarine, c))
    }
}

#[derive(Default)]
pub struct PartOneNav;

impl PartOneNav {
    pub fn new() -> PartOneNav {
        PartOneNav {}
    }
}

impl Navigator for PartOneNav {
    fn act_on(&self, submarine: &mut Submarine, command: &Command) {
        match command.action {
            Action::UP => submarine.ascend(command.value),
            Action::DOWN => submarine.dive(command.value),
            Action::FORWARD => submarine.forward(command.value),
        }
    }
}

#[derive(Default)]
pub struct AimingNav;

impl AimingNav {
    pub fn new() -> AimingNav {
        AimingNav {}
    }
}

impl Navigator for AimingNav {
    fn act_on(&self, submarine: &mut Submarine, command: &Command) {
        match command.action {
            Action::UP => submarine.aim(-command.value),
            Action::DOWN => submarine.aim(command.value),
            Action::FORWARD => {
                submarine.forward(command.value);
                submarine.dive(submarine.aim * command.value);
            }
        }
    }
}
