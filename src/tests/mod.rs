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
    ($behavior:expr, $steps:literal to $direction:expr => $x:literal, $y:literal with $new_direction:expr) => {
        assert_eq!(
            AgentState::default()
                .with_commands(vec![Command::Move($steps)])
                .with_pc(1)
                .with_coordinates(Coordinates($x, $y))
                .with_direction($new_direction),
            MoveCmd($behavior, $steps)
                .evaluate(AgentState::default().with_direction($direction).with_commands(vec![Command::Move($steps)]))
        );
    };
    ($($behavior:expr, $steps:literal to $direction:expr => $x:literal, $y:literal with $new_direction:expr,)*) => {
        $(
        generate_move_command_assertion!($behavior, $steps to $direction => $x, $y with $new_direction);
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
    use crate::ast::{Command, Direction};
    use crate::{Coordinates, MoveCmd, ReflectOnOverflow};

    generate_move_command_assertion!(
        ReflectOnOverflow, 6 to Direction::N => 0, 6 with Direction::S,
        ReflectOnOverflow, 51 to Direction::S => 0, 49 with Direction::N,
        ReflectOnOverflow, 6 to Direction::W => 6, 0 with Direction::E,
        ReflectOnOverflow, 51 to Direction::E => 49, 0 with Direction::W,
        ReflectOnOverflow, 6 to Direction::NW => 6, 6 with Direction::SE,
        ReflectOnOverflow, 51 to Direction::SE => 49, 49 with Direction::NW,
    );
}
