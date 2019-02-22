extern crate serde;
extern crate serde_json;
extern crate rust_decimal;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct PersonalData {
    name: String,
    #[serde(rename = "telephone")]
    phone: String,
    #[serde(rename = "address")]
    addr: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Permissions {
    Level0, Level1, Level2, All
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Person {
    CrewMember {
        personal: PersonalData,
        permissions: Permissions
    },
    Tourist {
        personal: PersonalData,
        budget: Decimal
    },
}

fn main() -> Result<()> {
    let mut persons = Vec::<Person>::new();

    persons.push(serde_json::from_str(r#"{
        "personal": {
            "name": "Wat", "telephone": "123456789"
        },
        "permissions": "Level0"
    }"#)?);

    persons.push(serde_json::from_str(r#"{
        "personal": {
            "name": "Żul", "telephone": "223456789"
        },
        "budget": "500.25"
    }"#)?);

    println!("{:?}", persons);

    Ok(())
}
