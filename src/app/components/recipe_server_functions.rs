use leptos::*;
use crate::app::components::recipe::*;

#[cfg(feature = "ssr")]
use leptos::logging::*;
#[cfg(feature = "ssr")]
pub mod ssr {
    pub use actix_web::HttpRequest;
    pub use leptos::ServerFnError;
    pub use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:cook-book.db").await?)
    }
}



#[allow(dead_code)]
const FAKE_API_DELAY: bool = false;


#[server]
pub async fn recipe_function(recipe_action_desc: RecipeActionDescriptor) -> Result<(), ServerFnError> {
    use self::ssr::*;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    match &recipe_action_desc {
        RecipeActionDescriptor::Add(r) =>       log!("Action received: ADD -> {:?}", r.name),
        RecipeActionDescriptor::Save(r) =>      log!("Action received: SAVE -> {:?}", r.name),
        RecipeActionDescriptor::Delete(i) =>    log!("Action received: DELETE -> id: {:?}", i),
        RecipeActionDescriptor::Duplicate(i) => log!("Action received: DUPLICATE -> id: {:?}", i),
    }

    let mut conn = db().await?;

    match recipe_action_desc {

        RecipeActionDescriptor::Add(recipe) => {

            let string_name =           recipe.name;
            let string_tags =           serde_json::to_string(&JsonRecipeTags::from_recipe_tags(recipe.tags)).expect("to serialize JsonRecipeTags into String");
            let string_ingredients =    serde_json::to_string(&JsonRecipeIngredients::from_recipe_ingredients(recipe.ingredients)).expect("to serialize JsonRecipeIngredients into String");
            let string_instructions =   serde_json::to_string(&JsonRecipeInstructions::from_recipe_instructions(recipe.instructions)).expect("to serialize JsonRecipeInstructions into String");
            let string_notes =          serde_json::to_string(&JsonRecipeNotes::from_recipe_notes(recipe.notes)).expect("to serialize JsonRecipeNotes into String");

            match sqlx::query("INSERT INTO recipes (recipe_name, recipe_tags, recipe_ingredients, recipe_instructions, recipe_notes) VALUES ($1, $2, $3, $4, $5)")
                .bind(string_name.clone())
                .bind(string_tags.clone())
                .bind(string_ingredients.clone())
                .bind(string_instructions.clone())
                .bind(string_notes.clone())
                .execute(&mut conn)
                .await
            {
                Ok(_row) => {
                    log!("\nThe Recipe: {:?} was ADDED Successfully!\n\n", string_name);
                    Ok(())
                },
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        },

        RecipeActionDescriptor::Save(recipe) => {

            let id: Option<u16> =       recipe.id;
            let string_name =           recipe.name;
            let string_tags =           serde_json::to_string(&JsonRecipeTags::from_recipe_tags(recipe.tags)).expect("to serialize JsonRecipeTags into String");
            let string_ingredients =    serde_json::to_string(&JsonRecipeIngredients::from_recipe_ingredients(recipe.ingredients)).expect("to serialize JsonRecipeIngredients into String");
            let string_instructions =   serde_json::to_string(&JsonRecipeInstructions::from_recipe_instructions(recipe.instructions)).expect("to serialize JsonRecipeInstructions into String");
            let string_notes =          serde_json::to_string(&JsonRecipeNotes::from_recipe_notes(recipe.notes)).expect("to serialize JsonRecipeNotes into String");

            if let Some(id) = id {
                match sqlx::query( "UPDATE recipes SET recipe_name = $1, recipe_tags = $2, recipe_ingredients = $3, recipe_instructions = $4, recipe_notes = $5 WHERE id = $6;" )
                    .bind(string_name.clone())
                    .bind(string_tags.clone())
                    .bind(string_ingredients.clone())
                    .bind(string_instructions.clone())
                    .bind(string_notes.clone())
                    .bind(id.clone())
                    .execute(&mut conn)
                    .await
                {
                    Ok(_row) => {
                        log!("\nThe Recipe: {:?} was ADDED Successfully!\n", string_name);
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
                "INSERT INTO recipes (recipe_name, recipe_tags, recipe_ingredients, recipe_instructions, recipe_notes)
                SELECT recipe_name, recipe_tags, recipe_ingredients, recipe_instructions, recipe_notes
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

    let mut conn = db().await?;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    let mut all_recipe_light: Vec<RecipeLight> = vec![];
    //let mut rows = sqlx::query_as::<_, DbRowRecipe>("SELECT recipe_name, recipe_tags FROM recipes").fetch(&mut conn);
    let mut rows = sqlx::query_as::<_, DbRowRecipeLight>("SELECT id, recipe_name, recipe_tags, recipe_ingredients FROM recipes").fetch(&mut conn);

    use futures::TryStreamExt;
    while let Some(row) = rows.try_next().await? {
        let recipe_name: String = row.recipe_name;
        let recipe_tags: Option<Vec<RecipeTag>> =
            serde_json::from_str::<JsonRecipeTags>(&row.recipe_tags)?
            .to_recipe_tags();
        let recipe_ingredients: Option<Vec<RecipeIngredient>> =
            serde_json::from_str::<JsonRecipeIngredients>(&row.recipe_ingredients)?
            .to_recipe_ingredients();
        let recipe_light: RecipeLight =
            RecipeLight {
                id:             row.id,
                name:           recipe_name,
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
pub async fn get_recipe_by_id(recipe_id: u16) -> Result<Recipe, ServerFnError> {
    use self::ssr::*;

    log!("Getting RECIPE with ID: {:?}", recipe_id);

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    let mut conn = db().await?;

    let recipe_row =
        sqlx::query_as::<_, DbRowRecipe>("SELECT * FROM recipes WHERE id = $1")
            .bind(recipe_id)
            .fetch_one(&mut conn)
            .await?;

    let json_recipe = JsonRecipe {
        name:           recipe_row.recipe_name,
        tags:           serde_json::from_str::<JsonRecipeTags>(&recipe_row.recipe_tags)?,
        ingredients:    serde_json::from_str::<JsonRecipeIngredients>(&recipe_row.recipe_ingredients)?,
        instructions:   serde_json::from_str::<JsonRecipeInstructions>(&recipe_row.recipe_instructions)?,
        notes:          serde_json::from_str::<JsonRecipeNotes>(&recipe_row.recipe_notes)?,
    };
    let recipe = json_recipe.to_recipe(recipe_row.id);

    log!("Recipe from id: {:?} fetched Successfully.", recipe_id);

    Ok(recipe)
}






#[server]
pub async fn get_recipe_id_by_name(name: String) -> Result<Option<u16>, ServerFnError> {
    use self::ssr::*;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    if name.len() < 1 {
        error!("ERROR: Provided recipe name is EMPTY !");
        return Ok(None);
    }

    let mut conn = db().await?;

    match sqlx::query_as::<_, DbRowRecipeID>("SELECT id FROM recipes WHERE recipe_name = $1")
            .bind(name.clone())
            .fetch_one(&mut conn)
            .await
        {
            Ok(recipe_row_id) => {
                let recipe_id = recipe_row_id.id;
                log!("Recipe named: '{:?}' -> ID: {:?} was found Succesfully.", name, recipe_id.clone().to_string());
                Ok(Some(recipe_id))
            },
            Err(e) => {
                error!("ERROR: Recipe named: '{:?}' FAILED because error: {:?}", name, e.to_string());
                Err(ServerFnError::ServerError(e.to_string()))
            },
        }
}



#[server]
pub async fn get_all_recipes_as_json_string() -> Result<String, ServerFnError> {
    use self::ssr::*;

    let mut conn = db().await?;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    // Fetch all
    let mut rows = sqlx::query_as::<_, DbRowRecipe>("SELECT * FROM recipes").fetch(&mut conn);

    use futures::TryStreamExt;
    let mut all_recipes_json = JsonRecipeCollection(vec![]);
    while let Some(row) = rows.try_next().await? {
        let json_recipe = JsonRecipe {
            name:           row.recipe_name,
            tags:           serde_json::from_str::<JsonRecipeTags>(&row.recipe_tags)?,
            ingredients:    serde_json::from_str::<JsonRecipeIngredients>(&row.recipe_ingredients)?,
            instructions:   serde_json::from_str::<JsonRecipeInstructions>(&row.recipe_instructions)?,
            notes:          serde_json::from_str::<JsonRecipeNotes>(&row.recipe_notes)?,
        };
        all_recipes_json.0.push(json_recipe);
    }

    // Sort recipes alphabetically
    all_recipes_json.0.sort_by_key(|r| r.name.to_lowercase());

    // Turn into a String
    let out: String = serde_json::to_string_pretty(&all_recipes_json)?;

    Ok(out)
}


#[server]
pub async fn apply_json_save(save: String) -> Result<(), ServerFnError> {
    use self::ssr::*;

    // fake API delay
    if FAKE_API_DELAY { std::thread::sleep(std::time::Duration::from_millis(1250)); }

    let save_json = serde_json::from_str::<JsonRecipeCollection>(&save)?;

    let mut conn = db().await?;

    let _ = sqlx::query("DELETE FROM recipes;").execute(&mut conn).await?;

    for recipe in save_json.0 {
        let string_name = recipe.name;
        let string_tags =
            serde_json::to_string(&JsonRecipeTags::from_recipe_tags(recipe.tags.to_recipe_tags()))
                .expect("to serialize JsonRecipeTags into String");
        let string_ingredients =
            serde_json::to_string(&JsonRecipeIngredients::from_recipe_ingredients(recipe.ingredients.to_recipe_ingredients()))
                .expect("to serialize JsonRecipeIngredients into String");
        let string_instructions =
            serde_json::to_string(&JsonRecipeInstructions::from_recipe_instructions(recipe.instructions.to_recipe_instructions()))
                .expect("to serialize JsonRecipeInstructions into String");
        let string_notes =
            serde_json::to_string(&JsonRecipeNotes::from_recipe_notes(recipe.notes.to_recipe_notes()))
                .expect("to serialize JsonRecipeNotes into String");

        match sqlx::query(
            "INSERT INTO recipes (recipe_name, recipe_tags, recipe_ingredients, recipe_instructions, recipe_notes) VALUES ($1, $2, $3, $4, $5);"
        )
            .bind(string_name.clone())
            .bind(string_tags.clone())
            .bind(string_ingredients.clone())
            .bind(string_instructions.clone())
            .bind(string_notes.clone())
            .execute(&mut conn)
            .await
        {
            Ok(_) => log!("Recipe: {:?}  was added succesfully !", string_name),
            Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
        }
    }

    Ok(())
}
