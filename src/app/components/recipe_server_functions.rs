use leptos::*;
use leptos::logging::log;

use crate::app::components::recipe::*;

#[allow(dead_code)]
const FAKE_API_DELAY: bool = false;


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
pub async fn recipe_function(recipe_action_desc: RecipeActionDescriptor) -> Result<(), ServerFnError> {
    use self::ssr::*;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    match &recipe_action_desc {
        RecipeActionDescriptor::Add(r) =>       log!("Action received: ADD -> {:?}", r),
        RecipeActionDescriptor::Save(r) =>      log!("Action received: SAVE -> {:?}", r),
        RecipeActionDescriptor::Delete(i) =>    log!("Action received: DELETE -> {:?}", i),
        RecipeActionDescriptor::Duplicate(i) => log!("Action received: DUPLICATE -> {:?}", i),
    }

    let mut conn = db().await?;

    match recipe_action_desc {

        RecipeActionDescriptor::Add(recipe) => {

            let recipe_to_serialize =           recipe.clone();
            let name: String =                  recipe.name;
            let debug_name: String =            name.clone();
            let json_tags: String =             serde_json::to_string(&recipe.tags).expect("to serialize tags into String");
            let json_ingredients: String =      serde_json::to_string(&recipe.ingredients).expect("to serialize tags into String");
            let json_recipe =                   JsonRecipe::from_recipe(recipe_to_serialize);
            let debug_recipe =                  json_recipe.clone();
            let ser_recipe: String =            serde_json::to_string(&json_recipe).expect("to serialize JsonRecipe into String");

            match sqlx::query("INSERT INTO recipes (recipe_name, recipe_tags, recipe_ingredients, recipe) VALUES ($1, $2, $3, $4)")
                .bind(name)
                .bind(json_tags)
                .bind(json_ingredients)
                .bind(ser_recipe)
                .execute(&mut conn)
                .await
            {
                Ok(_row) => {
                    log!("The Recipe :\n Name: {:?} // Json: {:?} \n was ADDED Successfully!", debug_name, debug_recipe);
                    Ok(())
                },
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        },

        RecipeActionDescriptor::Save(recipe) => {

            let recipe_to_serialize =           recipe.clone();
            let id: Option<u16> =               recipe.id;
            let name: String =                  recipe.name;
            let json_tags: String =             serde_json::to_string(&recipe.tags).expect("to serialize tags into String");
            let json_ingredients: String =      serde_json::to_string(&recipe.ingredients).expect("to serialize tags into String");
            let json_recipe =                   JsonRecipe::from_recipe(recipe_to_serialize);
            let debug_recipe =                  json_recipe.clone();
            let ser_recipe: String =            serde_json::to_string(&json_recipe).expect("to serialize JsonRecipe into String");

            if let Some(id) = id {
                match sqlx::query( "UPDATE recipes SET recipe_name = $1, recipe_tags = $2, recipe_ingredients = $3, recipe = $4 WHERE id = $5;" )
                    .bind(name)
                    .bind(json_tags)
                    .bind(json_ingredients)
                    .bind(ser_recipe)
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
                Err(ServerFnError::ServerError("No Recipe ID for recipe save".to_owned()))
            }
        },

        RecipeActionDescriptor::Delete(id) => { 

            match sqlx::query("DELETE FROM recipes WHERE id = $1")
                .bind(id)
                .execute(&mut conn)
                .await
            {
                Ok(_)   => {
                    log!("The Recipe with ID :\n {:?} \n was DELETED successfully", id);
                    Ok(())
                },
                Err(e)  => Err(ServerFnError::ServerError(e.to_string()))
            }
            
            
        },

        RecipeActionDescriptor::Duplicate(id) => {
            match sqlx::query(
                "INSERT INTO recipes (recipe_name, recipe_tags, recipe_ingredients, recipe)
                SELECT recipe_name, recipe_tags, recipe_ingredients, recipe
                FROM recipes
                WHERE id = $1;"
            )
                .bind(id)
                .execute(&mut conn)
                .await
            {
                Ok(_)   => {
                    log!("The Recipe with ID :\n {:?} \n was DUPLICATED successfully", id);
                    Ok(())
                },
                Err(e)  => Err(ServerFnError::ServerError(e.to_string()))
            }
            
        },
    }
}






#[server]
pub async fn get_all_recipes_light() -> Result<Vec<RecipeLight>, ServerFnError> {
    use self::ssr::*;

    log!("Fetch all recipes action.");

    let mut conn = db().await?;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    let mut all_recipe_light: Vec<RecipeLight> = vec![];
    //let mut rows = sqlx::query_as::<_, DbRowRecipe>("SELECT recipe_name, recipe_tags FROM recipes").fetch(&mut conn);
    let mut rows = sqlx::query_as::<_, DbRowRecipeLight>("SELECT id, recipe_name, recipe_tags, recipe_ingredients FROM recipes").fetch(&mut conn);

    use futures::TryStreamExt;
    while let Some(row) = rows.try_next().await? {
        let recipe_tags: Option<Vec<RecipeTag>> = serde_json::from_str(&row.recipe_tags)?;
        let recipe_ingredients: Option<Vec<RecipeIngredient>> = serde_json::from_str(&row.recipe_ingredients)?;
        let recipe_light: RecipeLight =
            RecipeLight {
                id:             row.id,
                name:           row.recipe_name,
                tags:           recipe_tags,
                ingredients:    recipe_ingredients,
            };
        
        all_recipe_light.push(recipe_light);
    }

    // Sort recipes alphabetically
    all_recipe_light.sort_by_key(|r| r.name.to_lowercase());

    Ok(all_recipe_light)
}






#[server]
pub async fn get_recipe_by_id(recipe_id: Option<u16>) -> Result<Recipe, ServerFnError> {
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

    match sqlx::query_as::<_, DbRowRecipeID>("SELECT id FROM recipes WHERE recipe_name = $1")
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

