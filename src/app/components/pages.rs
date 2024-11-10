use components::recipe_sheets::PrintRecipeSheet;
use elements::{
    icons_svg::{BackButtonSVG, LogoSVG},
    recipe_elements::SettingsMenu,
};
use ev::MouseEvent;
use leptos::*;
use leptos_router::*;
use leptos::logging::*;
use std::sync::Arc;

use crate::app::{
    *,
    components::{
        recipe_server_functions::*, recipe_sheets::{
            EditableRecipeSheet, RecipeCard, RecipeSheet
        }, tags::*, download_upload::{DownloadAll, UploadAll},
    },
    elements::molecules::*,
};


#[component]
pub fn LoginMenu() -> impl IntoView {

    set_page_name("Login");

    // get settings menu context
    let is_settings_menu_open =
        use_context::<IsSettingsMenuOpen>()
            .expect("Expected to find IsSettingsMenuOpen in context.")
            .0;

    let try_login_action = use_context::<TryLoginAction>().expect("Expected to find TryLoginAction in context.").0;
    let try_login_action_value = try_login_action.value();
    create_effect(move |_| {
        // If login is succesful, then close settings menu
        if try_login_action_value.get().is_some_and(|result| result) {
            is_settings_menu_open.set(false);
        }
    });

    // setup submission signals (username, password)
    let submission = create_signal((String::new(), String::new()));

    // name input noderef
    let name_input: NodeRef<html::Input> = create_node_ref();
    // password input noderef
    let password_input: NodeRef<html::Input> = create_node_ref();

    // Handler for form submission
    let submit_event = move |event: ev::SubmitEvent| {
        event.prevent_default(); // Prevent the default form submission

        let login_account = LoginAccount {
            username: name_input().expect("name <input> should be mounted").value(),
            password: password_input().expect("password <input> should be mounted").value()
        };

        try_login_action.dispatch(login_account);
    };
    
    view! {
        <div>
            <h3 class="login-title" >{"Login"}</h3>
            <form class="login-form" on:submit=submit_event>
                <input
                    class="login-input"
                    type="text"
                    placeholder="Username"
                    value=move || submission.0.get().0
                    node_ref=name_input
                />
                <br/>
                <input
                    class="login-input"
                    type="password"
                    placeholder="Password"
                    value=move || submission.0.get().1
                    node_ref=password_input
                />
                <br/>
                <button class="login-button" type="submit"> "ok" </button>
            </form>
        </div>
    }
}



#[component]
pub fn NewRecipePage() -> impl IntoView {

    set_page_name("New Recipe");

    // Ensure the user is logged in
    check_login_wall();

    // Setup action
    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;
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
                        let path = "/recipe/".to_string() + &id.to_string() + "/display";
                        let navigate = leptos_router::use_navigate();
                        navigate(&path, Default::default());
                    } else {
                        error!("Error fetching recipe by name, no ID fetched.")
                    }
                },
                Err(_) => error!("Error fetching recipe by name with name: {:?}", name),
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
                        error!("ERROR: Won't fetch the id with an empty recipe name.");
                    } else {
                        fetch_id_and_redirect.dispatch(name);
                    }
                },
                Err(e) => error!("ERROR: Error in getting recipe submission ID: {:?}", e.to_string()),
            }
        }
    });

    view! {

        <div class="main-content">
            <EditableRecipeSheet
                is_new_recipe=  true
            />
        </div>

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
pub struct RecipeModeParam {
    pub mode: Option<RecipePageMode>,
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
pub fn RecipePage() -> impl IntoView {

    // Get params functions
    let get_recipe_id_param = move |tracked: bool| {
        let params = if tracked {
            use_params::<RecipeIdParam>().get()
        } else {
            use_params::<RecipeIdParam>().get_untracked()
        };
        params.unwrap_or_default().id.expect("To get RecipeIdParam")
    };
    let get_recipe_mode = move |tracked: bool| {
        let params = if tracked {
            use_params::<RecipeModeParam>().get()
        } else {
            use_params::<RecipeModeParam>().get_untracked()
        };
        params.unwrap_or_default().mode.expect("To get RecipeModeParam")
    };

    // Page Name setup
    set_page_name("Recipes");
    // Update Page Name
    create_effect(move |_| {
        set_page_name(
            match get_recipe_mode(true) {
                RecipePageMode::Display => "Display Recipe",
                RecipePageMode::Editable => "Edit Recipe",
                RecipePageMode::Print => "Print Recipe",
            }
        );
    });

    // Ensure the user is logged in when edit mode
    match get_recipe_mode(false) {
        RecipePageMode::Editable => {
            check_login_wall();
        },
        _ => (),
    }

    // Get recipe
    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    // Recipe resource
    let recipe_resource = create_resource(
        move || (
            recipe_action
                .version()
                .get(),
            get_recipe_id_param(true)
        ),
        move |(_, recipe_id)| {
            async move {
                match get_recipe_by_id(recipe_id).await {
                    Ok(recipe) => Some(recipe),
                    Err(e) => {
                        error!("Error fetching recipe by id: {:?}", e.to_string());
                        None
                    }
                }
            }
        },
    );


    view! {

        <DeleteRecipePopup/>

        <div class="main-content">
        <Transition fallback=move || view! { "Waiting for resource..." } >
            {move || {
                let recipe = recipe_resource.get();

                if let Some(Some(recipe)) = recipe {

                    match get_recipe_mode(true) {
                        RecipePageMode::Editable => {
                            // Editable Recipe
                            view! {
                                <EditableRecipeSheet
                                    recipe=         recipe
                                    is_new_recipe=  false
                                />
                            }
                        },
                        RecipePageMode::Display => {
                            // Display Recipe
                            view! {
                                <RecipeSheet
                                    recipe= recipe
                                />
                            }
                        },
                        RecipePageMode::Print => {
                            // Display Recipe
                            view! {
                                <PrintRecipeSheet
                                    recipe= recipe
                                />
                            }
                        }
                    }
                } else {
                    {"Recipe empty."}.into_view()
                }
            }}
        </Transition>
        </div>
    }
}



// Colors
#[derive(Clone, Copy)]
pub enum ThemeColor {
    Color1,
    Color2,
    Color3,
    Color4
}
impl ThemeColor {
    pub fn main_color(&self) -> String {
        match self {
            ThemeColor::Color1 => "var(--theme-color-1)",
            ThemeColor::Color2 => "var(--theme-color-2)",
            ThemeColor::Color3 => "var(--theme-color-3)",
            ThemeColor::Color4 => "var(--theme-color-4)",
        }.to_string()
    }
    pub fn alt_color(&self) -> String {
        match self {
            ThemeColor::Color1 => "var(--theme-color-1-alt)",
            ThemeColor::Color2 => "var(--theme-color-2-alt)",
            ThemeColor::Color3 => "var(--theme-color-3-alt)",
            ThemeColor::Color4 => "var(--theme-color-4-alt)",
        }.to_string()
    }

    pub fn as_bg_main_color(&self) -> String {
        "background-color: ".to_string()
        + &self.main_color()
        + ";"
    }
    pub fn as_bg_alt_color(&self) -> String {
        "background-color: ".to_string()
        + &self.alt_color()
        + ";"
    }
    pub fn as_main_color(&self) -> String {
        "color: ".to_string()
        + &self.main_color()
        + ";"
    }
    pub fn as_alt_color(&self) -> String {
        "color: ".to_string()
        + &self.alt_color()
        + ";"
    }
    pub fn as_border_main_color(&self) -> String {
        "border-color: ".to_string()
        + &self.main_color()
        + ";"
    }
    pub fn as_border_alt_color(&self) -> String {
        "border-color: ".to_string()
        + &self.alt_color()
        + ";"
    }
    pub fn as_visible_color(&self) -> String {
        let col = match self {
            ThemeColor::Color1 => self.main_color(),
            ThemeColor::Color2 => self.main_color(),
            ThemeColor::Color3 => self.alt_color(),
            ThemeColor::Color4 => self.alt_color(),
        };

        "color: ".to_string()
        + &col
        + ";"
    }

    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => ThemeColor::Color1,
            1 => ThemeColor::Color2,
            2 => ThemeColor::Color3,
            3 => ThemeColor::Color4,
            _ => unreachable!(),
        }
    }
}


// Popup Colors
#[derive(Clone, Copy)]
pub enum PopupColor {
    Color1,
    Color2,
    Color3,
}
impl PopupColor {
    pub fn window_background_color(&self) -> String {
        match self {
            PopupColor::Color1 => "background-color: var(--theme-color-4);",
            PopupColor::Color2 => "background-color: var(--theme-color-3);",
            PopupColor::Color3 => "background-color: var(--theme-color-bg);",
        }.to_string()
    }
    pub fn button_right_style(&self) -> String {
        match self {
            PopupColor::Color1 => "color: var(--theme-color-bg); background-color: var(--theme-color-popup-1);",
            PopupColor::Color2 => "color: var(--theme-color-bg); background-color: var(--theme-color-menu);",
            PopupColor::Color3 => "color: var(--theme-color-bg); background-color: var(--theme-color-popup-2);",
        }.to_string()
    }
    pub fn button_left_style(&self) -> String {
        match self {
            PopupColor::Color1 => "color: black; background-color: var(--theme-color-bg);",
            PopupColor::Color2 => "color: black; background-color: var(--theme-color-bg);",
            PopupColor::Color3 => "color: var(--theme-color-4-alt); background-color: var(--theme-color-4);",
        }.to_string()
    }

    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => PopupColor::Color1,
            1 => PopupColor::Color2,
            2 => PopupColor::Color3,
            _ => unreachable!(),
        }
    }
}




/// Renders the home page of your application.
#[component]
pub fn AllRecipes() -> impl IntoView {

    set_page_name("Recipes");

    // Is logged in
    let is_logged_in =
        use_context::<IsLoggedIn>()
            .expect("Expected to find IsLoggedIn in context.")
            .0;

    let selected_tags_signal =
        use_context::<SelectedTagsRwSignal>()
            .expect("To find SelectedTagsRwSignal in context.")
            .0;

    let search_input = create_rw_signal::<Vec<String>>(vec![]);

    let request_search_clear = create_rw_signal(false);

    let all_recipes_light =
        use_context::<RecipesLightResource>()
            .expect("To find RecipesLightResource in context.")
            .0;

    let all_tags_memo =
        use_context::<AllTagsSignal>()
            .expect("To find AllTagsMemo in context.")
            .0;
    
    let on_cancel_search_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        request_search_clear.set(true);
    };

    view! {

        <SettingsMenu/>

        <div class="logo" >
            <LogoSVG/>
        </div>

        // TagList
        <Transition fallback=move || view! { <LoadingElem text="Loading Recipes...".to_owned() /> } >
            { move || {
                let tags_component = {
                    move || {

                        view! {
                            <TagList
                                tags=all_tags_memo.get()
                                // Tags that are selected
                                selected_tags_signal=selected_tags_signal
                                // Tags that are already checked (needed because the component might redraw if tags are added or removed)
                                // This needs to be updated ONLY if tags are added or removed (through addind/removing recipes)
                                //already_selected_tags=already_selected_tags 
                            />
                        }
                    }
                };


                view! {
                    <div>

                        <DeleteRecipePopup/>

                        <Show
                            when=is_logged_in
                        >
                            <button
                                class="new-recipe-button"
                                on:click=move |ev: MouseEvent| {
                                    ev.stop_propagation();
                                    let navigate = leptos_router::use_navigate();
                                    navigate("/new-recipe", Default::default());
                                }
                            >
                                <span>"+"</span>
                            </button>
                        </Show>

                        <div class="search-container">
                            <div>
                                {tags_component}
                            </div>
                            <RecipeSearchBar
                                search_input=search_input
                                request_search_clear=request_search_clear
                            />
                        </div>

                        <div class="recipe-list-container">
                            {move || {
                                all_recipes_light.get()
                                    .map(move |recipes| match recipes {
                                        Err(e) => {
                                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                        }
                                        Ok(mut recipes) => {
                                            if recipes.is_empty() {
                                                view! { <p>"No recipes were found."</p> }.into_view()
                                            } else {
                                                let sel_tags = selected_tags_signal.get();
                                                let search_input_value = search_input.get();
                                                // filter tags
                                                if sel_tags.len() > 0 {
                                                    recipes.retain(|recipe| recipe.has_tags(&sel_tags));
                                                }
                                                // filter search
                                                if search_input_value.len() > 0 {
                                                    recipes.retain(|recipe| recipe.is_in_search(&search_input_value));
                                                }
                                                // If no results:
                                                if recipes.len() < 1 {
                                                    view! {
                                                        <div>
                                                            <p>"No results..."</p>
                                                            <button
                                                                //class="cancel-search-button"
                                                                on:click=on_cancel_search_click
                                                            >
                                                                "Cancel"
                                                            </button>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    // else collect recipe views
                                                    recipes
                                                        .into_iter()
                                                        .map(move |recipe| {
                                                            let style_color = ThemeColor::random();
                                                            view! {
                                                                <RecipeCard
                                                                    recipe_light=recipe
                                                                    custom_color_style=style_color
                                                                />
                                                            }
                                                        })
                                                        .collect_view()
                                                }
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }}
                        </div>
                    </div>
                }
            }}
        </Transition>
    }
}



/// Download all recipes button
/// Renders the home page of your application.
#[component]
pub fn BackupPage() -> impl IntoView {

    // Ensure we are logged in
    check_login_wall();

    let has_been_backed_up: RwSignal<bool> = create_rw_signal(false);

    view! {

        <SettingsMenu/>

        <button
            class="recipe-menu-button back backup-page"
            on:click=move |ev| {
                ev.stop_propagation();
                let navigate = leptos_router::use_navigate();
                navigate("/", Default::default());
            }
        >
            <BackButtonSVG backup_page=true />
        </button>

        //<h2>"Download current Cook Book save or Upload save to current Cook Book."</h2>
        <div class="save-page-container" >
            <DownloadAll
                has_been_backed_up = has_been_backed_up
            />
            <UploadAll
                has_been_backed_up = has_been_backed_up
            />
        </div>
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

