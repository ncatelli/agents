use crate::{AgentState, Evaluate};

#[test]
fn should_generate_expected_new_coordinates_for_move() {
    use crate::ast::Command;
    use crate::Coordinates;

    assert_eq!(
        AgentState::default()
            .with_commands(vec![Command::Move(5)])
            .with_pc(1)
            .with_coordinates(Coordinates(0, 5)),
        Command::Move(5).evaluate(AgentState::default().with_commands(vec![Command::Move(5)]))
    );

    assert_eq!(
        AgentState::default()
            .with_commands(vec![Command::Move(51)])
            .with_pc(1)
            .with_coordinates(Coordinates(0, 1)),
        Command::Move(51).evaluate(AgentState::default().with_commands(vec![Command::Move(51)]))
    );

    assert_eq!(
        AgentState::default()
            .with_commands(vec![Command::Move(49)])
            .with_direction(crate::ast::Direction::N)
            .with_pc(1)
            .with_coordinates(Coordinates(0, 1)),
        Command::Move(49).evaluate(
            AgentState::default()
                .with_direction(crate::ast::Direction::N)
                .with_commands(vec![Command::Move(49)])
        )
    );

    assert_eq!(
        AgentState::default()
            .with_commands(vec![Command::Move(49)])
            .with_direction(crate::ast::Direction::W)
            .with_pc(1)
            .with_coordinates(Coordinates(1, 0)),
        Command::Move(49).evaluate(
            AgentState::default()
                .with_direction(crate::ast::Direction::W)
                .with_commands(vec![Command::Move(49)])
        )
    );

    assert_eq!(
        AgentState::default()
            .with_commands(vec![Command::Move(51)])
            .with_direction(crate::ast::Direction::E)
            .with_pc(1)
            .with_coordinates(Coordinates(1, 0)),
        Command::Move(51).evaluate(
            AgentState::default()
                .with_direction(crate::ast::Direction::E)
                .with_commands(vec![Command::Move(51)])
        )
    )
}
