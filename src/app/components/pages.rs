use leptos::ev::MouseEvent;
use leptos::logging::*;
use leptos::prelude::*;
use leptos_router::params::{Params, ParamsError};
use leptos::html;
use leptos::ev;
use leptos_router::hooks::use_params;
use std::sync::Arc;
use crate::app::components::recipe_sheets::PrintRecipeSheet;
use elements::{
    icons_svg::{BackButtonSVG, LogoSVG, PlusIconSVG},
    recipe_elements::SettingsMenu,
};

use crate::app::{
    components::{
        download_upload::{DownloadAll, UploadAll},
        recipe_server_functions::*,
        recipe_sheets::{EditableRecipeSheet, RecipeCard, RecipeSheet},
        tags::*,
    },
    elements::molecules::*,
    *,
};

#[component]
pub fn LoginMenu() -> impl IntoView {

    // get settings menu context
    let is_settings_menu_open = use_context::<IsSettingsMenuOpen>()
        .expect("Expected to find IsSettingsMenuOpen in context.")
        .0;

    let try_login_action = use_context::<TryLoginAction>()
        .expect("Expected to find TryLoginAction in context.")
        .0;
    let try_login_action_value = try_login_action.value();
    Effect::new(move |_| {
        // If login is succesful, then close settings menu
        if try_login_action_value.get().is_some_and(|result| result) {
            is_settings_menu_open.set(false);
        }
    });

    // setup submission signals (username, password)
    let submission = signal((String::new(), String::new()));

    // name input noderef
    let name_input: NodeRef<html::Input> = NodeRef::new();
    // password input noderef
    let password_input: NodeRef<html::Input> = NodeRef::new();

    // Handler for form submission
    let submit_event = move |event: ev::SubmitEvent| {
        event.prevent_default(); // Prevent the default form submission

        let login_account = LoginAccount {
            username: name_input
                .get()
                .expect("name <input> should be mounted")
                .value(),
            password: password_input
                .get()
                .expect("password <input> should be mounted")
                .value(),
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

    // Prevents Refresh on this page
    let handle = window_event_listener(
        leptos::ev::beforeunload,
        move |event: web_sys::BeforeUnloadEvent| {
            event.prevent_default();
            event.set_return_value("Are you sure you want to leave?");
            log::info!("Do something here");
        },
    );
    on_cleanup(move || handle.remove());

    // Is page Dirty Signal (to know if we need to save it before leaving)
    let is_page_dirty = RwSignal::new(false);
    provide_context(IsPageDirtySignal(is_page_dirty));

    // Setup action
    let recipe_action = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0;
    let action_submitted = recipe_action.input();
    let action_done_id = recipe_action.value();

    // store the submitted recipe name
    let submitted_name = RwSignal::new("".to_owned());
    Effect::new(move |_| {
        if let Some(action_desc) = action_submitted.get() {
            if let RecipeActionDescriptor::Add(recipe) = action_desc { submitted_name.set(recipe.name) }
        }
    });

    // Action that takes the recipe name to fetch the recipe ID
    // and then redirect to the edit page for this recipe
    let fetch_id_and_redirect = Action::new(|name: &String| {
        let name = name.clone();
        async move {
            match get_recipe_id_by_name(name.clone()).await {
                Ok(id) => {
                    if let Some(id) = id {
                        let path = "/recipe/".to_string() + &id.to_string() + "/display";
                        let navigate = leptos_router::hooks::use_navigate();
                        navigate(&path, Default::default());
                    } else {
                        error!("Error fetching recipe by name, no ID fetched.")
                    }
                }
                Err(_) => error!("Error fetching recipe by name with name: {:?}", name),
            }
        }
    });

    let fetch_and_redirect_pending = fetch_id_and_redirect.pending();

    // Once the recipe submission is done (when 'action_done_id' is Some)
    // grab the name and launch the 'fetch_id_and_redirect' action
    Effect::new(move |_| {
        if let Some(r) = action_done_id.get() {
            match r {
                Ok(_) => {
                    let name = submitted_name.get();
                    if name.is_empty() {
                        error!("ERROR: Won't fetch the id with an empty recipe name.");
                    } else {
                        fetch_id_and_redirect.dispatch(name);
                    }
                }
                Err(e) => error!(
                    "ERROR: Error in getting recipe submission ID: {:?}",
                    e.to_string()
                ),
            }
        }
    });

    view! {

        <CheckLogin/>

        <div
            class="main-content"
        >
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



#[derive(Clone)]
pub struct IsPageDirtySignal(pub RwSignal<bool>);

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
        params
            .unwrap_or_default()
            .mode
            .expect("To get RecipeModeParam")
    };

    // Is page Dirty Signal (to know if we need to save it before leaving)
    let is_page_dirty = RwSignal::new(false);
    provide_context(IsPageDirtySignal(is_page_dirty));

    // Page Name setup
    set_page_name("Recipes");

    // Update Page Name
    Effect::new(move |_| {
        set_page_name(match get_recipe_mode(true) {
            RecipePageMode::Display => "Display Recipe",
            RecipePageMode::Editable => "Edit Recipe",
            RecipePageMode::Print => "Print Recipe",
        });
    });

    // Get recipe
    let recipe_action = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0;

    // Recipe resource
    let recipe_resource = Resource::new(
        move || (recipe_action.version().get(), get_recipe_id_param(true)),
        move |(_, recipe_id)| async move {
            match get_recipe_by_id(recipe_id).await {
                Ok(recipe) => Some(recipe),
                Err(e) => {
                    error!("Error fetching recipe by id: {:?}", e.to_string());
                    None
                }
            }
        },
    );

    
    Effect::new(move |_| {
        if get_recipe_mode(true) == RecipePageMode::Editable && is_page_dirty.get() {
            let handle = window_event_listener(
                leptos::ev::beforeunload,
                move |event: web_sys::BeforeUnloadEvent| {
                    event.prevent_default();
                    event.set_return_value("Are you sure you want to leave?");
                },
            );
            on_cleanup(move || handle.remove());
        }
    });

    view! {

        <Show
            when=move || get_recipe_mode(true) == RecipePageMode::Editable
        >
            <CheckLogin/>
        </Show>

        <DeleteRecipePopup/>

        <div class="main-content">
        <Transition
            fallback=move || view! {
                <ServerWarningPopup
                    text="Waiting for resource...".to_string()
                />
            }
        >
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
                            }.into_any()
                        },
                        RecipePageMode::Display => {
                            // Display Recipe
                            view! {
                                <RecipeSheet
                                    recipe= recipe
                                />
                            }.into_any()
                        },
                        RecipePageMode::Print => {
                            // Display Recipe
                            view! {
                                <PrintRecipeSheet
                                    recipe= recipe
                                />
                            }.into_any()
                        }
                    }
                } else {
                    view! {
                        <ServerWarningPopup
                            text="Recipe empty.".to_string()
                        />
                    }.into_any()
                }
            }}
        </Transition>
        </div>
    }
}

// Colors
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemeColor {
    Color1,
    Color2,
    Color3,
    Color4,
    Undefined,
}
use ThemeColor::*;
impl ThemeColor {
    pub fn main_color(&self) -> String {
        match self {
            Color1  => "var(--theme-color-1)",
            Color2  => "var(--theme-color-2)",
            Color3  => "var(--theme-color-3)",
            Color4  => "var(--theme-color-4)",
            Undefined    => "var(--theme-color-undefined)",
        }
        .to_string()
    }
    pub fn alt_color(&self) -> String {
        match self {
            Color1  => "var(--theme-color-1-alt)",
            Color2  => "var(--theme-color-2-alt)",
            Color3  => "var(--theme-color-3-alt)",
            Color4  => "var(--theme-color-4-alt)",
            Undefined    => "var(--theme-color-undefined-alt)",
        }
        .to_string()
    }

    pub fn as_bg_main_color(&self) -> String {
        "background-color: ".to_string() + &self.main_color() + ";"
    }
    pub fn as_bg_alt_color(&self) -> String {
        "background-color: ".to_string() + &self.alt_color() + ";"
    }
    pub fn as_main_color(&self) -> String {
        "color: ".to_string() + &self.main_color() + ";"
    }
    pub fn as_alt_color(&self) -> String {
        "color: ".to_string() + &self.alt_color() + ";"
    }
    pub fn as_border_main_color(&self) -> String {
        "border-color: ".to_string() + &self.main_color() + ";"
    }
    pub fn as_border_alt_color(&self) -> String {
        "border-color: ".to_string() + &self.alt_color() + ";"
    }
    pub fn as_visible_color(&self) -> String {
        let col = match self {
            Color1  => self.main_color(),
            Color2  => self.main_color(),
            Color3  => self.alt_color(),
            Color4  => self.alt_color(),
            Undefined    => self.main_color(),
        };

        "color: ".to_string() + &col + ";"
    }

    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Color1,
            1 => Color2,
            2 => Color3,
            3 => Color4,
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
        }
        .to_string()
    }
    pub fn button_right_style(&self) -> String {
        match self {
            PopupColor::Color1 => {
                "color: var(--theme-color-bg); background-color: var(--theme-color-popup-1);"
            }
            PopupColor::Color2 => {
                "color: var(--theme-color-bg); background-color: var(--theme-color-menu);"
            }
            PopupColor::Color3 => {
                "color: var(--theme-color-bg); background-color: var(--theme-color-popup-2);"
            }
        }
        .to_string()
    }
    pub fn button_left_style(&self) -> String {
        match self {
            PopupColor::Color1 => "color: black; background-color: var(--theme-color-bg);",
            PopupColor::Color2 => "color: black; background-color: var(--theme-color-bg);",
            PopupColor::Color3 => {
                "color: var(--theme-color-4-alt); background-color: var(--theme-color-4);"
            }
        }
        .to_string()
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
    let check_login_resource = use_context::<LoginCheckResource>()
        .expect("Expected to find LoginCheckAction in context")
        .0;

    let selected_tags_signal = use_context::<SelectedTagsRwSignal>()
        .expect("To find SelectedTagsRwSignal in context.")
        .0;

    let search_input = RwSignal::<Vec<String>>::new(vec![]);

    let request_search_clear = RwSignal::new(false);

    let all_recipes_light = use_context::<RecipesLightResource>()
        .expect("To find RecipesLightResource in context.")
        .0;

    let all_tags_signal = use_context::<AllTagsSignal>()
        .expect("To find AllTagsMemo in context.")
        .0;

    let on_cancel_search_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        request_search_clear.set(true);
    };

    view! {

        <SettingsMenu/>

        <div
            class="logo"
            on:click=move |ev| {
                ev.stop_propagation();
                // Trigger Shuffle Colors
                log!("Shuffle Clicked !");
                use_context::<ShuffleColors>()
                    .expect("Expected to find ShuffleColors in context")
                    .0
                    .set(true);
            }
        >
            <LogoSVG/>
        </div>

        // TagList
        <Transition fallback=move || view! { <LoadingElem text="Loading Recipes...".to_owned() /> } >

            <div class="all-recipes">

                <DeleteRecipePopup/>

                <Transition
                    fallback=move || { view! {
                        <ServerWarningPopup
                            text="Wait for Login Check...".to_string()
                        />
                    }}
                >
                    <Show
                        when=move || { check_login_resource.get() == Some(true) }
                    >
                        <button
                            class="new-recipe-button"
                            on:click=move |ev: MouseEvent| {
                                ev.stop_propagation();
                                let navigate = leptos_router::hooks::use_navigate();
                                navigate("/new-recipe", Default::default());
                            }
                        >
                            <PlusIconSVG add_class="new-recipe".to_string() />
                        </button>
                    </Show>
                </Transition>

                <div class="search-container">
                    <TagList
                        all_tags=all_tags_signal
                        selected_tags_signal=selected_tags_signal
                    />
                    <RecipeSearchBar
                        search_input=search_input
                        request_search_clear=request_search_clear
                    />
                </div>

                <div class="recipe-list-container">
                    {move || {
                        all_recipes_light
                            .get()
                            .map(move |recipes| match recipes {
                                Err(e) => {
                                    view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_any()
                                }
                                Ok(mut recipes) => {
                                    if recipes.is_empty() {
                                        view! { <p>"No recipes were found."</p> }.into_any()
                                    } else {
                                        // Fetch selected tags and search inputs
                                        let sel_tags = selected_tags_signal.get();
                                        let search_input_value = search_input.get();

                                        // Give a new ID to each recipe so it can fetch into the color pool
                                        let mut recipes: Vec<(usize, RecipeLight)> =
                                            recipes
                                                .into_iter()
                                                .enumerate()
                                                .collect();

                                        
                                        // If no results:
                                        if recipes.is_empty() {

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
                                            }.into_any()

                                        } else {

                                            // filter tags
                                            if !sel_tags.is_empty() {
                                                recipes.retain(|recipe| recipe.1.has_tags(&sel_tags));
                                            }
                                            // filter search
                                            if !search_input_value.is_empty() {
                                                recipes.retain(|recipe| recipe.1.is_in_search(&search_input_value));
                                            }

                                            // Fetch the current Color Map
                                            let color_map = use_context::<RecipesColorMap>()
                                                .expect("Expected to find RecipesColorMap in context.")
                                                .0
                                                .get();
                                            // And don't forget to refresh the shuffle !
                                            use_context::<ShuffleColors>()
                                                .expect("Expected to find ShuffleColors in context.")
                                                .0
                                                .set(false);

                                            // Recipe Views
                                            recipes
                                                .into_iter()
                                                .map(move |(local_id, recipe)| {
                                                    let style_color = color_map
                                                        .get(local_id)
                                                        .copied()
                                                        .unwrap_or(ThemeColor::Undefined);

                                                    view! {
                                                        <RecipeCard
                                                            recipe_light=recipe
                                                            color=style_color
                                                        />
                                                    }
                                                })
                                                .collect_view()
                                                .into_any()
                                        }
                                    }
                                }
                            })
                            .unwrap_or(().into_any())
                    }}
                </div>
            </div>
        </Transition>
    }
}

/// Download all recipes button
/// Renders the home page of your application.
#[component]
pub fn BackupPage() -> impl IntoView {
    // Ensure we are logged in
    set_page_name("Backup");

    let has_been_backed_up: RwSignal<bool> = RwSignal::new(false);

    view! {

        <CheckLogin/>

        <SettingsMenu/>

        <button
            class="recipe-menu-button back backup-page"
            on:click=move |ev| {
                ev.stop_propagation();
                let navigate = leptos_router::hooks::use_navigate();
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
