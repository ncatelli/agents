#[derive(Debug)]
pub struct Program {
    agents: Vec<Agent>,
}

impl Program {
    pub fn new(agents: Vec<Agent>) -> Self {
        Self { agents }
    }

    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }
}

impl From<Program> for Vec<Agent> {
    fn from(program: Program) -> Self {
        program.agents
    }
}

#[derive(Debug, PartialEq)]
pub struct Agent {
    commands: Vec<Command>,
}

impl Agent {
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}

#[derive(Debug, Clone, PartialEq)]

pub enum Command {
    SetVariable(String, Expression),
    Face(Direction),
    Turn(i32),
    Move(u32),
    Goto(u32),
    JumpTrue(u32, Expression),
}

#[derive(Debug, Clone, PartialEq)]

pub enum Expression {
    Literal(Primitive),
    Equals(Box<Expression>, Box<Expression>),
    GetVariable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Direction {
    N = 0,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<i32> for Direction {
    fn from(src: i32) -> Self {
        match (src % 8).abs() {
            0 => Self::N,
            1 => Self::NE,
            2 => Self::E,
            3 => Self::SE,
            4 => Self::S,
            5 => Self::SW,
            6 => Self::W,
            7 => Self::NW,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Integer(i32),
    Boolean(bool),
}
