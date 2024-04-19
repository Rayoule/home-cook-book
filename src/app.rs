use leptos::*;
use leptos_meta::*;
use leptos_router::*;

//use serde::{Deserialize, Serialize};


mod components;
use crate::app::components::recipe::*;


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
pub async fn save_recipe(recipe: Recipe) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let mut conn = db().await?;

    println!("{:?}", recipe);

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    let recipe_id =
        recipe
            .id
            .expect("to save recipe with no ID");
    
    let json_recipe = JsonRecipe::from_recipe(recipe);
    let serialized_recipe: String = serde_json::to_string(&json_recipe)?;

    match sqlx::query( "UPDATE recipes SET recipe = $1 WHERE id = $2;" )
        .bind(serialized_recipe)
        .bind(recipe_id)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}


#[server]
pub async fn add_recipe(recipe: Recipe) -> Result<(), ServerFnError> {
    use self::ssr::*;

    println!("YOLOOOOOOOOOOOOO");

    let mut conn = db().await?;

    println!("New recipe added: /n {:?}", recipe);

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    let json_recipe = JsonRecipe::from_recipe(recipe);

    let serialized_recipe: String = serde_json::to_string(&json_recipe)?;

    match sqlx::query("INSERT INTO recipes (recipe) VALUES ($1)")
        .bind(serialized_recipe)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server]
pub async fn get_recipes() -> Result<Vec<Recipe>, ServerFnError> {
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

// The struct name and path prefix arguments are optional.
#[server]
pub async fn delete_recipe(id: u16) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let mut conn = db().await?;

    Ok(sqlx::query("DELETE FROM recipes WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .map(|_| ())?)
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

    let add_recipe = create_server_multi_action::<AddRecipe>();
    
    let recipes = create_resource(
        move || add_recipe.version().get(),
        move |_| get_recipes(),
    );

    let editable_recipe = false ;

    view! {
        <h1>"Welcome to Home Cook Book!"</h1>
        <Transition fallback=move || view! {<p>"Loading..."</p> }>
            {move || {
                let existing_todos = {
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
                                                    <EditableRecipeSheet recipe=recipe editable=editable_recipe />
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
                        {existing_todos}
                    </ul><br/>

                    <NewRecipe/>
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
