#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsBuffer, JsObject, JsString, JsUndefined, Result};
use serde::{Deserialize, Serialize};
use toml::Value;

pub mod iso8601;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sketch {
    caption: String,
    id: String,
    image: String,
    author: String,
    #[serde(serialize_with = "iso8601::serialize")]
    date: iso8601::Stamp,
    // #[serde(serialize_with = "iso8601::serialize_toml")]
    // date: toml::value::Datetime,
    tools: Vec<Tool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tool {
    id: String,
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

/// Maps an error to napi.
#[inline]
fn invalid_argument(err: impl std::error::Error) -> napi::Error {
    napi::Error::new(napi::Status::InvalidArg, err.to_string())
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("fromToml", from_toml)?;
    exports.create_named_method("fromSketchToml", from_sketch_toml)?;
    exports.create_named_method("setBuffer", set_buffer)?;

    Ok(())
}

#[js_function(1)]
fn from_toml(ctx: CallContext) -> Result<JsObject> {
    let raw_value = ctx.get::<JsString>(0)?.into_utf8()?;
    let value = raw_value
        .as_str()?
        .parse::<Value>()
        .map_err(invalid_argument)?;

    ctx.env.to_js_value(&value)?.coerce_to_object()
}

#[js_function(1)]
fn from_sketch_toml(ctx: CallContext) -> Result<JsObject> {
    let raw_value = ctx.get::<JsString>(0)?.into_utf8()?;
    let value: Sketch = toml::from_str(raw_value.as_str()?).map_err(invalid_argument)?;

    // TODO: Would be nicer if `to_js_value` took care of casting dates.
    let timestamp = value.date.timestamp() as f64;
    let mut obj = ctx.env.to_js_value(&value)?.coerce_to_object()?;
    obj.set_named_property("date", ctx.env.create_date(timestamp)?)?;

    Ok(obj)
}

#[js_function(1)]
fn set_buffer(ctx: CallContext) -> Result<JsUndefined> {
    let buf = &mut ctx.get::<JsBuffer>(0)?.into_value()?; // &mut [u8]
    buf[0] = 1;
    buf[1] = 2;
    ctx.env.get_undefined()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn des_toml_with_date() -> std::result::Result<(), Box<dyn Error>> {
        let raw = r#"
type = "Sketch"
caption = "Bold man"
id = "12206bd06e46db3fc1c79bced449bc3844c6ea5b90c457e626e506e923a2beb67532"
image = "./image.png"
author = "arnau"
date = 2019-08-03

[[tools]]
id = "ipadpro"
name = "iPad Pro"

[[tools]]
id = "sketches"
name = "Sketches Pro"
url = "https://tayasui.com/sketches/"
            "#;

        let actual: Sketch = toml::from_str(raw)?;

        assert_eq!(actual.author, "arnau".to_string());

        Ok(())
    }
}
