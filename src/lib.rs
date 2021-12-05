mod ast;
mod parser;

#[macro_use]
extern crate lazy_static;

use ast::{Expression, Command};
use wasm_bindgen::prelude::*;
use std::sync::Mutex;

pub trait Evaluate<T> {
    fn evaluate(self, state: T) -> T;
}

pub trait EvaluateMut<T> {
    type Output;

    fn evaluate_mut(&mut self, operation: T) -> Self::Output;
}

#[wasm_bindgen]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates(pub u32, pub u32);

impl Coordinates {
    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}

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
        use ast::Primitive;
        match operation {
            ast::Command::SetVariable(id, expr) => {
                let value = self.evaluate_mut(expr)?;
                self.vars.insert(id, value);
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
            ast::Command::Move(steps) => match self.direction {
                ast::Direction::N => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x, self.y + offset))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.y = end.y();
                    Ok(touched_cells)
                }
                ast::Direction::S => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x, self.y - offset))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.y = end.y();
                    Ok(touched_cells)
                }
                ast::Direction::E => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x + offset, self.y))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.x = end.x();
                    Ok(touched_cells)
                }
                ast::Direction::W => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x - offset, self.y))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.x = end.x();
                    Ok(touched_cells)
                }
                ast::Direction::NE => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x + offset, self.y + offset))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.x = end.x();
                    self.y = end.y();
                    Ok(touched_cells)
                }
                ast::Direction::SE => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x + offset, self.y - offset))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.x = end.x();
                    self.y = end.y();
                    Ok(touched_cells)
                }
                ast::Direction::SW => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x - offset, self.y - offset))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.x = end.x();
                    self.y = end.y();
                    Ok(touched_cells)
                }
                ast::Direction::NW => {
                    let touched_cells: Vec<Coordinates> = (0..=steps)
                        .into_iter()
                        .map(|offset| Coordinates(self.x - offset, self.y + offset))
                        .collect();
                    let end = touched_cells
                        .last()
                        .copied()
                        .unwrap_or(Coordinates(self.x, self.y));

                    self.x = end.x();
                    self.y = end.y();
                    Ok(touched_cells)
                }
            },
            ast::Command::Goto(command) => {
                if (command as usize) < self.commands.len() {
                    self.pc = command;
                    Ok(vec![])
                } else {
                    Err("goto out of bounds".to_string())
                }
            }
            ast::Command::JumpTrue(next, expr) => {
                let prim = self.evaluate_mut(expr)?;
                match prim {
                    pi @ Primitive::Integer(_) => {
                        Err(format!("condition is non-boolean: {:?}", &pi))
                    }
                    Primitive::Boolean(false) => todo!(),
                    Primitive::Boolean(true) => {
                        self.pc = next;
                        Ok(vec![])
                    }
                }
            }
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
    // TODO: change pc here
    let command = agent_state.commands.get(agent_state.pc as usize).cloned().unwrap();
    agent_state.evaluate_mut(command).unwrap()
}

pub fn tick_world(board: &mut Board) {
    let mut all_touched: HashMap<Coordinates, u32> = HashMap::new();
    for state in board.agents.iter_mut() {
        let touched = tick_agent(state);
        for t in touched.iter() {
            all_touched.insert(Coordinates(t.x(), t.y()), state.color);
        }
    }

    for (coord, color) in all_touched {
        board.update_cell_mut(coord.x(), coord.y(), |cell| cell.color = color);
    }
}

lazy_static! {
    static ref BOARD: Mutex<Board> = {
        let mut board = Board::new(50, 50);
        let commands = vec![
            Command::Move(2),
            Command::Goto(0)
        ];
        board.agents.push(AgentState::new(commands.clone(), 0, 4, 4, ast::Direction::N, 0xff0000));
        board.agents.push(AgentState::new(commands.clone(), 0, 6, 4, ast::Direction::E, 0x0000ff));
        Mutex::new(board)
    };
}

#[wasm_bindgen]
pub fn tick() -> Vec<u32> {
    tick_world(&mut BOARD.lock().unwrap());
    get_board_state(&BOARD.lock().unwrap())
}

// call from js
pub fn get_board_state(board: &Board) -> Vec<u32> {
    board.cells.clone().into_iter().map(|c| c.color).collect()
}
