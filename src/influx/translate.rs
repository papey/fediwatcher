// Mod translate - used to translate data from config and get to timeseries
// Uses
use crate::config::Config;
use std::collections::HashMap;
use std::fmt;

// Errors
// Define TranslateError
#[derive(Debug, Clone)]
pub struct TranslateError {
    field: String,
    kind: String,
    url: String,
}

// Implements for TranslateError
impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error translating field {} for url {} on kind {}",
            self.field, self.url, self.kind
        )
    }
}

// DataField enum
#[derive(PartialEq)]
pub enum DataField {
    Int(i64),
    Str(String),
}

// implements for DataField
impl fmt::Debug for DataField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for DataField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Mesurement struct
#[derive(Default)]
pub struct Measurement {
    pub key: String,
    pub tags: HashMap<String, String>,
    pub fields: HashMap<String, DataField>,
}

// implements for Measurement
impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mesurement key: {}\n", self.key)?;
        write!(f, "Mesurement tags:\n")?;
        for tag in self.tags.keys() {
            write!(f, "\ttag: {}, value: {}\n", tag, self.tags[tag])?;
        }
        write!(f, "Mesurement fields :\n")?;
        for field in self.fields.keys() {
            write!(f, "\tfield: {}, value: {}\n", field, self.fields[field])?;
        }

        Ok(())
    }
}

// Funcions - public
// new_from wraps all the from kind
pub fn new_from(val: &serde_json::Value, conf: &Config) -> Result<Measurement, TranslateError> {
    // match on kind
    match conf.kind.as_str() {
        "mastodon" | "pleroma" => new_from_mastodon_or_pleroma(val, conf),
        "mastodon_user" | "pleroma_user" => new_from_mastodon_or_pleroma_user(val, conf),
        "plume" => new_from_plume(val, conf),
        "funkwhale" => new_from_funkwhale(val, conf),
        _ => panic!(
            "Unrecoverable error config of kind {} not supported",
            conf.kind
        ),
    }
}

// Function - private
// new_from_funkwhale will take data from funkwhale instance and convert it into a Measurement
fn new_from_funkwhale(
    val: &serde_json::Value,
    conf: &Config,
) -> Result<Measurement, TranslateError> {
    let mut mesurement: Measurement = Measurement::default();

    // add tags
    // kind is the key
    mesurement.key = conf.kind.clone();

    // name
    mesurement
        .tags
        .insert("name".to_string(), conf.name.clone());
    // url
    mesurement.tags.insert("url".to_string(), conf.url.clone());

    // add fields
    // user count
    match val["usage"]["users"]["total"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("users".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                field: String::from("users total"),
                kind: conf.kind.clone(),
                url: conf.url.clone(),
            })
        }
    };

    // library
    // tracks total
    match val["metadata"]["library"]["tracks"]["total"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("tracks".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                field: String::from("tracks total"),
                kind: conf.kind.clone(),
                url: conf.url.clone(),
            })
        }
    };

    // albums total
    match val["metadata"]["library"]["albums"]["total"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("albums".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                field: String::from("albums total"),
                kind: conf.kind.clone(),
                url: conf.url.clone(),
            })
        }
    };

    // artists total
    match val["metadata"]["library"]["artists"]["total"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("artists".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                field: String::from("artists total"),
                kind: conf.kind.clone(),
                url: conf.url.clone(),
            })
        }
    };

    // version
    match val["version"].as_str() {
        Some(val) => mesurement
            .fields
            .insert("version".to_string(), DataField::Str(val.to_string())),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("version"),
            })
        }
    };

    Ok(mesurement)
}

// new_from_plume will take data from plume instance and convert it into a Measurement
fn new_from_plume(val: &serde_json::Value, conf: &Config) -> Result<Measurement, TranslateError> {
    let mut mesurement: Measurement = Measurement::default();

    // add tags
    // kind is the key
    mesurement.key = conf.kind.clone();

    // name
    mesurement
        .tags
        .insert("name".to_string(), conf.name.clone());
    // url
    mesurement.tags.insert("url".to_string(), conf.url.clone());

    // add fields
    // user_count
    match val["usage"]["users"]["total"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("users".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                field: String::from("users total"),
                kind: conf.kind.clone(),
                url: conf.url.clone(),
            })
        }
    };

    // local_posts
    match val["usage"]["localPosts"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("local_posts".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("local posts"),
            })
        }
    };

    // comments
    match val["usage"]["localComments"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("local_comments".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("local comments"),
            })
        }
    };

    // version
    match val["software"]["version"].as_str() {
        Some(val) => mesurement
            .fields
            .insert("version".to_string(), DataField::Str(val.to_string())),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("version"),
            })
        }
    };

    Ok(mesurement)
}

// new_from_mastodon will take data from mastodon instance and convert it into a Measurement
fn new_from_mastodon_or_pleroma(
    val: &serde_json::Value,
    conf: &Config,
) -> Result<Measurement, TranslateError> {
    let mut mesurement: Measurement = Measurement::default();

    // add tags
    // kind is the key
    mesurement.key = conf.kind.clone();
    // name
    mesurement
        .tags
        .insert("name".to_string(), conf.name.clone());
    // url
    mesurement.tags.insert("url".to_string(), conf.url.clone());

    // add fields
    // user_count
    match val["stats"]["user_count"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("users".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("user count"),
            })
        }
    };

    // local_posts
    match val["stats"]["domain_count"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("local_posts".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("domain count"),
            })
        }
    };

    // posts
    match val["stats"]["status_count"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("posts".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("status count"),
            })
        }
    };

    // version
    match val["version"].as_str() {
        Some(val) => mesurement
            .fields
            .insert("version".to_string(), DataField::Str(String::from(val))),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("version"),
            })
        }
    };

    Ok(mesurement)
}

// new_from_mastodon_user will take data from a mastodon user and convert it into a Measurement
fn new_from_mastodon_or_pleroma_user(
    val: &serde_json::Value,
    conf: &Config,
) -> Result<Measurement, TranslateError> {
    let mut mesurement: Measurement = Measurement::default();

    // add tags
    // kind is the key
    mesurement.key = conf.kind.clone();
    // name
    mesurement
        .tags
        .insert("name".to_string(), conf.name.clone());

    // add fields
    // followers
    match val["followers_count"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("followers".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("followers"),
            })
        }
    };

    // following
    let following: i64 = match val["following_count"].as_i64() {
        Some(val) => val,
        None => 0,
    };
    mesurement
        .fields
        .insert("following".to_string(), DataField::Int(following));

    // posts
    match val["statuses_count"].as_i64() {
        Some(val) => mesurement
            .fields
            .insert("statuses".to_string(), DataField::Int(val)),
        None => {
            return Err(TranslateError {
                url: conf.url.clone(),
                kind: conf.kind.clone(),
                field: String::from("statuses count"),
            })
        }
    };

    Ok(mesurement)
}

// Tests
// Tester c'est douter
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::create_test_config;
    use std::fs::File;

    #[test]
    fn test_new_from_mastodon() {
        // prepare
        let conf = create_test_config();

        let file = File::open("./tests/json/test.new.from.mastodon.json")
            .expect("Unable to read test file");

        let json = serde_json::from_reader(file).expect("Error parsing json file");

        // launch test
        let mesurement = new_from_mastodon_or_pleroma(&json, &conf).unwrap();

        assert_eq!(mesurement.fields["users"], DataField::Int(31));
        assert_eq!(mesurement.fields["posts"], DataField::Int(28354));
    }

    #[test]
    fn test_new_from_pleroma() {
        // prepare
        let conf = create_test_config();

        let file = File::open("./tests/json/test.new.from.pleroma.json")
            .expect("Unable to read test file");

        let json = serde_json::from_reader(file).expect("Error parsing json file");

        // launch test
        let mesurement = new_from_mastodon_or_pleroma(&json, &conf).unwrap();

        assert_eq!(mesurement.fields["users"], DataField::Int(132));
        assert_eq!(mesurement.fields["posts"], DataField::Int(30687));
    }

    #[test]
    fn test_new_from_mastodon_user() {
        // prepare
        let conf = create_test_config();

        let file = File::open("./tests/json/test.new.from.mastodon_user.json")
            .expect("Unable to read test file");

        let json = serde_json::from_reader(file).expect("Error parsing json file");

        // launch test
        let mesurement = new_from_mastodon_or_pleroma_user(&json, &conf).unwrap();

        assert_eq!(mesurement.fields["followers"], DataField::Int(274));
        assert_eq!(mesurement.fields["statuses"], DataField::Int(15392));
    }

    #[test]
    fn test_new_from_pleroma_user() {
        // prepare
        let conf = create_test_config();

        let file = File::open("./tests/json/test.new.from.pleroma_user.json")
            .expect("Unable to read test file");

        let json = serde_json::from_reader(file).expect("Error parsing json file");

        // launch test
        let mesurement = new_from_mastodon_or_pleroma_user(&json, &conf).unwrap();

        assert_eq!(mesurement.fields["followers"], DataField::Int(7));
        assert_eq!(mesurement.fields["statuses"], DataField::Int(42));
    }

    #[test]
    fn test_new_from_funkwhale() {
        // prepare
        let conf = create_test_config();

        let file = File::open("./tests/json/test.new.from.funkwhale.json")
            .expect("Unable to read test file");

        let json = serde_json::from_reader(file).expect("Error parsing json file");

        // launch test
        let measurement = new_from_funkwhale(&json, &conf).unwrap();

        assert_eq!(measurement.fields["albums"], DataField::Int(20));
        assert_eq!(measurement.fields["artists"], DataField::Int(17));
    }
}
