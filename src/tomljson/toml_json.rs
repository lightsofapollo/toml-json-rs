use std::collections::BTreeMap;
use std::io::{Read};
use rustc_serialize::json::{ToJson, Json};
use std::str;
use toml;
use std;

use toml::Value::{
  self,
  String,
  Integer,
  Float,
  Boolean,
  Datetime,
  Array,
  Table
};

type JsonOut = BTreeMap<Value, Json>;

pub struct TomlConverter;
impl TomlConverter {
  pub fn new() -> TomlConverter {
    TomlConverter
  }

  pub fn convert_value(&self, toml: &toml::Value) -> Json {
    match *toml {
      Table(ref value) => {
        self.convert_table(value)
      },

      Array(ref array) => {
        let mut vec = Vec::new();
        for value in array.iter() {
          vec.push(self.convert_value(value));
        };
        vec.to_json()
      },

      String(ref value) =>  value.to_json(),
      Integer(ref value) => value.to_json(),
      Float(ref value) => value.to_json(),
      Boolean(ref value) => value.to_json(),
      Datetime(ref value) => value.to_json(),
    }
  }

  pub fn convert_table(&self, table: &BTreeMap<std::string::String, Value>) -> Json {
    let mut json: BTreeMap<std::string::String, Json> =  BTreeMap::new();
    for (key, value) in table.iter() {
      json.insert(key.to_string(), self.convert_value(value));
    };
    json.to_json()
  }

  pub fn convert(&self, reader: &mut Read) -> Json {
    // get the contents
    let mut content: Vec<u8> = Vec::new();
    reader.read_to_end(&mut content).unwrap();
    // convert them to a string
    let content_as_str = str::from_utf8(content.as_slice()).unwrap();
    // parser the string as toml
    let mut parser = toml::Parser::new(content_as_str);

    let toml = match parser.parse() {
      Some(value) => value,
      // XXX: implement better error reporting...
      None => {
        for err in parser.errors.iter() {
          println!("{}", err);
        };
        panic!("");
      }
    };
    self.convert_table(&toml)
  }
}
