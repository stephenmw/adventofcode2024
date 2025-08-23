use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    println!("{:?}", data);
    bail!("not yet implemented")
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    bail!("not yet implemented")
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug)]
struct MachineBehavior {
    a: Point,
    b: Point,
    prize: Point,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<MachineBehavior>> {
        let specs = separated_list1(multispace1, machine_behavior());
        ws_all_consuming(specs).parse_complete(input)
    }

    fn machine_behavior<'a>()
    -> impl Parser<&'a str, Output = MachineBehavior, Error = nom::error::Error<&'a str>> {
        let button_a = ws_line(button('A'));
        let button_b = ws_line(button('B'));
        let prize = ws_line(prize());

        (button_a, button_b, prize).map(|(a, b, prize)| MachineBehavior { a, b, prize })
    }

    fn button<'a>(
        label: char,
    ) -> impl Parser<&'a str, Output = Point, Error = nom::error::Error<&'a str>> {
        let header = (tag("Button "), char(label), (char(':'), space0));

        let x_offset = preceded(tag("X+"), uint());
        let y_offset = preceded(tag("Y+"), uint());
        let offsets = separated_pair(x_offset, space_delimited(char(',')), y_offset);

        preceded(header, offsets).map(|(x, y)| Point { x, y })
    }

    fn prize<'a>() -> impl Parser<&'a str, Output = Point, Error = nom::error::Error<&'a str>> {
        let header = (tag("Prize:"), space0);

        let x_offset = preceded(tag("X="), uint());
        let y_offset = preceded(tag("Y="), uint());
        let offsets = separated_pair(x_offset, space_delimited(char(',')), y_offset);

        preceded(header, offsets).map(|(x, y)| Point { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "480")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
