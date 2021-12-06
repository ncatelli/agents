use crate::{AgentState, Evaluate};

macro_rules! generate_move_command_assertion {
    ($behavior:expr, $steps:literal => $x:literal, $y:literal) => {
        assert_eq!(
            AgentState::default()
                .with_commands(vec![Command::Move($steps)])
                .with_pc(1)
                .with_coordinates(Coordinates($x, $y)),
            MoveCmd($behavior, $steps)
                .evaluate(AgentState::default().with_commands(vec![Command::Move($steps)]))
        );
    };
    ($($behavior:expr, $steps:literal => $x:literal, $y:literal,)*) => {
        $(
        generate_move_command_assertion!($behavior, $steps=> $x, $y);
        )*
    };
    ($behavior:expr, $steps:literal to $direction:expr => $x:literal, $y:literal) => {
        assert_eq!(
            AgentState::default()
                .with_commands(vec![Command::Move($steps)])
                .with_pc(1)
                .with_coordinates(Coordinates($x, $y))
                .with_direction($direction),
            MoveCmd($behavior, $steps)
                .evaluate(AgentState::default().with_direction($direction).with_commands(vec![Command::Move($steps)]))
        );
    };
    ($($behavior:expr, $steps:literal to $direction:expr => $x:literal, $y:literal,)*) => {
        $(
        generate_move_command_assertion!($behavior, $steps to $direction => $x, $y);
        )*
    };
}

#[test]
fn should_generate_expected_new_coordinates_for_move_when_wrapped() {
    use crate::ast::{Command, Direction};
    use crate::{Coordinates, MoveCmd, WrapOnOverflow};

    generate_move_command_assertion!(
        WrapOnOverflow, 5 => 0, 5,
        WrapOnOverflow, 51 => 0, 1,
    );

    generate_move_command_assertion!(
        WrapOnOverflow, 49 to Direction::N => 0, 1,
        WrapOnOverflow, 49 to Direction::W => 1, 0,
        WrapOnOverflow, 51 to Direction::E => 1, 0,
    );
}

#[test]
fn should_generate_expected_new_coordinates_for_move_when_reflected() {
    use crate::ast::Command;
    use crate::{Coordinates, MoveCmd, ReflectOnOverflow};

    generate_move_command_assertion!(
        ReflectOnOverflow, 5 => 0, 5,
    );
}
