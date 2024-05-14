use std::{error::Error, sync::Arc};

use leptos::{ev::MouseEvent, *};
use leptos_router::*;
use leptos::logging::log;
use itertools::Itertools;

use crate::app::{
    components::{
        recipe::*, recipe_server_functions::*, recipe_sheets::{
            EditableRecipeSheet, RecipeLightSheet, RecipeSheet
        }, tags::*
    },
    elements::{molecules::*, popups::*},
    set_page_name, RoundMenu, RoundMenuButton, RoundMenuInfo,

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


    let round_menu_info = create_signal(RoundMenuInfo::default());

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
                        let path = "/recipe/".to_string() + &id.to_string() + "/display";
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

        <RoundMenu
            info=round_menu_info.0
        />

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
    id: Option<u16>,
}

#[derive(Params, PartialEq, Clone, Default)]
struct RecipeModeParam {
    mode: Option<RecipePageMode>,
}

// Implement the error type for failed conversion
#[derive(Debug, Clone)]
struct ParseRecipePageModeError;
// Implement Display for your error type
impl std::fmt::Display for ParseRecipePageModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid input for RecipePageMode")
    }
}
impl serde::ser::StdError for ParseRecipePageModeError {}

#[derive(Clone, PartialEq, Debug)]
pub enum RecipePageMode {
    Display,
    Editable,
    Print,
}

impl std::str::FromStr for RecipePageMode {
    type Err = ParamsError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "display" => Ok(RecipePageMode::Display),
            "editable" => Ok(RecipePageMode::Editable),
            "print" => Ok(RecipePageMode::Print),
            _ => Err(ParamsError::Params(Arc::new(ParseRecipePageModeError))),
        }
    }
}

#[component]
pub fn RecipePage(
    //editable: RecipePageMode,
) -> impl IntoView {

    // Get params functions
    let get_recipe_id_param =move || {
        use_params::<RecipeIdParam>().get().unwrap_or_default().id.expect("To get RecipeIdParam")
    };
    let get_recipe_mode = move || {
        use_params::<RecipeModeParam>().get().unwrap_or_default().mode.expect("To get RecipeModeParam")
    };

    // Page Name setup
    set_page_name("Recipes");
    // Update Page Name
    create_effect(move |_| {
        set_page_name(
            match get_recipe_mode() {
                RecipePageMode::Display => "Display Recipe",
                RecipePageMode::Editable => "Edit Recipe",
                RecipePageMode::Print => "Print Recipe",
            }
        );
    });

    // Delete Popup infos
    let delete_popup_info = create_signal::<Option<DeletePopupInfo>>(None);
    create_effect(move |_| {
        log!("Delete Popup Info has changed to -> {:?}", delete_popup_info.0.get());
    });

    // Setup recipe action
    let recipe_action = 
        create_action(|desc: &RecipeActionDescriptor| {
            recipe_function(desc.clone())
        });
    let action_pending = recipe_action.pending();

    // RoundMenu setup for this page
    let round_menu_info = create_signal(
        RoundMenuInfo {
            recipe_action: recipe_action.into(),
            delete_info: delete_popup_info.1.into(),
            ..Default::default()
        }
    );
    // Update RoundMenu recipe_id
    create_effect(move |_| {
        round_menu_info.1.update(|rmi| rmi.recipe_id = Some(get_recipe_id_param()));
    });
    // Update RoundMenu buttons
    create_effect(move |_| {
        round_menu_info.1.update(|rmi| {
            rmi.buttons = {
                match get_recipe_mode() {
                    RecipePageMode::Display => vec![
                        RoundMenuButton::HomePage,
                        RoundMenuButton::New,
                        RoundMenuButton::Edit,
                        RoundMenuButton::Duplicate,
                        RoundMenuButton::Print,
                        RoundMenuButton::Delete,
                    ].into(),
                    RecipePageMode::Editable => vec![
                        RoundMenuButton::HomePage,
                        RoundMenuButton::Delete,
                    ].into(),
                    RecipePageMode::Print => vec![
                        RoundMenuButton::HomePage,
                        RoundMenuButton::New,
                        RoundMenuButton::Display,
                        RoundMenuButton::Edit,
                        RoundMenuButton::Duplicate,
                        RoundMenuButton::Delete,
                    ].into(),
                }
            }
        });
    });

    // Recipe resource
    let recipe_resource = create_resource(
        move || (
            recipe_action
                .version()
                .get(),
                get_recipe_id_param()
        ),
        move |(_, recipe_id)| {
            get_recipe_by_id(Some(recipe_id))
        },
    );

    let view_fallback =move || view! {
        <PendingPopup/>
    };


    view! {

        <RoundMenu
            info=round_menu_info.0
        />

        <DeleteRecipePopup
            recipe_action=  recipe_action
            info=           delete_popup_info.0
        />

        <PendingPopup
            get_signal=action_pending
        />

        <Transition fallback=move || view! { <PendingPopup/> } >
            {move || {
                let recipe = recipe_resource.get();

                if let Some(Ok(recipe)) = recipe {

                    match get_recipe_mode() {
                        RecipePageMode::Display => {
                            // Display Recipe
                            view! {
                                <RecipeSheet
                                    recipe= recipe
                                    print=  false
                                />
                            }
                        },
                        RecipePageMode::Editable => {
                            // Editable Recipe
                            view! {
                                <EditableRecipeSheet
                                    recipe_action=  recipe_action
                                    recipe=         recipe
                                    is_new_recipe=  false
                                />
                            }
                        },
                        RecipePageMode::Print => {
                            // Display Recipe
                            view! {
                                <RecipeSheet
                                    recipe= recipe
                                    print=  true
                                />
                            }
                        },
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

    let delete_popup_info = create_signal::<Option<DeletePopupInfo>>(None);
    create_effect(move |_| {
        log!("Delete Popup Info has changed to -> {:?}", delete_popup_info.0.get());
    });

    let recipe_action = create_action(|desc: &RecipeActionDescriptor| {
        recipe_function(desc.clone())
    });

    // Round Menu setup for this page
    let round_menu_info = create_signal(
        RoundMenuInfo {
            buttons: vec![ RoundMenuButton::New ].into(),
            ..Default::default()
        }
    );

    let recipe_action_pending = recipe_action.pending();
    
    let recipes_resource = create_resource(
        move || recipe_action.version().get(),
        move |_| get_all_recipes_light(),
    );

    let (all_tags, set_all_tags) = create_signal::<Vec<String>>(vec![]);
    let (selected_tags, set_selected_tags) = create_signal::<Vec<String>>(vec![]);
    let (already_selected_tags, _set_already_selected_tags) = create_signal::<Vec<String>>(vec![]);

    let (get_search_input, set_search_input) = create_signal::<Vec<String>>(vec![]);
    
    view! {

        <RecipeSearchBar
            set_search_input=set_search_input
        />

        <RoundMenu
            info=round_menu_info.0
        />

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
                                Ok(mut recipes) => {
                                    if recipes.is_empty() {
                                        view! { <p>"No recipes were found."</p> }.into_view()
                                    } else {
                                        let sel_tags = selected_tags.get();
                                        let search_input = get_search_input.get();
                                        // filter tags
                                        if sel_tags.len() > 0 {
                                            recipes.retain(|recipe| recipe.has_tags(&sel_tags));
                                        }
                                        // filter search
                                        if search_input.len() > 0 {
                                            recipes.retain(|recipe| recipe.is_in_search(&search_input));
                                        }
                                        // collect views
                                        recipes
                                            .into_iter()
                                            .map(move |recipe| {
                                                view! {
                                                    <RecipeLightSheet
                                                        recipe_light=   recipe
                                                        recipe_action=  recipe_action
                                                        delete_info=    delete_popup_info.1.clone()
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
                    <div>
                        <div>
                            {tags_component}
                        </div>

                        <DeleteRecipePopup
                            recipe_action=  recipe_action
                            info=           delete_popup_info.0
                        />

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
            <h3
                class="logo"
            >{"Home Cook Book"}</h3>
            <h4
                class="page-name"
            >{move || page_name.get()}</h4>
        </header>
    }
}

