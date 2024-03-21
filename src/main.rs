mod db;
mod http;
mod model;

use crate::{db::create_combination, http::request_combination, model::InfiniteCraft};

use crate::db::{create_database, initial_combinations, initial_ingredients};
use crate::http::build_http_client;

use itertools::Itertools;
use rusqlite::{params, Connection, OptionalExtension, Result};
use std::collections::HashSet;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = build_http_client().await?;

    let conn = Connection::open("./infinite-craft.db").unwrap();

    create_database(&conn)?;

    let mut current = initial_ingredients(&conn)?;

    let mut duplicated_combinations: HashSet<Vec<String>> = initial_combinations(&conn)?;

    loop {
        let combinations: HashSet<Vec<String>> =
            HashSet::from_iter(current.clone().into_iter().combinations_with_replacement(2));

        for combination in combinations.symmetric_difference(&duplicated_combinations) {
            if duplicated_combinations.contains(combination) {
                println!("🔄 {} + {} Skiped ...", combination[0], combination[1]);
                continue;
            };

            let mut stmt = conn.prepare(
                "SELECT * FROM combination WHERE (ingr1 = ?1 AND ingr2 = ?2) OR (ingr1 = ?2 AND ingr2 = ?1)",
            )?;

            let existing_combination = stmt
                .query_row(params![&combination[0], &combination[1]], |row| {
                    Ok(InfiniteCraft {
                        ingr1: row.get(1)?,
                        ingr2: row.get(2)?,
                        out: row.get(3)?,
                    })
                })
                .optional()?;

            if let Some(combination) = existing_combination {
                println!(
                    "☑  {} + {} -> skip...",
                    &combination.ingr1, &combination.ingr2
                );
                continue;
            };

            let combination_result = request_combination(combination, &client).await?;

            if !current.contains(&combination_result.result) {
                current.push(combination_result.result.clone());
            };
            let _ = create_combination(
                &conn,
                &combination[0],
                &combination[1],
                &combination_result.result,
            )?;

            println!(
                "{} + {} -> {} {} {}",
                combination[0],
                combination[1],
                combination_result.emoji,
                &combination_result.result,
                if combination_result.is_new {
                    "(FIRST EVER)"
                } else {
                    ""
                }
            );
        }
        duplicated_combinations.extend(combinations);
    }
}
