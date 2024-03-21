mod db;
mod http;
mod model;


use crate::db::{create_database, initial_combinations, initial_ingredients,create_combination,check_existing_combination};
use crate::http::{ request_combination,build_http_client };

use itertools::Itertools;
use rusqlite::{ Connection, Result};
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


            if let Ok(Some(combination)) = check_existing_combination(&conn, &combination[0], &combination[1]) {
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
