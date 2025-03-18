use nom::{
    bytes::complete::take_while1,
    character::complete::{char, space0},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
pub struct StyleParser {
    input: String,
}

impl StyleParser {
    pub fn new(input: &str) -> Self {
        Self { input: input.to_string() }
    }

    pub fn parse(&self) -> Vec<StyleRule> {
        let mut rules = Vec::new();
        let mut input = self.input.as_str();

        while !input.is_empty() {
            if let Ok((remaining, rule)) = self.parse_rule(input) {
                rules.push(rule);
                input = remaining;
            }
        }

        rules
    }

    fn parse_rule(&self, input: &str) -> IResult<&str, StyleRule> {
        let (input, (selector, _, properties)) = tuple((
            take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '#'),
            char('{'),
            self.parse_properties
        ))(input)?;

        Ok((input, StyleRule { selector: selector.to_string(), properties }))
    }
}
