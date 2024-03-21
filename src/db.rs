use std::collections::HashSet;
use rusqlite::{params, Connection, OptionalExtension, Result};
use crate::model::{CombinationResult, InfiniteCraft};

pub fn create_database(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS combination
             (id INTEGER PRIMARY KEY, ingr1 TEXT, ingr2 TEXT, out TEXT )",
        (),
    )?;
    Ok(())
}

pub fn create_combination(
    connection: &Connection,
    first: &str,
    second: &str,
    result: &str,
) -> Result<usize, rusqlite::Error> {
    let row = connection.execute(
        "INSERT INTO combination (ingr1,ingr2,out) VALUES (?1,?2,?3)",
        (first, second, result),
    )?;
    Ok(row)
}
pub fn initial_ingredients(conn: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT out FROM combination;")?;

    let rows = stmt.query_and_then([], |f| f.get::<_, String>(0))?;

    let mut ingredients = Vec::new();

    for ingredient in rows {
        ingredients.push(ingredient?);
    }

    if ingredients.is_empty() {
        return Ok(vec![
            "Water".to_string(),
            "Fire".to_string(),
            "Wind".to_string(),
            "Earth".to_string(),
        ]);
    }

    Ok(ingredients)
}
pub fn initial_combinations(conn: &Connection) -> Result<HashSet<Vec<String>>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT ingr1, ingr2 FROM combination;")?;

    let mut rows = stmt.query([])?;

    let mut combinations: Vec<Vec<String>> = Vec::new();
    while let Some(combination) = rows.next()? {
        combinations.push(vec![combination.get(0)?, combination.get(1)?]);
        combinations.push(vec![combination.get(1)?, combination.get(0)?]);
    }
    Ok(HashSet::from_iter(combinations))

}

pub fn check_existing_combination(conn: &Connection,first:&str,second:&str) -> Result<Option<InfiniteCraft>,rusqlite::Error>{ 
    let mut stmt = conn.prepare(
        "SELECT * FROM combination WHERE (ingr1 = ?1 AND ingr2 = ?2) OR (ingr1 = ?2 AND ingr2 = ?1)",
    )?;

    let existing_combination = stmt
        .query_row(params![first,second], |row| {
            Ok(InfiniteCraft {
                ingr1: row.get(1)?,
                ingr2: row.get(2)?,
                out: row.get(3)?,
            })
        })
        .optional()?;

    Ok(existing_combination)

}

