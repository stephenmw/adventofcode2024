use std::str::FromStr;

use nom::combinator::all_consuming;
use prelude::*;

pub mod prelude {
    pub use nom::{
        AsChar, IResult, Input, Mode, Parser,
        branch::alt,
        bytes::{is_a, tag, take},
        character::complete::{char, digit1, line_ending, multispace0, one_of, space0, space1},
        combinator::{eof, opt, recognize, value, verify},
        multi::{fold_many1, many1, separated_list1},
        sequence::{delimited, separated_pair, terminated},
    };

    #[allow(unused_imports)]
    pub use super::{complete, int, uint, ws_all_consuming, ws_line};
}

pub fn uint<'a, T: FromStr>() -> impl Parser<&'a str, Output = T, Error = nom::error::Error<&'a str>>
{
    digit1.map_res(|x: &str| x.parse())
}

#[allow(dead_code)]
pub fn int<'a, T: FromStr>() -> impl Parser<&'a str, Output = T, Error = nom::error::Error<&'a str>>
{
    let num = (opt(tag("-")), digit1);
    recognize(num).map_res(|x: &str| x.parse())
}

#[allow(dead_code)]
pub fn complete<I, P>(parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error>
where
    I: nom::Input,
    <I as Input>::Item: AsChar,
    P: nom::Parser<I>,
{
    terminated(parser, (multispace0, eof))
}

pub fn ws_all_consuming<I, P>(parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error>
where
    I: nom::Input,
    <I as Input>::Item: AsChar,
    P: nom::Parser<I>,
{
    all_consuming(delimited(multispace0, parser, multispace0))
}

pub fn ws_line<I, P>(parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error>
where
    I: nom::Input + nom::Compare<&'static str>,
    <I as Input>::Item: AsChar,
    P: nom::Parser<I>,
{
    let end_of_line = alt((line_ending, eof));
    delimited(space0, parser, (space0, end_of_line))
}
