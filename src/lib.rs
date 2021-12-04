mod ast;

use ast::{Agent, Expression};

pub trait Evaluate<T> {
    fn evaluate(self, state: T) -> T;
}

pub trait EvaluateMut<T> {
    type Output;

    fn evaluate_mut(&mut self, operation: T) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    color: u32,
}

impl Cell {
    pub fn new(color: u32) -> Self {
        Self { color }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { color: 0xFFFFFF }
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates(u32, u32);

#[derive(Debug)]
pub struct AgentState {
    vars: HashMap<String, ast::Primitive>,
    commands: Vec<ast::Command>,
    pc: u32,
    x: u32,
    y: u32,
    direction: ast::Direction,
    color: u32,
}

impl AgentState {
    fn new(
        commands: Vec<ast::Command>,
        pc: u32,
        x: u32,
        y: u32,
        direction: ast::Direction,
        color: u32,
    ) -> Self {
        Self {
            vars: HashMap::new(),
            commands,
            pc,
            x,
            y,
            direction,
            color,
        }
    }
}

impl EvaluateMut<ast::Command> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: ast::Command) -> Self::Output {
        match operation {
            ast::Command::SetVariable(id, _) => {
                // placeholder
                self.vars.insert(id, ast::Primitive::Boolean(true));
                Ok(vec![])
            }
            ast::Command::Face(dir) => {
                self.direction = dir;
                Ok(vec![])
            }
            ast::Command::Turn(rotations) => {
                let original_direction = self.direction as i32;
                self.direction = ast::Direction::from(original_direction + rotations);
                Ok(vec![])
            }
            ast::Command::Move(_) => todo!(),
            ast::Command::Goto(command) => {
                if (command as usize) < self.commands.len() {
                    self.pc = command;
                    Ok(vec![])
                } else {
                    Err("goto out of bounds".to_string())
                }
            }
            ast::Command::JumpTrue(_, _) => todo!(),
        }
    }
}

impl EvaluateMut<ast::Expression> for AgentState {
    type Output = Result<ast::Primitive, String>;

    fn evaluate_mut(&mut self, expr: ast::Expression) -> Self::Output {
        use ast::Primitive;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum BinaryOp {
            Add,
            Sub,
            Mul,
            Div,
        }

        fn evaluate_binary_op(
            agent: &mut AgentState,
            op: BinaryOp,
            lhs: Expression,
            rhs: Expression,
        ) -> Result<ast::Primitive, String> {
            let l = agent.evaluate_mut(lhs)?;
            let r = agent.evaluate_mut(rhs)?;

            match (l, r) {
                (Primitive::Integer(l), Primitive::Integer(r)) => match op {
                    BinaryOp::Add => Ok(Primitive::Integer(l + r)),
                    BinaryOp::Sub => Ok(Primitive::Integer(l - r)),
                    BinaryOp::Mul => Ok(Primitive::Integer(l * r)),
                    BinaryOp::Div => Ok(Primitive::Integer(l / r)),
                },
                _ => Err(format!("type mismatch ({:?}, {:?})", &l, &r)),
            }
        }

        match expr {
            Expression::Literal(lit) => Ok(lit),
            Expression::GetVariable(key) => self
                .vars
                .get(&key)
                .copied()
                .ok_or_else(|| format!("key [{}] undefined.", &key)),
            Expression::Equals(lhs, rhs) => {
                let l = self.evaluate_mut(*lhs)?;
                let r = self.evaluate_mut(*rhs)?;
                Ok(Primitive::Boolean(l == r))
            }
            Expression::Add(lhs, rhs) => evaluate_binary_op(self, BinaryOp::Add, *lhs, *rhs),
            Expression::Sub(lhs, rhs) => evaluate_binary_op(self, BinaryOp::Sub, *lhs, *rhs),
            Expression::Mul(lhs, rhs) => evaluate_binary_op(self, BinaryOp::Mul, *lhs, *rhs),
            Expression::Div(lhs, rhs) => evaluate_binary_op(self, BinaryOp::Div, *lhs, *rhs),
        }
    }
}

pub struct Board {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    agents: Vec<AgentState>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            agents: vec![],
            cells: vec![Cell::default(); (width * height) as usize],
            width,
            height,
        }
    }

    fn index_of(&self, x: u32, y: u32) -> u32 {
        x + y * self.width
    }

    pub fn update_cell(mut self, x: u32, y: u32, f: impl Fn(&mut Cell)) -> Self {
        self.update_cell_mut(x, y, f);
        self
    }

    pub fn update_cell_mut(&mut self, x: u32, y: u32, f: impl Fn(&mut Cell)) {
        let idx = self.index_of(x, y) as usize;
        let cell = self.cells.get_mut(idx).unwrap();
        f(cell);
    }
}

pub fn tick_agent(agent_state: &mut AgentState) -> Vec<Coordinates> {
    // TODO: implement interpreter here
    agent_state.pc += 1 as u32;
    todo!()
}

pub fn tick(mut board: Board) -> Board {
    for state in board.agents.iter_mut() {
        let touched = tick_agent(state);
        // TODO: set color for touched cells
    }

    board
}

pub fn tick_mut(_board: &mut Board) {}

// call from js
pub fn get_board_state(board: Board) -> Vec<u32> {
    board.cells.into_iter().map(|c| c.color).collect()
}
