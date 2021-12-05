use std::fmt::format;
use std::slice::SliceIndex;

use parcel::parsers::character::*;
use parcel::prelude::v1::*;

use crate::ast;

#[derive(Debug)]
pub enum CommandOrLabel {
    Label(String),
    Command(ParsedCommand),
}

#[derive(Debug, PartialEq)]
pub enum ParsedCommand {
    SetVariable(String, ast::Expression),
    Face(ast::Direction),
    Turn(i32),
    Move(u32),
    Goto(String),
    JumpTrue(String, ast::Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    UndefinedLabel(String),
    Unspecified(String),
}

#[allow(dead_code)]
pub fn parse(input: &[(usize, char)]) -> Result<ast::Program, ParseErr> {
    parcel::one_or_more(agent())
        .map(ast::Program::new)
        .parse(input)
        .map_err(ParseErr::Unspecified)
        .and_then(|ms| match ms {
            MatchStatus::Match {
                span: _,
                remainder: _,
                inner,
            } => Ok(inner),
            MatchStatus::NoMatch(_) => {
                Err(ParseErr::Unspecified("not a valid program".to_string()))
            }
        })
}

fn agent<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], ast::Agent> {
    use std::collections::HashMap;
    move |input: &'a [(usize, char)]| {
        let (span, remainder, command_or_labels) = expect_str("agent ")
            .and_then(|_| parcel::join(label(), parcel::zero_or_more(command_or_label())))
            .parse(input)
            .map_err(ParseErr::Unspecified)
            .and_then(|ms| match ms {
                MatchStatus::Match {
                    span,
                    remainder,
                    inner,
                } => Ok((span, remainder, inner.1)),
                MatchStatus::NoMatch(_) => {
                    Err(ParseErr::Unspecified("not a valid program".to_string()))
                }
            })
            .map_err(|e| format!("{:?}", e))?;

        let labels =
            command_or_labels
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut labels, (idx, col)| match col {
                    CommandOrLabel::Label(id) => {
                        labels.insert(id.clone(), idx);
                        labels
                    }
                    CommandOrLabel::Command(_) => labels,
                });
        let commands: Vec<ParsedCommand> = command_or_labels
            .into_iter()
            .map(|col| match col {
                CommandOrLabel::Label(_) => None,
                CommandOrLabel::Command(pc) => Some(pc),
            })
            .flatten()
            .collect();

        commands
            .into_iter()
            .map(|pc| match pc {
                ParsedCommand::SetVariable(id, expr) => Ok(ast::Command::SetVariable(id, expr)),
                ParsedCommand::Face(direction) => Ok(ast::Command::Face(direction)),
                ParsedCommand::Turn(rotations) => Ok(ast::Command::Turn(rotations)),
                ParsedCommand::Move(distance) => Ok(ast::Command::Move(distance)),
                ParsedCommand::Goto(label) => {
                    if let Some(&offset) = labels.get(&label) {
                        Ok(ast::Command::Goto(offset as u32))
                    } else {
                        Err(ParseErr::UndefinedLabel(label))
                    }
                }
                ParsedCommand::JumpTrue(label, expr) => {
                    if let Some(&offset) = labels.get(&label) {
                        Ok(ast::Command::JumpTrue(offset as u32, expr))
                    } else {
                        Err(ParseErr::UndefinedLabel(label))
                    }
                }
            })
            .collect::<Result<Vec<ast::Command>, ParseErr>>()
            .map_err(|e| format!("{:?}", e))
            .map(|commands| parcel::MatchStatus::Match {
                span,
                remainder,
                inner: ast::Agent::new(commands),
            })
    }
}

fn command_or_label<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], CommandOrLabel> {
    parcel::one_or_more(non_newline_whitespace()).and_then(|_| {
        command()
            .map(CommandOrLabel::Command)
            .or(|| label().map(CommandOrLabel::Label))
    })
}

fn label<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], String> {
    parcel::left(parcel::join(
        identifier(),
        expect_character(':').and_then(|_| newline_terminated_whitespace()),
    ))
}

fn command<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], ParsedCommand> {
    parcel::left(parcel::join(
        face_command().or(move_command).or(goto_command),
        newline_terminated_whitespace(),
    ))
}

pub fn move_command<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], ParsedCommand> {
    expect_str("move ")
        .and_then(|_| dec_u32())
        .map(ParsedCommand::Move)
}

fn face_command<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], ParsedCommand> {
    expect_str("face ")
        .and_then(|_| direction())
        .map(ParsedCommand::Face)
}

fn goto_command<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], ParsedCommand> {
    expect_str("goto ")
        .and_then(|_| identifier())
        .map(ParsedCommand::Goto)
}

fn direction<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], ast::Direction> {
    use ast::Direction;

    parcel::one_of(vec![
        expect_str("NW"),
        expect_str("nw"),
        expect_str("NE"),
        expect_str("ne"),
        expect_str("SW"),
        expect_str("sw"),
        expect_str("SE"),
        expect_str("se"),
        expect_str("N"),
        expect_str("n"),
        expect_str("W"),
        expect_str("w"),
        expect_str("S"),
        expect_str("s"),
        expect_str("E"),
        expect_str("e"),
    ])
    .map(|direction| match direction.as_str() {
        "N" | "n" => Direction::N,
        "E" | "e" => Direction::E,
        "S" | "s" => Direction::S,
        "W" | "w" => Direction::W,
        "NW" | "nw" => Direction::NW,
        "NE" | "ne" => Direction::NE,
        "SW" | "sw" => Direction::SW,
        "SE" | "se" => Direction::SE,
        _ => unreachable!(),
    })
}

fn identifier<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], String> {
    parcel::one_or_more(alphabetic().or(|| digit(10)).or(|| expect_character('_')))
        .map(|chars| chars.into_iter().collect())
}

fn newline_terminated_whitespace<'a>() -> impl parcel::Parser<'a, &'a [(usize, char)], char> {
    parcel::zero_or_more(space().or(tab)).and_then(|_| newline())
}

fn dec_u32<'a>() -> impl Parser<'a, &'a [(usize, char)], u32> {
    move |input: &'a [(usize, char)]| {
        let preparsed_input = input;
        let res = parcel::one_or_more(digit(10))
            .map(|digits| {
                let vd: String = digits.into_iter().collect();
                vd.parse::<u32>()
            })
            .parse(input);

        match res {
            Ok(MatchStatus::Match {
                span,
                remainder,
                inner: Ok(u),
            }) => Ok(MatchStatus::Match {
                span,
                remainder,
                inner: u,
            }),

            Ok(MatchStatus::Match {
                span: _,
                remainder: _,
                inner: Err(_),
            }) => Ok(MatchStatus::NoMatch(preparsed_input)),

            Ok(MatchStatus::NoMatch(remainder)) => Ok(MatchStatus::NoMatch(remainder)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    const MIN_TEST_PROGRAM: &str = "agent red_agent:
    loop:
        face NW
        move 10
agent blue_agent:
    loop:
        face NE
        move 20

";

    const _TEST_PROGRAM: &str = "agent red_agent:
    set color = 0xff0000
    set x = 0
    set y = 0
    set direction = 0
    set a = 0
    loop:
        set a = a + 1
        move 10
        jump to set_zero if a is 1
        jump to set_one if a is 0
        goto exit
    set_one:
        set a = 1
        face NW
        goto loop
    set_zero:
        set a = 0
        turn -2
        goto loop
    exit:
agent blue_agent:
    set color = 0x00ff00
    set x = 0
    set y = 0
    set direction = 0
    set a = 0
    loop:
        set a = a + 1
        move 10
        jump to set_zero if a is 1
        jump to set_one if a is 0
        goto exit
    set_one:
        set a 1
        face NW
        goto loop
    set_zero:
        set a 0
        turn -2
        goto loop
    exit:";

    #[test]
    fn should_parse_agent_expression() {
        let input: Vec<(usize, char)> = MIN_TEST_PROGRAM.chars().enumerate().collect();
        let res = crate::parser::parse(&input);

        assert_eq!(Ok(2), res.map(|program| program.agents().len()))
    }
}
