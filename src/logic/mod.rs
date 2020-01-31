use super::data::{Gender, Relationship};
use super::server::db::Character;

fn get_characters_by_parent_name(
    subset: &[&Relationship],
    characters: &[Character],
) -> Vec<Character> {
    characters
        .iter()
        .filter(|c| subset.iter().any(|r| r.parent_name == c.name))
        .map(|c| c.clone())
        .collect()
}

fn get_characters_by_child_name(
    subset: &[&Relationship],
    characters: &[Character],
) -> Vec<Character> {
    characters
        .iter()
        .filter(|c| subset.iter().any(|r| r.child_name == c.name))
        .map(|c| c.clone())
        .collect()
}

fn get_parents(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
) -> Vec<Character> {
    let parents_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r| r.child_name == *name)
        .collect();

    get_characters_by_parent_name(&parents_name[..], characters)
}

fn get_children(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
    gender: Gender,
) -> Vec<Character> {
    let sons_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r| r.parent_name == *name && r.child_sex == gender)
        .collect();

    get_characters_by_child_name(&sons_name[..], characters)
}

fn get_sons(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
) -> Vec<Character> {
    get_children(name, characters, relationships, Gender::M)
}

fn get_daughters(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
) -> Vec<Character> {
    get_children(name, characters, relationships, Gender::F)
}

fn get_siblings(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
    gender: Gender,
) -> Vec<Character> {
    let parents = get_parents(name, characters, relationships);

    let siblings_name: Vec<&Relationship> = relationships
        .iter()
        .filter(|r| {
            parents.iter().any(|p| p.name == r.parent_name)
                && r.child_sex == gender
                && r.child_name != *name
        })
        .collect();

    get_characters_by_child_name(&siblings_name[..], characters)
}

fn get_brothers(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
) -> Vec<Character> {
    get_siblings(name, characters, relationships, Gender::M)
}

fn get_sisters(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
) -> Vec<Character> {
    get_siblings(name, characters, relationships, Gender::F)
}

fn get_nephews(
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
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
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
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
    name: &str,
    characters: &[Character],
) -> Vec<Character> {
    let surname: Vec<&str> = name.rsplitn(2, ' ').collect();
    characters
        .iter()
        .filter(|c| {
            let c_surname: Vec<&str> = c.name.rsplitn(2, ' ').collect();

            c.name != *name && c_surname[0] == surname[0] && c.alive
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
    name: &str,
    characters: &[Character],
    relationships: &[Relationship],
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
