use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use itertools::Itertools;


pub mod components;
pub mod elements;
use crate::app::components::{recipe::*, recipe_server_functions::*, tags::*};




#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/home-cook-book.css"/>

        // sets the document title
        <Title text="Home Cook Book"/>

        // content for this welcome page
        <Router>
            <main>
                <HeaderMenu/>
                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                 >
                    <Route path="/" view=AllRecipes/>
                    <Route path="/new-recipe" view=NewRecipePage/>
                    <Route path="/edit-recipe/:id" view=EditRecipePage/>
                    <Route path="/*any" view=NotFound/>
                </AnimatedRoutes>
            </main>
        </Router>
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


#[component]
fn HeaderMenu() -> impl IntoView {
    view! {
        <header class="header-menu">
            <h1>{"Home Cook Book"}</h1>
            <A href="">"Recipes"</A>
            <A href="/new-recipe">"New Recipe"</A>
        </header>
    }
}



#[component]
fn NewRecipePage() -> impl IntoView {

    let recipe_action = create_action(|input: &(Recipe, RecipeAction)| {
        let (current_recipe, current_action) = input;
        recipe_function(current_recipe.clone(), current_action.clone())
    });

    view! {
        <NewRecipe
            recipe_action=recipe_action
        />
    }
}


#[derive(Params, PartialEq, Clone, Default)]
struct EditRecipeParam {
    id: Option<u16>
}

#[component]
fn EditRecipePage() -> impl IntoView {
    
    let recipe_action = create_action(|input: &(Recipe, RecipeAction)| {
        let (current_recipe, current_action) = input;
        recipe_function(current_recipe.clone(), current_action.clone())
    });

    let action_pending = recipe_action.pending();

    let get_recipe_id_param =move || {
        use_params::<EditRecipeParam>()
            .get()
            .unwrap_or_default().id
    };

    let recipe_resource = create_resource(
        move || (recipe_action.version().get(), get_recipe_id_param()),
        move |(_, recipe_id)| get_recipe(recipe_id),
    );

    let recipe_loading = recipe_resource.loading();

    view! {
        <Transition fallback=move || view! {<p>"Loading..."</p> }>
            {move || {
                let recipe = recipe_resource.get();

                if let Some(Ok(recipe)) = recipe {
                    view! {
                        <EditableRecipeSheet
                            recipe=                 recipe
                            editable=               true
                            is_new_recipe=          false
                            recipe_action=          recipe_action
                        />
                    }
                } else {
                    view! {
                        <h1>{"Error: Could not fetch the recipe..."}</h1>
                    }.into_view()
                }
            }}
        </Transition>
    }
}


/// Renders the home page of your application.
#[component]
fn AllRecipes() -> impl IntoView {

    use components::recipe::Recipe;

    let recipe_action = create_action(|input: &(Recipe, RecipeAction)| {
        let (current_recipe, current_action) = input;
        recipe_function(current_recipe.clone(), current_action.clone())
    });

    let action_pending = recipe_action.pending();
    
    let recipes_resource = create_resource(
        move || recipe_action.version().get(),
        move |_| get_recipes(),
    );

    let (all_tags, set_all_tags) = create_signal::<Vec<String>>(vec![]);
    let (selected_tags, set_selected_tags) = create_signal::<Vec<String>>(vec![]);
    let (already_selected_tags, set_already_selected_tags) = create_signal::<Vec<String>>(vec![]);

    let editable_recipe = false;

    view! {
        //<h1>"Welcome to Home Cook Book!"</h1>
        <Transition fallback=move || view! {<p>"Loading..."</p> }>
            { move || {
                let tags_component = {
                    move || {
                        let recipes = recipes_resource.get();
                        let mut tag_list =
                            if let Some(Ok(recipes)) = recipes {
                                recipes
                                    .iter()
                                    .map(|recipe| recipe.tags.clone().unwrap_or_else(|| vec![]) )
                                    .flatten()
                                    .unique()
                                    .collect::<Vec<String>>()
                            } else { vec![] };
                        tag_list.sort_by_key(|t| t.to_lowercase().clone());
                        set_all_tags.set(tag_list);

                        view! {
                            <TagList
                                tags=all_tags
                                // Tags that are selected
                                selected_tags_signal=set_selected_tags
                                // Tags that are already checked (needed because the component might redraw if tags are added or removed)
                                // This needs to be updated ONLY if tags are added or removed (through addind/removing recipes)
                                already_selected_tags=already_selected_tags 
                            />
                        }
                    }
                };

                let existing_recipes = {
                    move || {
                        recipes_resource.get()
                            .map(move |recipes| match recipes {
                                Err(e) => {
                                    view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                }
                                Ok(recipes) => {
                                    if recipes.is_empty() {
                                        view! { <p>"No recipes were found."</p> }.into_view()
                                    } else {
                                        let sel_tags = selected_tags.get();
                                        recipes
                                            .into_iter()
                                            .filter_map(move |recipe| {
                                                if recipe.has_tags(&sel_tags) {
                                                    Some(view! {
                                                        <RecipeSheet
                                                            recipe=         recipe
                                                            start_expended= false
                                                        />
                                                    })
                                                } else {
                                                    None
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
                    <div>
                        <div>
                            {tags_component}
                        </div>

                        <div
                            class="action-pending-popup"
                            class:action-pending-hidden = move || !action_pending.get()
                        >
                            <p>{"Please Wait..."}</p>
                        </div>

                        <div class="recipe-list-container">
                            {existing_recipes}
                        </div>
                    </div>
                }
            }
        }
        </Transition>
    }
}


