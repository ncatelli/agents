use crate::{AgentState, Evaluate};

#[test]
fn should_generate_expected_new_coordinates_for_move() {
    let agent_state = AgentState::default().with_commands(vec![crate::ast::Command::Move(5)]);

    assert_eq!(
        AgentState::default()
            .with_commands(vec![crate::ast::Command::Move(5)])
            .with_pc(1)
            .with_coordinates(crate::Coordinates(0, 5)),
        crate::ast::Command::Move(5).evaluate(agent_state)
    )
}
