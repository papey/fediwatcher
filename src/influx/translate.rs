// Mod translate - used to translate data from config and get to timeseries
// Uses
use crate::config::Config;
use std::collections::HashMap;
use std::fmt;

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
pub fn new_from(val: &serde_json::Value, conf: &Config) -> Result<Measurement, String> {
    // match on kind
    match conf.kind.as_str() {
        "mastodon" | "pleroma" => Ok(new_from_mastodon_or_plemora(val, conf)),
        "mastodon_user" => Ok(new_from_mastodon_user(val, conf)),
        _ => Err(String::from("Error getting data")),
    }
}

// Function - private
// new_from_mastodon will take data from mastodon instance and convert it into a Measurement
fn new_from_mastodon_or_plemora(val: &serde_json::Value, conf: &Config) -> Measurement {
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
    let users: i64 = val["stats"]["user_count"].as_i64().unwrap();
    mesurement
        .fields
        .insert("users".to_string(), DataField::Int(users));
    // local_posts
    let local_posts: i64 = val["stats"]["domain_count"].as_i64().unwrap();
    mesurement
        .fields
        .insert("local_posts".to_string(), DataField::Int(local_posts));
    // posts
    let posts: i64 = val["stats"]["status_count"].as_i64().unwrap();
    mesurement
        .fields
        .insert("posts".to_string(), DataField::Int(posts));
    // version
    let version: String = val["version"].as_str().unwrap().to_string();
    mesurement
        .fields
        .insert("version".to_string(), DataField::Str(version));

    return mesurement;
}

// new_from_mastodon_user will take data from a mastodon user and convert it into a Measurement
fn new_from_mastodon_user(val: &serde_json::Value, conf: &Config) -> Measurement {
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
    let followers: i64 = val["followers_count"].as_i64().unwrap();
    mesurement
        .fields
        .insert("followers".to_string(), DataField::Int(followers));
    // local_posts
    let following: i64 = val["following_count"].as_i64().unwrap();
    mesurement
        .fields
        .insert("following".to_string(), DataField::Int(following));
    // posts
    let status: i64 = val["statuses_count"].as_i64().unwrap();
    mesurement
        .fields
        .insert("statuses".to_string(), DataField::Int(status));

    return mesurement;
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
        let mesurement = new_from_mastodon_or_plemora(&json, &conf);

        assert_eq!(mesurement.fields["users"], DataField::Int(31));
        assert_eq!(mesurement.fields["posts"], DataField::Int(28354));
    }

    #[test]
    fn test_new_from_pleroma() {
        // prepare
        let conf = create_test_config();

        let file = File::open("./tests/json/test.new.from.plemora.json")
            .expect("Unable to read test file");

        let json = serde_json::from_reader(file).expect("Error parsing json file");

        // launch test
        let mesurement = new_from_mastodon_or_plemora(&json, &conf);

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
        let mesurement = new_from_mastodon_user(&json, &conf);

        assert_eq!(mesurement.fields["followers"], DataField::Int(274));
        assert_eq!(mesurement.fields["statuses"], DataField::Int(15392));
    }
}
