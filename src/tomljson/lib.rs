#![crate_name="tomljson"]
// feature-io is for issue #27802
#![feature(io)]
extern crate toml;
extern crate rustc_serialize;

pub use toml_json::TomlConverter;
pub use json_toml::JsonConverter;

mod toml_json;
mod json_toml;
