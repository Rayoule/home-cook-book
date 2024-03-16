use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Recipe {
    name: String,
    ingredients: String,
    instructions: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipe {
    id: u16,
    recipe: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use actix_web::HttpRequest;
    pub use leptos::ServerFnError;
    pub use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:cook-book.db").await?)
    }
}


/*#[component]
fn RecipeSheet(
    recipe: Recipe,
) -> impl IntoView {

    let ingredient_list = (0..recipe.ingredients.len())
        .map(move |i| view! { <li>{recipe.ingredients[i].clone()}</li> })
        .collect_view();
    let instructions_list = (0..recipe.instructions.len())
        .map(move |i| view! { <li>{recipe.instructions[i].clone()}</li> })
        .collect_view();

    view! {
        <div>
            <h1>{recipe.name}</h1>
            <h2>Ingredients</h2>
            <ul>{ingredient_list}</ul>
            <h2>Instructions</h2>
            <ul>{instructions_list}</ul>
        </div>
    }
}*/

#[component]
fn RecipeSheet(
    recipe: Recipe,
) -> impl IntoView {

    let ingredient_list = view! { <li>{recipe.ingredients.clone()}</li> };
    let instructions_list = view! {<li>{recipe.instructions.clone()}</li>};

    view! {
        <div>
            <h1>{recipe.name}</h1>
            <h2>Ingredients</h2>
            <ul>{ingredient_list}</ul>
            <h2>Instructions</h2>
            <ul>{instructions_list}</ul>
        </div>
    }
}


#[server]
pub async fn add_recipe(recipe: Recipe) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let mut conn = db().await?;

    println!("{:?}", recipe);

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO recipes (name, ingredients, instructions) VALUES ($1, $2, $3)")
        .bind(recipe.name)
        .bind(recipe.ingredients.clone())
        .bind(recipe.instructions.clone())
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

    let mut recipes = Vec::new();
    //let mut rows = sqlx::query_as::<_, Recipe>("SELECT * FROM recipes").fetch(&mut conn);
    let mut rows = sqlx::query_as::<_, JsonRecipe>("SELECT * FROM recipes").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        let row_recipe: Recipe = serde_json::from_str(&row.recipe)?;
        recipes.push(row_recipe);
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
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let add_recipe = create_server_multi_action::<AddRecipe>();
    
    let recipes = create_resource(
        move || add_recipe.version().get(),
        move |_| get_recipes(),
    );

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <MultiActionForm
            // we can handle client-side validation in the on:submit event
            // leptos_router implements a `FromFormData` trait that lets you
            // parse deserializable types from form data and check them
            on:submit=move |ev| {
                let data = AddRecipe::from_event(&ev).expect("to parse form data");
                logging::log!("where do I run?");
                // silly example of validation: if the todo is "nope!", nope it
                if data.recipe.name == "" {
                    // ev.prevent_default() will prevent form submission
                    ev.prevent_default();
                }
            }
            action=add_recipe
        >
            <label>
                "Add a Recipe"
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
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
                                                    <li>
                                                        {recipe.name}
                                                        {recipe.ingredients}
                                                        {recipe.instructions}
                                                    </li>
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
                    </ul>
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
