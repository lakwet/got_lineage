use std::error::Error;
use csv::Reader;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gender {
    M,
    F,
}

#[derive(Debug, Deserialize)]
struct RelationshipRaw {
    parent_name: String,
    parent_sex: char,
    child_name: String,
    child_sex: char,
}

#[derive(Debug)]
pub struct Relationship {
    pub parent_name: String,
    pub parent_sex: Gender,
    pub child_name: String,
    pub child_sex: Gender,
}

fn convert(c: RelationshipRaw) -> Result<Relationship, Box<dyn Error>> {
    let result_parent_sex = if c.parent_sex == 'M' {
        Ok(Gender::M)
    } else if c.parent_sex == 'F' {
        Ok(Gender::F)
    } else {
        Err("Bad parent gender.")
    };
    let result_child_sex = if c.child_sex == 'M' {
        Ok(Gender::M)
    } else if c.child_sex == 'F' {
        Ok(Gender::F)
    } else {
        Err("Bad child gender.")
    };

    let parent_sex = result_parent_sex?;
    let child_sex = result_child_sex?;

    Ok(Relationship {
        parent_name: c.parent_name,
        parent_sex,
        child_name: c.child_name,
        child_sex,
    })
}

pub fn read_raw_input() -> Result<Vec<Relationship>, Box<dyn Error>> {
    let mut rdr = Reader::from_path("./src/data/raw_input.csv")?;

    let mut relationships = Vec::new();

    for result in rdr.records() {
        let mut record = result?;
        record.trim();
        let row: RelationshipRaw = record.deserialize(None)?;
        let c = convert(row)?;

        relationships.push(c);

    }

    Ok(relationships)
}
