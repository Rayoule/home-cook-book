use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use itertools::Itertools;

//use serde::{Deserialize, Serialize};


mod components;
use crate::app::components::{recipe::*, tags::*};


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

    println!("{:?}", &recipe);

    let mut conn = db().await?;

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));
    let id = recipe.id.clone();
    let json_recipe = JsonRecipe::from_recipe(recipe);
    let serialized_recipe: String = serde_json::to_string(&json_recipe)?;

    match recipe_action {

        RecipeAction::Add => {
            match sqlx::query("INSERT INTO recipes (recipe) VALUES ($1)")
                .bind(serialized_recipe)
                .execute(&mut conn)
                .await
            {
                Ok(_row) => Ok(()),
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        },

        RecipeAction::Save => {
            if let Some(id) = id {
                match sqlx::query( "UPDATE recipes SET recipe = $1 WHERE id = $2;" )
                    .bind(serialized_recipe)
                    .bind(id)
                    .execute(&mut conn)
                    .await
                {
                    Ok(_row) => Ok(()),
                    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
                }
            } else {
                Err(ServerFnError::ServerError("No Recipe ID for recipe deletion".to_owned()))
            }
        },

        RecipeAction::Delete => {
            if let Some(id) = id {
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
pub async fn get_recipes(mon_bool: bool) -> Result<Vec<Recipe>, ServerFnError> {
    use self::ssr::*;

    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut recipes: Vec<Recipe> = vec![];
    let mut rows = sqlx::query_as::<_, DbRowRecipe>("SELECT * FROM recipes").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        let json_recipe: JsonRecipe = serde_json::from_str(&row.recipe)?;
        let recipe = json_recipe.to_recipe(row.id);
        recipes.push(recipe);
    }

    Ok(recipes)
}


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/home-cook-book.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}


/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    use components::recipe::Recipe;

    let recipe_action = create_action(|input: &(Recipe, RecipeAction)| {
        let (current_recipe, current_action) = input;
        recipe_function(current_recipe.clone(), current_action.clone())
    });

    let custom_sig = create_signal(true);
    
    let recipes = create_resource(
        move || recipe_action.version().get(),
        move |_| get_recipes(custom_sig.0.get()),
    );

    let (all_tags, set_all_tags) = create_signal::<Vec<String>>(vec![]);
    let (selected_tags, set_selected_tags) = create_signal::<Vec<String>>(vec![]);
    let (already_selected_tags, set_already_selected_tags) = create_signal::<Vec<String>>(vec![]);

    create_effect(move |_| {
        let recipes = recipes.get();
        let tag_list =
            if let Some(Ok(recipes)) = recipes {
                recipes
                    .iter()
                    .map(|recipe| recipe.tags.clone().unwrap_or_else(|| vec![]))
                    .flatten()
                    .unique()
                    .collect::<Vec<String>>()

            } else {
                vec![]
            };
        set_all_tags.set(tag_list);
    });

    let editable_recipe = false;

    view! {
        <h1>"Welcome to Home Cook Book!"</h1>
        <Transition fallback=move || view! {<p>"Loading..."</p> }>
            <TagList
                tags=all_tags
                // Tags that are selected
                selected_tags_signal=set_selected_tags
                // Tags that are already checked (needed because the component might redraw if tags are added or removed)
                // This needs to be updated ONLY if tags are added or removed (through addind/removing recipes)
                already_selected_tags=already_selected_tags
            />
        </Transition>
        <Transition fallback=move || view! {<p>"Loading..."</p> }>
            {move || {
                let existing_recipes = {
                    move || {
                        recipes.get()
                            .map(move |recipes| match recipes {
                                Err(e) => {
                                    view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                }
                                Ok(recipes) => {
                                    if recipes.is_empty() {
                                        view! { <p>"No recipes were found."</p> }.into_view()
                                    } else {
                                        recipes
                                            .into_iter()
                                            .map(move |recipe| {
                                                view! {
                                                    <EditableRecipeSheet
                                                        recipe=recipe
                                                        editable=editable_recipe
                                                        recipe_action=recipe_action
                                                    />
                                                }
                                            })
                                            .collect_view()
                                    }
                                }
                            })
                            .unwrap_or_default()
                    }
                };

                view! {
                    <ul>
                        {existing_recipes}
                    </ul><br/>

                    <NewRecipe
                        recipe_action=recipe_action
                    />
                }
            }
        }
        </Transition>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
