mod ast;
mod parser;

#[macro_use]
extern crate lazy_static;

use ast::Expression;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

/// Provides traits for evaluating a type onto a state, returning the modified
/// state.
pub trait Evaluate<T> {
    fn evaluate(self, state: T) -> T;
}

/// Provides traits for evaluating a given operation onto a mutable State type.
pub trait EvaluateMut<State> {
    type Output;

    fn evaluate_mut(&mut self, operation: State) -> Self::Output;
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

#[derive(Debug, Clone, PartialEq)]
pub struct AgentState {
    vars: HashMap<String, ast::Primitive>,
    commands: Vec<ast::Command>,
    pc: u32,
    coords: Coordinates,
    direction: ast::Direction,
    color: u32,
}

impl AgentState {
    #[allow(dead_code)]
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
            coords: Coordinates(x, y),
            direction,
            color,
        }
    }

    pub fn with_commands(mut self, commands: Vec<ast::Command>) -> Self {
        self.commands = commands;
        self
    }

    pub fn with_pc(mut self, pc: u32) -> Self {
        self.pc = pc;
        self
    }

    pub fn with_direction(mut self, direction: ast::Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coords = coordinates;
        self
    }
}

impl Default for AgentState {
    fn default() -> Self {
        Self {
            vars: Default::default(),
            commands: Default::default(),
            pc: Default::default(),
            coords: Coordinates(0, 0),
            direction: ast::Direction::S,
            color: Default::default(),
        }
    }
}

impl<M> Evaluate<AgentState> for M
where
    AgentState: EvaluateMut<M>,
{
    fn evaluate(self, mut state: AgentState) -> AgentState {
        state.evaluate_mut(self);
        state
    }
}

/// Sets a variable identified by the string to the primitive evaluated to by
/// the associated Expression on an associated agent..
#[derive(Debug, Clone, PartialEq)]
pub struct SetVariableCmd(pub String, pub Expression);

/// Defines the direction an agent should face.
#[derive(Debug, Clone, PartialEq)]
pub struct FaceCmd(pub ast::Direction);

/// Turns by a number of rotations where a positive number represents a
/// clockwise rotation and a negavite represents a counter-clockwise
/// rotation.
#[derive(Debug, Clone, PartialEq)]
pub struct TurnCmd(pub i32);

pub trait BoardInteraction {}

pub struct ReflectOnOverflow;

impl BoardInteraction for ReflectOnOverflow {}

pub struct WrapOnOverflow;

impl BoardInteraction for WrapOnOverflow {}

/// Move specifies the steps that an agent will move in the direction it is
/// facing.
#[derive(Debug, Clone, PartialEq)]
pub struct MoveCmd<BI: BoardInteraction>(pub u32, pub BI);

/// Goto jumps to the enclosed offset in an agents command list.
#[derive(Debug, Clone, PartialEq)]
pub struct GotoCmd(pub u32);

/// Like Goto, JumpTrue jumps to the enclosed offset if the passed conditional
/// expression evaluates to true.
#[derive(Debug, Clone, PartialEq)]
pub struct JumpTrueCmd(pub u32, pub Expression);

impl EvaluateMut<ast::Command> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: ast::Command) -> Self::Output {
        match operation {
            ast::Command::SetVariable(id, expr) => self.evaluate_mut(SetVariableCmd(id, expr)),
            ast::Command::Face(dir) => self.evaluate_mut(FaceCmd(dir)),
            ast::Command::Turn(rotations) => self.evaluate_mut(TurnCmd(rotations)),
            ast::Command::Move(steps) => self.evaluate_mut(MoveCmd(steps, WrapOnOverflow)),
            ast::Command::Goto(command) => self.evaluate_mut(GotoCmd(command)),
            ast::Command::JumpTrue(next, expr) => self.evaluate_mut(JumpTrueCmd(next, expr)),
        }
    }
}

impl EvaluateMut<SetVariableCmd> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: SetVariableCmd) -> Self::Output {
        use ast::Primitive;

        let SetVariableCmd(id, expr) = operation;
        let value = self.evaluate_mut(expr)?;

        match id.as_str() {
            "x" => match value {
                Primitive::Integer(i) if i > 0 => {
                    self.coords = Coordinates(i as u32, self.coords.y())
                }
                other => return Err(format!("invalid type [{:?}] for x", other)),
            },
            "y" => match value {
                Primitive::Integer(i) if i > 0 => {
                    self.coords = Coordinates(self.coords.x(), i as u32)
                }
                other => return Err(format!("invalid type [{:?}] for y", other)),
            },
            "color" => match value {
                Primitive::Integer(i) if i > 0 => self.color = i as u32,
                other => return Err(format!("invalid type [{:?}] for color", other)),
            },
            _ => {
                self.vars.insert(id, value);
            }
        };
        self.pc += 1;
        Ok(vec![])
    }
}

impl EvaluateMut<FaceCmd> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: FaceCmd) -> Self::Output {
        let FaceCmd(new_direction) = operation;

        self.direction = new_direction;
        self.pc += 1;
        Ok(vec![])
    }
}

impl EvaluateMut<TurnCmd> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: TurnCmd) -> Self::Output {
        let TurnCmd(rotations) = operation;

        let original_direction = self.direction as i32;
        self.direction = ast::Direction::from(original_direction + rotations);
        self.pc += 1;
        Ok(vec![])
    }
}

impl EvaluateMut<MoveCmd<WrapOnOverflow>> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: MoveCmd<WrapOnOverflow>) -> Self::Output {
        let MoveCmd::<WrapOnOverflow>(steps, _) = operation;
        let orientation = self.direction;
        let origin = self.coords;

        let touched_cells: Vec<Coordinates> = (0..=steps)
            .into_iter()
            .map(|offset| {
                const BW: i32 = BOARD_WIDTH as i32;
                const BH: i32 = BOARD_HEIGHT as i32;

                let Coordinates(x_u32, y_u32) = origin;
                let steps = offset as i32;
                let (x, y) = ((x_u32 as i32), (y_u32 as i32));

                let (offset_x, offset_y) = match orientation {
                    ast::Direction::N => (x, y - steps),
                    ast::Direction::NE => (x + steps, y - steps),
                    ast::Direction::NW => (x - steps, y - steps),
                    ast::Direction::E => (x + steps, y),
                    ast::Direction::SE => (x - steps, y + steps),
                    ast::Direction::S => (x, y + steps),
                    ast::Direction::SW => (x - steps, y + steps),
                    ast::Direction::W => (x - steps, y),
                };

                let (adjusted_x, adjusted_y) =
                    ((offset_x % BW + BW) % BW, (offset_y % BH + BH) % BH);

                Coordinates(adjusted_x as u32, adjusted_y as u32)
            })
            .collect();
        let end = touched_cells.last().copied().unwrap_or(origin);

        self.coords = Coordinates(end.x(), end.y());
        self.pc += 1;
        Ok(touched_cells)
    }
}

impl EvaluateMut<MoveCmd<ReflectOnOverflow>> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: MoveCmd<ReflectOnOverflow>) -> Self::Output {
        let MoveCmd::<ReflectOnOverflow>(steps, _) = operation;
        let orientation = self.direction;
        let origin = self.coords;

        let touched_cells: Vec<Coordinates> = (0..=steps)
            .into_iter()
            .map(|offset| {
                const BW: i32 = BOARD_WIDTH as i32;
                const BH: i32 = BOARD_HEIGHT as i32;

                let Coordinates(x_u32, y_u32) = origin;
                let steps = offset as i32;
                let (x, y) = ((x_u32 as i32), (y_u32 as i32));

                let (offset_x, offset_y) = match orientation {
                    ast::Direction::N => (x, y - steps),
                    ast::Direction::NE => (x + steps, y - steps),
                    ast::Direction::NW => (x - steps, y - steps),
                    ast::Direction::E => (x + steps, y),
                    ast::Direction::SE => (x - steps, y + steps),
                    ast::Direction::S => (x, y + steps),
                    ast::Direction::SW => (x - steps, y + steps),
                    ast::Direction::W => (x - steps, y),
                };

                let (adjusted_x, adjusted_y) = ((offset_x % BW + BW), (offset_y % BH + BH));

                Coordinates(adjusted_x as u32, adjusted_y as u32)
            })
            .collect();
        let end = touched_cells.last().copied().unwrap_or(origin);

        self.coords = Coordinates(end.x(), end.y());
        self.pc += 1;
        Ok(touched_cells)
    }
}

impl EvaluateMut<GotoCmd> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: GotoCmd) -> Self::Output {
        let GotoCmd(command) = operation;
        if (command as usize) < self.commands.len() {
            self.pc = command;
            Ok(vec![])
        } else {
            Err("goto out of bounds".to_string())
        }
    }
}

impl EvaluateMut<JumpTrueCmd> for AgentState {
    type Output = Result<Vec<Coordinates>, String>;

    fn evaluate_mut(&mut self, operation: JumpTrueCmd) -> Self::Output {
        use ast::Primitive;

        let JumpTrueCmd(next, condition) = operation;
        let prim = self.evaluate_mut(condition)?;

        match prim {
            pi @ Primitive::Integer(_) => Err(format!("condition is non-boolean: {:?}", &pi)),
            Primitive::Boolean(false) => todo!(),
            Primitive::Boolean(true) => {
                self.pc = next;
                Ok(vec![])
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
                .ok_or_else(|| format!("key [{}] undefined", &key)),
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
    agents: Vec<AgentState>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            agents: vec![],
            cells: vec![Cell::default(); (width * height) as usize],
            width,
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
    let command = agent_state
        .commands
        .get(agent_state.pc as usize)
        .cloned()
        .unwrap();
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

pub const BOARD_WIDTH: u32 = 50;
pub const BOARD_HEIGHT: u32 = 50;

lazy_static! {
    static ref BOARD: Mutex<Board> = Mutex::new(Board::new(BOARD_WIDTH, BOARD_HEIGHT));
}

#[wasm_bindgen]
pub fn run(source: &str) {
    let program = parser::parse(source).unwrap();
    for agent in <Vec<ast::Agent>>::from(program).into_iter() {
        let new_state = AgentState::default().with_commands(agent.commands);
        BOARD.lock().unwrap().agents.push(new_state);
    }
}

#[wasm_bindgen]
pub fn tick() -> Vec<u32> {
    tick_world(&mut BOARD.lock().unwrap());
    get_board_state(&BOARD.lock().unwrap())
}

pub fn get_board_state(board: &Board) -> Vec<u32> {
    board.cells.clone().into_iter().map(|c| c.color).collect()
}
