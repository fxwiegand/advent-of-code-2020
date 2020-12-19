use itertools::Itertools;
use pest::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) fn solve_day19() -> u64 {
    // make_pest_file();
    let input_file = include_str!("../resources/day19.txt");
    let tmp = input_file.split("\n\n").collect_vec()[1];
    let mut count = 0;
    for line in tmp.lines() {
        match StringParser::parse(Rule::main, line) {
            Ok(_) => count += 1,
            _ => println!("Didn't match"),
        }
    }

    count
}

#[derive(Parser)]
#[grammar = "days/day19.pest"] // relative to src
struct StringParser;

#[derive(Debug, Clone)]
struct MyRule {
    key: u32,
    rules: Option<Vec<Vec<u32>>>,
    letter: Option<char>,
}

impl MyRule {
    fn _to_pest_rule(&self) -> String {
        let mut base = String::from("rule_") + &self.key.to_string() + " = { ";
        if self.rules.is_some() {
            for rules in self.rules.clone().unwrap() {
                for r in rules {
                    base = base + "rule_" + &r.to_string() + " ~ ";
                }
                base = base.trim_end_matches(" ~ ").to_string() + " | ";
            }
            base = base.trim_end_matches("| ").parse().unwrap();
            base = base + "}" + "\n";
        } else {
            base = base + r#"""# + &self.letter.unwrap().to_string() + r#"" }"# + "\n";
        }
        base
    }
}

impl FromStr for MyRule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(": ").collect_vec();
        let rules = split[1].split('|').collect_vec();
        let key = u32::from_str(split[0]).unwrap();
        if !rules[0].starts_with("\"") {
            let paired_rules = rules
                .iter()
                .map(|r| {
                    r.split_whitespace()
                        .map(|n| u32::from_str(n).unwrap())
                        .collect_vec()
                })
                .collect_vec();
            Ok(MyRule {
                key,
                rules: Some(paired_rules),
                letter: None,
            })
        } else {
            let letter = rules[0]
                .trim_start_matches("\"")
                .trim_end_matches("\"")
                .chars()
                .collect_vec()[0];
            Ok(MyRule {
                key,
                rules: None,
                letter: Some(letter),
            })
        }
    }
}
