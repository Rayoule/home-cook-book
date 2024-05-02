use leptos::*;
use leptos::logging::log;

use crate::app::components::recipe::*;

#[allow(dead_code)]
const FAKE_API_DELAY: bool = true;


#[cfg(feature = "ssr")]
pub mod ssr {
    pub use actix_web::HttpRequest;
    pub use leptos::ServerFnError;
    pub use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:cook-book.db").await?)
    }
}


#[server]
pub async fn recipe_function(recipe: Recipe, recipe_action: RecipeAction) -> Result<(), ServerFnError> {
    use self::ssr::*;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    log!("Action {:?} for this -> {:?}", recipe_action.clone(), recipe.clone());

    let mut conn = db().await?;

    let id = recipe.id.clone();
    let name = recipe.name.clone();
    let json_recipe = JsonRecipe::from_recipe(recipe);
    let debug_recipe = json_recipe.clone();
    let serialized_recipe: String = serde_json::to_string(&json_recipe)?;

    match recipe_action {

        RecipeAction::Add => {
            match sqlx::query("INSERT INTO recipes (recipe_name, recipe) VALUES ($1, $2)")
                .bind(name.clone())
                .bind(serialized_recipe)
                .execute(&mut conn)
                .await
            {
                Ok(_row) => {
                    log!("The Recipe :\n Name: {:?} // Json: {:?} \n was ADDED Successfully!", name, debug_recipe);
                    Ok(())
                },
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        },

        RecipeAction::Save => {
            if let Some(id) = id {
                match sqlx::query( "UPDATE recipes SET recipe_name = $1, recipe = $2 WHERE id = $3;" )
                    .bind(name)
                    .bind(serialized_recipe)
                    .bind(id)
                    .execute(&mut conn)
                    .await
                {
                    Ok(_row) => {
                        log!("The Recipe :\n {:?} \n was SAVED Successfully!", debug_recipe);
                        Ok(())
                    },
                    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
                }
            } else {
                Err(ServerFnError::ServerError("No Recipe ID for recipe deletion".to_owned()))
            }
        },

        RecipeAction::Delete => {
            if let Some(id) = id {
                log!("The Recipe :\n {:?} \n is in DELETE process", debug_recipe);
                
                Ok(sqlx::query("DELETE FROM recipes WHERE id = $1")
                .bind(id)
                .execute(&mut conn)
                .await
                .map(|_| ())?)
            } else {
                Err(ServerFnError::ServerError("No Recipe ID for recipe deletion".to_owned()))
            }
        },
    }
}


#[server]
pub async fn get_recipes() -> Result<Vec<Recipe>, ServerFnError> {
    use self::ssr::*;

    log!("Fetch all recipes action.");

    let mut conn = db().await?;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    let mut recipes: Vec<Recipe> = vec![];
    let mut rows = sqlx::query_as::<_, DbRowRecipe>("SELECT * FROM recipes").fetch(&mut conn);

    use futures::TryStreamExt;
    while let Some(row) = rows.try_next().await? {
        let json_recipe: JsonRecipe = serde_json::from_str(&row.recipe)?;
        let recipe = json_recipe.to_recipe(row.id);
        recipes.push(recipe);
    }

    // Sort recipes alphabetically
    recipes.sort_by_key(|r| r.name.to_lowercase());

    Ok(recipes)
}

#[server]
pub async fn get_recipe(recipe_id: Option<u16>) -> Result<Recipe, ServerFnError> {
    use self::ssr::*;

    log!("Get recipe from id: {:?}", recipe_id);

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    if let Some(recipe_id) = recipe_id {
        let mut conn = db().await?;

        let recipe_row =
            sqlx::query_as::<_, DbRowRecipe>("SELECT * FROM recipes WHERE id = $1")
                .bind(recipe_id)
                .fetch_one(&mut conn)
                .await?;

        let json_recipe: JsonRecipe = serde_json::from_str(&recipe_row.recipe)?;
        let recipe = json_recipe.to_recipe(recipe_row.id);

        Ok(recipe)
    } else {
        log::error!("No Recipe ID");
        leptos_actix::redirect("/");
        Err(ServerFnError::ServerError("No Recipe ID".to_owned()))
    }
}


#[server]
pub async fn get_recipe_id_by_name(name: String) -> Result<Option<u16>, ServerFnError> {
    use self::ssr::*;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    if name.len() < 1 {
        log!("ERROR: Provided recipe name is EMPTY !");
        return Ok(None);
    }

    log!("Checking Recipe by name --> {:?}", name.clone());

    let mut conn = db().await?;

    match sqlx::query_as::<_, DbRowRecipe>("SELECT * FROM recipes WHERE recipe_name = $1")
            .bind(name.clone())
            .fetch_one(&mut conn)
            .await
        {
            Ok(recipe_row_id) => {
                let recipe_id = recipe_row_id.id;
                log!("Recipe named: '{:?}' -> ID: {:?} was found succesfully !", name, recipe_id.clone().to_string());
                Ok(Some(recipe_id))
            },
            Err(e) => {
                log!("ERROR: Recipe named: '{:?}' FAILED because error: {:?}", name, e.to_string());
                Err(ServerFnError::ServerError(e.to_string()))
            },
        }
}

