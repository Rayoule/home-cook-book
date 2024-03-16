use std::f32::consts::E;

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

    let ingredient_list = view! {<li>{recipe.ingredients.clone()}</li>};
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


#[component]
fn NewRecipeForm() -> impl IntoView {
    /*let (get_recipe_name, set_recipe_name) = create_signal(recipe.name.clone());
    let (get_recipe_ingredients, set_recipe_ingredients) = create_signal(recipe.ingredients.clone());
    let (get_recipe_instructions, set_recipe_instructions) = create_signal(recipe.instructions.clone());

    // Handler for the form submission
    let on_submit = move |e: Event| {
        e.prevent_default();
        log::info!("Form submitted: name={}, email={}, message={}", name.get(), email.get(), message.get());
        // Here you would typically process or send the form data
    };*/

    /*let (get_ingredients_entries, set_igredients_entries) = create_signal(vec!["".to_owned()]);

    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();


    view! {
        <form>
            <label for="name">Recipe Name:</label><br/>
            <input type="text" id="name" name="name" required/><br/>
            {
                let mut entries = get_ingredients_entries.get();
                for e in entries {
                    view! {
                        <label for="ingredients">Ingredients:</label><br/>
                        <input type="text" id="ingredients" name="ingredients" required/>{e}<br/>
                    }
                }
            }
            <label for="ingredients">Ingredients:</label><br/>
            <input type="text" id="ingredients" name="ingredients" required/><br/>
    
            <label for="instructions">Instructions:</label><br/>
            <textarea id="instructions" name="instructions" rows="5" required></textarea><br/>
    
            <input type="submit" value="Submit"/>
        </form>
    }*/


    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.


    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let ingredient_list = vec![(
        // an ID that wont change
        0_u16,
        // is the entry in edit mode ?
        create_signal(false),
        // the content of the entry
        create_signal("".to_owned())
    )];

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (get_ingredients, set_ingredients) = create_signal(ingredient_list);

    // CCreate a unique ID
    let mut unique_id = 1_u16;

    let add_ingredient = move |_| {
        // create a signal for the new ingredient
        let new_ingredient_signal = create_signal("".to_owned());
        let is_edit_signal = create_signal(true);
        // add this counter to the list of counters
        set_ingredients.update(move |ingredients| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            let new_id: u16 = unique_id;
            ingredients.push((new_id, is_edit_signal, new_ingredient_signal));
        });

        unique_id += 1;
    };

    view! {
        <div>
            <button on:click=add_ingredient>
                "Add Ingredient"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=get_ingredients
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|ingredient| ingredient.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, is_edit, (ingredient, set_ingredient))| {
                        view! {
                            <li>

                                <label>Ingredient:</label><br/>
                                <label>{is_edit.0.get()}</label><br/>

                                <Show
                                    when=move || { is_edit.0.get() }
                                    fallback=|| view! {}
                                >
                                    {
                                        view!{
                                            <input type="text" id="ingredients" name="ingredients" required/><br/>
                                            <button
                                                on:click=move |_| {
                                                    set_ingredients.update(|ingredients| {
                                                        // Set edit mode for this entry
                                                        ingredients.iter_mut().for_each(|i| {
                                                            if i.0 == id {
                                                                i.1.1.set(false);
                                                            }
                                                        });
                                                    });
                                                }
                                            >
                                                "Done"
                                            </button>
                                        }
                                    }
                                </Show>

                                <Show
                                    when=move || { !is_edit.0.get() }
                                    fallback=|| view! {}
                                >
                                    {
                                        view!{
                                            <button
                                                on:click=move |_| {
                                                    set_ingredients.update(|ingredients| {
                                                        // Set edit mode for this entry
                                                        ingredients.iter_mut().for_each(|i| {
                                                            if i.0 == id {
                                                                i.1.1.set(true);
                                                            }
                                                        });
                                                    });
                                                }
                                            >
                                                "Edit"
                                            </button>
                                        }
                                    }
                                </Show>
                                

                                

                                <button
                                    on:click=move |_| {
                                        set_ingredients.update(|ingredients| {
                                            ingredients.retain(|(ingredient_id, _, (signal, _))| {
                                                // NOTE: in this example, we are creating the signals
                                                // in the scope of the parent. This means the memory used to
                                                // store them will not be reclaimed until the parent component
                                                // is unmounted. Here, we're removing the signal early (i.e, before
                                                // the DynamicList is unmounted), so we manually dispose of the signal
                                                // to avoid leaking memory.
                                                //
                                                // This is only necessary with nested signals like this one.
                                                if ingredient_id == &id {
                                                    signal.dispose();
                                                }
                                                ingredient_id != &id
                                            })
                                        });
                                    }
                                >
                                    "X"
                                </button>

                            </li>
                        }
                    }
                />
            </ul>
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
        .bind(recipe.ingredients)
        .bind(recipe.instructions)
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
        <NewRecipeForm/>
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
