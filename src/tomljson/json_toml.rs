use toml;
use serialize::json;
use std::io::{BufferedReader};
use std::collections::hashmap::HashMap;

pub struct JsonConverter;
impl JsonConverter {
  pub fn new() -> JsonConverter {
    JsonConverter
  }

  fn convert_json(&self, json: &json::Json) -> toml::Value {
    match json {
      // XXX: Should we attempt to be smart and conver some to Float and other
      // to Integer?
      &json::Number(ref v) => toml::Float(v.clone()),
      &json::String(ref v) => toml::String(v.clone()),
      &json::Boolean(ref v) => toml::Boolean(v.clone()),
      // XXX: What else could this be aside from an empty string?
      &json::Null => toml::String("".to_string()),
      &json::List(ref list) => {
        // Array is Vec<toml::Value>.
        let mut toml_list = Vec::<toml::Value>::new();
        //let mut toml_list = toml::Array::new();
        for json_value in list.iter() {
          toml_list.push(self.convert_json(json_value));
        }
        toml::Array(toml_list)
      },
      &json::Object(ref obj) => {
        //let mut toml_map = toml::Table::new();
        let mut toml_map = HashMap::<String, toml::Value>::new();
        for (key, json_value) in obj.iter() {
          toml_map.insert(key.clone(), self.convert_json(json_value));
        }
        toml::Table(toml_map)
      }
    }
  }

  pub fn convert
    (&self, reader: &mut Reader) -> Result<toml::Value, json::BuilderError>  {
    // First we must convert the reader into a JSON type.
    let mut buf_reader = BufferedReader::new(reader);
    let char_iter = buf_reader.chars().map(|char_res| {
      // XXX: Is there some better way to handle this then failing the entire
      // task?
      char_res.unwrap()
    });

    let mut builder = json::Builder::new(char_iter);
    let json = try!(builder.build());

    Ok(self.convert_json(&json))
  }
}

#[cfg(test)]
mod tests {
  use super::JsonConverter;
  use std::io::fs::{File};
  #[test]
  fn convert_i64() {
    println!("{}", 120i64);
  }

  #[test]
  fn convert() {
    let converter = JsonConverter::new();
    let path = Path::new("examples/short.json");
    let mut file = File::open(&path).unwrap();

    let toml = converter.convert(&mut file);
    println!("{}", toml);
  }
}
