// use itertools::Itertools;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

pub(crate) fn solve_day18() -> f64 {
    let input_file = include_str!("../resources/day18.txt");
    input_file
        .lines()
        .map(|l| {
            let parse_result = Calculator::parse(Rule::calculation, l).unwrap();
            eval(parse_result)
        })
        .sum()
}

pub(crate) fn solve_day18_part2() -> f64 {
    let input_file = include_str!("../resources/day18.txt");
    input_file
        .lines()
        .map(|l| {
            let parse_result = Calculator::parse(Rule::calculation, l).unwrap();
            eval_2(parse_result)
        })
        .sum()
}

#[derive(Parser)]
#[grammar = "days/day18.pest"] // relative to src
struct Calculator;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left)
                | Operator::new(subtract, Left)
                | Operator::new(multiply, Left)
                | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

lazy_static! {
    static ref PREC_CLIMBER_2: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(power, Right),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}

fn eval_2(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER_2.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval_2(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}
