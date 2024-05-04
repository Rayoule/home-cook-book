use leptos::*;
use leptos_router::*;
use leptos::logging::log;
use itertools::Itertools;

use crate::app::{
    components::{
        recipe::*, recipe_server_functions::*, tags::*
    },
    elements::{
        molecules::PageName, popups::PendingPopup,
    },
};




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
pub fn HeaderMenu() -> impl IntoView {
    view! {
        <header class="header-menu">
            <h1>{"Home Cook Book"}</h1>
            <nav>
                <A class="header-links" href="">"Recipes"</A>
                <A class="header-links" href="/new-recipe">"New Recipe"</A>
            </nav>
        </header>
    }
}



#[component]
pub fn NewRecipePage() -> impl IntoView {

    // Create the action to add the recipe
    let recipe_action = create_action(|input: &(Recipe, RecipeAction)| {
        let (current_recipe, current_action) = input;
        recipe_function(current_recipe.clone(), current_action.clone())
    });

    // Keep track of this action
    let action_pending = recipe_action.pending();
    let action_submitted = recipe_action.input();
    let action_done_id = recipe_action.value();

    // First we store the submitted recipe name in a signal as when the recipe is submitted
    let submitted_recipe_name = create_signal::<Option<String>>(None);
    create_effect(move |_| {
        if let Some((recipe_submitted, _)) = action_submitted.get() {
            log!("1/ Submitted recipe stored !");
            submitted_recipe_name.1.set(Some(recipe_submitted.name))
        }
    });

    // Then, when the action_id is Some (when the action as ended),
    // we fetch the recipe id using the name previously stored
    let ready_to_fetch_by_name = create_signal::<Option<String>>(None);
    create_effect(move |_| {
        if action_done_id.get().is_some() {
            log!("2/ ACTION ID has ARRIVED !");
            if let Some(recipe_name) = submitted_recipe_name.0.get() {
                log!("3/ Trigger Update");
                ready_to_fetch_by_name.1.set(Some(recipe_name));
            }
        }
    });

    // As soon as 'ready_to_fetch_by_name' is filled, the resource get fetched
    let submitted_recipe_id = create_resource(
        move || {
            ready_to_fetch_by_name.0.get().unwrap_or_else(|| "".to_string())
        },
        move |name| {
            log!("4/ Trigger Update with name: {:?}", name.clone());
            get_recipe_id_by_name(name)
        },
    );

    view! {
        <PendingPopup
            get_signal=action_pending
        />

        <PageName
            page_name={"New Recipe".to_string()}
        />

        <A href="/">{"Return to Home Page"}</A>

        <NewRecipe
            recipe_action=recipe_action
        />

        <Transition fallback=move || view! { <p>{"Wait for recipe id before edit..."}</p> }>
            {move || {
                if let Some(Ok(Some(recipe_id))) = submitted_recipe_id.get() {
                    let path = "/edit-recipe/".to_string() + &recipe_id.to_string();
                    view! {
                        <A href={path}>{"Edit"}</A>
                    }
                } else if ready_to_fetch_by_name.0.get().is_some() {
                    view! { <p>{"Wait for recipe id before edit..."}</p> }.into_view()
                } else {
                    ().into_view()
                }
            }}
        </Transition>
    }
}


#[derive(Params, PartialEq, Clone, Default)]
struct EditRecipeParam {
    id: Option<u16>
}

#[component]
pub fn EditRecipePage() -> impl IntoView {

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

    let view_fallback =move || view! {
        <PendingPopup/>
    };

    view! {
        <PendingPopup
            get_signal=action_pending
        />

        <PageName
            page_name={"Edit Recipe".to_string()}
        />

        <Transition fallback=view_fallback >
            {move || {
                let recipe = recipe_resource.get();

                if let Some(Ok(recipe)) = recipe {
                    view! {
                        <EditableRecipeSheet
                            recipe=                 recipe
                            is_new_recipe=          false
                            recipe_action=          recipe_action
                        />
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

    let recipe_action = create_action(|input: &(ReadSignal<Recipe>, RecipeAction)| {
        let (current_recipe, current_action) = input;
        recipe_function(current_recipe.get(), current_action.clone())
    });

    let recipe_action_pending = recipe_action.pending();
    
    let recipes_resource = create_resource(
        move || recipe_action.version().get(),
        move |_| get_recipes(),
    );

    let (all_tags, set_all_tags) = create_signal::<Vec<String>>(vec![]);
    let (selected_tags, set_selected_tags) = create_signal::<Vec<String>>(vec![]);
    let (already_selected_tags, set_already_selected_tags) = create_signal::<Vec<String>>(vec![]);


    view! {
        <Show
            when=move || recipe_action_pending.get()
        >
            <PendingPopup/>
        </Show>

        <PageName
            page_name={"Recipes".to_string()}
        />

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
                                                        <RecipeSheet
                                                            recipe=         recipe
                                                            start_expended= false
                                                            recipe_action=  recipe_action
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