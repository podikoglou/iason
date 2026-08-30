use std::{collections::BTreeMap, fmt::Display};

use winnow::{
    ModalResult, Parser,
    ascii::{dec_int, space0},
    combinator::{alt, delimited, repeat, separated_pair},
    stream::AsChar,
    token::take_while,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonString(pub String);

#[derive(Debug, PartialEq)]
pub struct JsonNumber(pub f32);

#[derive(Debug, PartialEq)]
pub struct JsonObject(pub BTreeMap<JsonString, JsonValue>);

#[derive(Debug, PartialEq)]
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
            .map(|obj: BTreeMap<JsonString, JsonValue>| obj)
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

/// This is the high level function for parsing some text into a [JsonValue].
pub fn parse_json(input: &str) -> anyhow::Result<JsonValue> {
    parse_value
        .parse(input)
        .map_err(|e| anyhow::format_err!("{e}"))
}
