use std::{collections::HashMap, hash};

use pulldown_cmark::{html, Options, Parser};
use serde_json::Value;

pub fn markdown<S: hash::BuildHasher>(
    value: &Value,
    _args: &HashMap<String, Value, S>,
) -> tera::Result<Value> {
    if let Some(value) = value.as_str() {
        let parser = Parser::new_ext(value, Options::all());
        let mut output = String::new();
        html::push_html(&mut output, parser);

        return Ok(serde_json::to_value(output)?);
    }

    Err(tera::Error::msg(
        "Expected the incoming value to be a string",
    ))
}
