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

/// All valid Command variants that an Agent can perform.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Sets a variable identified by the string to the primitive evaluated to by
    /// the associated Expression.
    SetVariable(String, Expression),
    /// Defines the direction an agent should face.
    Face(Direction),
    /// Turns by a number of rotations where a positive number represents a
    /// clockwise rotation and a negavite represents a counter-clockwise
    /// rotation.
    Turn(i32),
    /// Move specifies the steps that an agent will move in the direction it is
    /// facing.
    Move(u32),
    /// Goto jumps to the enclosed offset in an agents command list.
    Goto(u32),
    /// Like Goto, JumpTrue jumps to the enclosed offset if the passed conditional
    /// expression evaluates to true.
    JumpTrue(u32, Expression),
}

/// Sets a variable identified by the string to the primitive evaluated to by
/// the associated Expression on an associated agent..
#[derive(Debug, Clone, PartialEq)]
pub struct SetVariableCmd(pub String, pub Expression);

/// Defines the direction an agent should face.
#[derive(Debug, Clone, PartialEq)]
pub struct FaceCmd(pub Direction);

/// Turns by a number of rotations where a positive number represents a
/// clockwise rotation and a negavite represents a counter-clockwise
/// rotation.
#[derive(Debug, Clone, PartialEq)]
pub struct TurnCmd(pub i32);

/// Move specifies the steps that an agent will move in the direction it is
/// facing.
#[derive(Debug, Clone, PartialEq)]
pub struct MoveCmd(pub u32);

/// Goto jumps to the enclosed offset in an agents command list.
#[derive(Debug, Clone, PartialEq)]
pub struct GotoCmd(pub u32);

/// Like Goto, JumpTrue jumps to the enclosed offset if the passed conditional
/// expression evaluates to true.
#[derive(Debug, Clone, PartialEq)]
pub struct JumpTrueCmd(pub u32, pub Expression);

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

/// Represents the cardinal directions that an agent can face.
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

/// Valid primititive types for the language, currently only integer or boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Integer(i32),
    Boolean(bool),
}
