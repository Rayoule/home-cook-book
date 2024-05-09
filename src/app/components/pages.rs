use leptos::*;
use leptos_router::*;
use leptos::logging::log;
use itertools::Itertools;

use crate::app::{
    components::{
        recipe::*, recipe_server_functions::*, recipe_sheets::{
            EditableRecipeSheet, RecipeLightSheet, RecipeSheet
        }, tags::*
    },
    elements::popups::PendingPopup, set_page_name,
};



#[component]
pub fn NewRecipePage() -> impl IntoView {

    set_page_name("New Recipe");

    // Setup action
    let recipe_action = 
        create_action(|desc: &RecipeActionDescriptor| {
            recipe_function(desc.clone())
        });
    let action_pending = recipe_action.pending();
    let action_submitted = recipe_action.input();
    let action_done_id = recipe_action.value();


    // store the submitted recipe name
    let submitted_name = create_rw_signal("".to_owned());
    create_effect(move |_| {
        if let Some(action_desc) = action_submitted.get() {
            match action_desc {
                RecipeActionDescriptor::Add(recipe) => submitted_name.set(recipe.name),
                _ => (),
            }
        }
    });

    // Action that takes the recipe name to fetch the recipe ID
    // and then redirect to the edit page for this recipe
    let fetch_id_and_redirect = create_action(|name: &String| {
        let name = name.clone();
        async move {
            match get_recipe_id_by_name(name.clone()).await {
                Ok(id) => {
                    if let Some(id) = id {
                        let path = "/edit-recipe/".to_string() + &id.to_string();
                        let navigate = leptos_router::use_navigate();
                        navigate(&path, Default::default());
                    } else {
                        log!("Error fetching recipe by name, no ID fetched.")
                    }
                },
                Err(_) => log!("Error fetching recipe by name with name: {:?}", name),
            }
        }
    });

    let fetch_and_redirect_pending = fetch_id_and_redirect.pending();

    // Once the recipe submission is done (when 'action_done_id' is Some)
    // grab the name and launch the 'fetch_id_and_redirect' action
    create_effect(move |_| {
        if let Some(r) = action_done_id.get() {
            match r {
                Ok(_) => {
                    let name = submitted_name.get();
                    if name.len() < 1 {
                        log!("ERROR: Won't fetch the id with an empty recipe name.");
                    } else {
                        fetch_id_and_redirect.dispatch(name);
                    }
                },
                Err(e) => log!("ERROR: Error in getting recipe submission ID: {:?}", e.to_string()),
            }
        } else { log!("No action ID yet") }
    });

    view! {

        <div class="sub-header">
        </div>

        <PendingPopup
            get_signal=action_pending
        />

        <A href="/">{"Return to Home Page"}</A>

        <EditableRecipeSheet
            recipe_action=  recipe_action
            is_new_recipe=  true
        />

        <Show
            when=fetch_and_redirect_pending
        >
            <p>{"Wait for recipe id before edit..."}</p>
        </Show>
    }
}





#[derive(Params, PartialEq, Clone, Default)]
struct RecipeIdParam {
    id: Option<u16>
}

#[component]
pub fn RecipePage(
    editable: bool,
) -> impl IntoView {

    let page_name = if editable {
        "Edit Recipe"
    } else {
        "Recipe"
    };
    set_page_name(page_name);

    // fetch param
    let get_recipe_id_param = move || {
        use_params::<RecipeIdParam>()
            .get()
            .unwrap_or_default().id
    };

    // Setup action
    let recipe_action = 
        create_action(|desc: &RecipeActionDescriptor| {
            recipe_function(desc.clone())
        });
    let action_pending = recipe_action.pending();


    let recipe_resource = create_resource(
        move || (
            recipe_action
                .version()
                .get(),
            get_recipe_id_param()
        ),
        move |(_, recipe_id)| get_recipe_by_id(recipe_id),
    );

    let view_fallback =move || view! {
        <PendingPopup/>
    };

    view! {

        <div class="sub-header">
        </div>

        <PendingPopup
            get_signal=action_pending
        />

        <Transition fallback=move || view! { <PendingPopup/> } >
            {move || {
                let recipe = recipe_resource.get();

                if let Some(Ok(recipe)) = recipe {

                    if editable {
                        // Editable Recipe
                        view! {
                            <EditableRecipeSheet
                                recipe_action=  recipe_action
                                recipe=         recipe
                                is_new_recipe=  false
                            />
                        }
                    } else {
                        // Display Recipe
                        view! {
                            <RecipeSheet
                                recipe=recipe
                            />
                        }
                    }
                } else {
                    view_fallback().into_view()
                }
            }}
        </Transition>
    }
}

/// Renders the home page of your application.
#[component]
pub fn AllRecipes() -> impl IntoView {

    set_page_name("Recipes");

    let recipe_action = create_action(|desc: &RecipeActionDescriptor| {
        recipe_function(desc.clone())
    });

    let recipe_action_pending = recipe_action.pending();
    
    let recipes_resource = create_resource(
        move || recipe_action.version().get(),
        move |_| get_all_recipes_light(),
    );

    let (all_tags, set_all_tags) = create_signal::<Vec<String>>(vec![]);
    let (selected_tags, set_selected_tags) = create_signal::<Vec<String>>(vec![]);
    let (already_selected_tags, set_already_selected_tags) = create_signal::<Vec<String>>(vec![]);


    view! {

        <div class="sub-header">
        </div>

        <Show
            when=move || recipe_action_pending.get()
        >
            <PendingPopup/>
        </Show>

        // TagList
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
                                    .map(|t| t.name)
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

                // list of RecipeLightSheet
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
                                                    Some( view! {
                                                        <RecipeLightSheet
                                                            recipe_light=   recipe
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

                        <div class="recipe-list-container">
                            {existing_recipes}
                        </div>
                    </div>
                }
            }}
        </Transition>
    }
}


/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
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
pub fn HeaderMenu(
    page_name: ReadSignal<String>
) -> impl IntoView {
    view! {
        <header class="header-menu">
            <h2>{"Home Cook Book"}</h2>
            <h1>{page_name}</h1>
            <nav>
                <A class="header-links" href="">"Recipes"</A>
                <A class="header-links" href="/new-recipe">"New Recipe"</A>
            </nav>
        </header>
    }
}
