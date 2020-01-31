use super::data::{Relationship, Gender};
use super::server::db::Character;

fn get_parents(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let parents_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r| r.child_name == *name)
        .collect();

    characters
        .iter()
        .filter(|c| parents_name.iter().find(|r| r.parent_name == c.name).is_some())
        .map(|c| c.clone())
        .collect()
}

fn get_sons(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let sons_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r| r.parent_name == *name && r.child_sex == Gender::M)
        .collect();

    characters
        .iter()
        .filter(|c| sons_name.iter().find(|r| r.child_name == c.name).is_some())
        .map(|c| c.clone())
        .collect()
}

fn get_brothers(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let parents = get_parents(name, characters, relationships);

    let brothers_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r|
            parents.iter().find(|p| p.name == r.parent_name).is_some()
            && r.child_sex == Gender::M
            && r.child_name != *name)
        .collect();

    characters
        .iter()
        .filter(|c| brothers_name.iter().find(|r| r.child_name == c.name).is_some())
        .map(|c| c.clone())
        .collect()
}

fn get_sisters(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let parents = get_parents(name, characters, relationships);

    let sisters_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r|
            parents.iter().find(|p| p.name == r.parent_name).is_some()
            && r.child_sex == Gender::F
            && r.child_name != *name)
        .collect();

    characters
        .iter()
        .filter(|c| sisters_name.iter().find(|r| r.child_name == c.name).is_some())
        .map(|c| c.clone())
        .collect()
}

fn get_daughters(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let daugthers_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r| r.parent_name == *name && r.child_sex == Gender::F)
        .collect();

    characters
        .iter()
        .filter(|c| daugthers_name.iter().find(|r| r.child_name == c.name).is_some())
        .map(|c| c.clone())
        .collect()
}

fn get_nephews(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let mut sisters = get_sisters(name, characters, relationships);
    let brothers = get_brothers(name, characters, relationships);

    sisters.extend(brothers);

    let mut nephews = Vec::new();

    for sibling in sisters.iter() {
        let sons = get_sons(&sibling.name, characters, relationships);
        nephews.extend(sons);
    }

    nephews
}

fn get_nieces(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Vec<Character> {
    let mut sisters = get_sisters(name, characters, relationships);
    let brothers = get_brothers(name, characters, relationships);

    sisters.extend(brothers);

    let mut nieces = Vec::new();

    for sibling in sisters.iter() {
        let daughters = get_daughters(&sibling.name, characters, relationships);
        nieces.extend(daughters);
    }

    nieces
}

fn get_survivors_within_the_house(
    name: &String,
    characters: &Vec<Character>,
) -> Vec<Character> {
    let surname: Vec<&str> = name.rsplitn(2, ' ').collect();
    characters
        .iter()
        .filter(|c| {
            let c_surname: Vec<&str> = c.name.rsplitn(2, ' ').collect();

            c.name != *name
            && c_surname[0] == surname[0]
            && c.alive
        })
        .map(|c| c.clone())
        .collect()
}

// Succession rules:
// 1 - Sons
// 2 - Brothers
// 3 - Nephews
// 4 - Daughters
// 5 - Sisters
// 6 - Nieces
// 7 - Others

pub fn next_heir(
    name: &String,
    characters: &Vec<Character>,
    relationships: &Vec<Relationship>,
) -> Option<Character> {
    let mut sons = get_sons(name, characters, relationships);
    sons.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut daughters = get_daughters(name, characters, relationships);
    daughters.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut brothers = get_brothers(name, characters, relationships);
    brothers.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut sisters = get_sisters(name, characters, relationships);
    sisters.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut nephews = get_nephews(name, characters, relationships);
    nephews.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut nieces = get_nieces(name, characters, relationships);
    nieces.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    let mut survivors = get_survivors_within_the_house(name, characters);
    survivors.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    println!("sons: {:?}", sons);
    println!("daughters: {:?}", daughters);
    println!("brothers: {:?}", brothers);
    println!("sisters: {:?}", sisters);
    println!("nephews: {:?}", nephews);
    println!("nieces: {:?}", nieces);
    println!("survivors: {:?}", survivors);

    sons.extend(brothers);
    sons.extend(nephews);
    sons.extend(daughters);
    sons.extend(sisters);
    sons.extend(nieces);
    sons.extend(survivors);

    match sons.iter().find(|c| c.alive) {
        Some(c) => Some(c.clone()),
        None => None,
    }
}
