use toml;
use rustc_serialize::json::{self, Json};
use std::io::{BufReader, Read};
use std::collections::BTreeMap;

pub struct JsonConverter;
impl JsonConverter {
    pub fn new() -> JsonConverter {
        JsonConverter
    }

    fn convert_json(&self, json: &json::Json) -> toml::Value {
        match json {
            // XXX: Should we attempt to be smart and conver some to Float and other
            // to Integer?
            &Json::F64(ref v) => toml::Value::Float(v.clone()),
            &Json::I64(ref v) => toml::Value::Integer(v.clone()),
            &Json::U64(ref v) => toml::Value::Integer(v.clone() as i64),
            &Json::String(ref v) => toml::Value::String(v.clone()),
            &Json::Boolean(ref v) => toml::Value::Boolean(v.clone()),
            // XXX: What else could this be aside from an empty string?
            &Json::Null => toml::Value::String("".to_string()),
            &Json::Array(ref list) => {
                // Array is Vec<toml::Value>.
                let mut toml_list = Vec::<toml::Value>::new();
                // let mut toml_list = toml::Array::new();
                for json_value in list.iter() {
                    toml_list.push(self.convert_json(json_value));
                }
                toml::Value::Array(toml_list)
            }
            &Json::Object(ref obj) => {
                // let mut toml_map = toml::Table::new();
                let mut toml_map = BTreeMap::<String, toml::Value>::new();
                for (key, json_value) in obj.iter() {
                    toml_map.insert(key.clone(), self.convert_json(json_value));
                }
                toml::Value::Table(toml_map)
            }
        }
    }

    pub fn convert<R>(&self, reader: &mut R) -> Result<toml::Value, json::BuilderError>
        where R: Read
    {
        // First we must convert the reader into a JSON type.
        let buf_reader = BufReader::new(reader);
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
    use std::fs::File;
    use std::path::Path;
    #[test]
    fn convert_i64() {
        println!("{}", 120i64);
    }

    #[test]
    fn convert() {
        let converter = JsonConverter::new();
        let path = Path::new("examples/short.json");
        let mut file = File::open(&path).unwrap();

        let toml = converter.convert(&mut file).expect("toml convert");
        println!("{}", toml.to_string());
    }
}
