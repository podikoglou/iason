use std::collections::HashMap;

use winnow::{
    ModalResult, Parser,
    ascii::{dec_int, space0},
    combinator::{alt, delimited, repeat, separated_pair},
    stream::AsChar,
    token::take_while,
};

#[derive(PartialEq, Eq, Hash)]
pub struct JsonString(String);

pub struct JsonNumber(pub f32);
pub struct JsonObject(pub HashMap<JsonString, JsonValue>);

pub enum JsonValue {
    String(JsonString),
    Number(JsonNumber),
    Object(JsonObject),
}

pub fn parse_string(input: &mut &str) -> ModalResult<JsonString> {
    delimited('"', take_while(0.., AsChar::is_alpha), '"')
        .map(&str::to_string)
        .map(JsonString)
        .parse_next(input)
}

pub fn parse_number(input: &mut &str) -> ModalResult<JsonNumber> {
    dec_int
        .map(|x: i32| x as f32)
        .map(JsonNumber)
        .parse_next(input)
}

pub fn parse_key_value(input: &mut &str) -> ModalResult<(JsonString, JsonValue)> {
    separated_pair(parse_string, (space0, ':', space0), parse_value).parse_next(input)
}

pub fn parse_object(input: &mut &str) -> ModalResult<JsonObject> {
    delimited(
        "{",
        repeat(0.., parse_key_value)
            .map(|obj: HashMap<JsonString, JsonValue>| obj)
            .map(JsonObject),
        "}",
    )
    .parse_next(input)
}

pub fn parse_value(input: &mut &str) -> ModalResult<JsonValue> {
    alt((
        parse_string.map(JsonValue::String),
        parse_number.map(JsonValue::Number),
        parse_object.map(JsonValue::Object),
    ))
    .parse_next(input)
}
