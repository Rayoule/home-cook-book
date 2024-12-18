use crate::app::{
    components::{
        auth::{
            auth_server_functions::{server_login_check, server_try_login},
            auth_utils::LoginAccount,
        },
        pages::*,
        recipe::*,
        recipe_server_functions::{apply_json_save, get_all_recipes_light, recipe_function},
    },
    elements::popups::*,
};
use itertools::Itertools;
use leptos::logging::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod elements;

#[derive(Clone)]
pub struct PageNameSetter(WriteSignal<String>);
#[derive(Clone)]
pub struct IsLoggedIn(RwSignal<bool>);
#[derive(Clone)]
pub struct LoginCheckResource(Resource<(usize, usize, usize), bool>);
#[derive(Clone)]
pub struct IsSettingsMenuOpen(RwSignal<bool>);
#[derive(Clone)]
pub struct IsTagsMenuOpen(RwSignal<bool>);
#[derive(Clone)]
pub struct ApplySaveFromJson(Action<String, bool>);
#[derive(Clone)]
pub struct TryLoginAction(Action<LoginAccount, bool>);
#[derive(Clone)]
pub struct RecipeServerAction(Action<RecipeActionDescriptor, Result<(), ServerFnError>>);
#[derive(Clone)]
pub struct RecipesLightResource(Resource<(usize, usize), Result<Vec<RecipeLight>, ServerFnError>>);
#[derive(Clone)]
pub struct AllTagsSignal(RwSignal<Vec<String>>);
#[derive(Clone)]
pub struct SelectedTagsRwSignal(RwSignal<Vec<String>>);
#[derive(Clone)]
pub struct DeleteInfoSignal(RwSignal<Option<DeletePopupInfo>>);

#[component]
pub fn App() -> impl IntoView {
    log!("Rendering <App/>");

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // PageName signal
    let (_, set_page_name) = create_signal("".to_owned());
    provide_context(PageNameSetter(set_page_name));

    // Recipe Action
    let recipe_action =
        create_action(|desc: &RecipeActionDescriptor| recipe_function(desc.clone()));
    provide_context(RecipeServerAction(recipe_action));

    // Settings Menu
    let is_settings_menu_open = create_rw_signal(false);
    provide_context(IsSettingsMenuOpen(is_settings_menu_open));
    // Tags Menu
    let is_tags_menu_open = create_rw_signal(false);
    provide_context(IsTagsMenuOpen(is_tags_menu_open));

    // LOGIN
    // Add Is Logged In in context
    let is_logged_in_signal = create_rw_signal(false);
    provide_context(IsLoggedIn(is_logged_in_signal));

    // Redirect to "/" if logged in
    let rw_wants_redirect = create_rw_signal(false);
    create_effect(move |_| {
        if rw_wants_redirect.get() {
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        }
    });

    // Try login action
    let try_login_action = create_action(move |input: &LoginAccount| {
        let input = input.clone();
        async move {
            match server_try_login(input.clone()).await {
                Ok(login) => {
                    if login {
                        // If login was successful
                        rw_wants_redirect.set(true);
                        true
                    } else {
                        // If login failed
                        false
                    }
                }
                Err(e) => {
                    error!("Error trying login: {:?}", e.to_string());
                    false
                }
            }
        }
    });
    provide_context(TryLoginAction(try_login_action));

    // reload action
    let reload_action = create_action(|_: &()| {
        //login_check_resource.refetch();
        async { () }
    });
    // This is needed so the login_check_resource is evaluated again on refresh
    reload_action.dispatch(());

    // Login Check Resource
    let login_check_resource = create_resource(
        move || {
            (
                try_login_action.version().get(),
                recipe_action.version().get(),
                reload_action.version().get(),
            )
        },
        move |_| async move {
            match server_login_check().await {
                Ok(succeeded) => {
                    if succeeded {
                        is_logged_in_signal.set(true);
                        true
                    } else {
                        is_logged_in_signal.set(false);
                        false
                    }
                }
                Err(e) => {
                    error!("Error checking login: {:?}", e.to_string());
                    false
                }
            }
        },
    );
    provide_context(LoginCheckResource(login_check_resource));

    // Apply save from JSON
    let upload_save_action = create_action(|save: &String| {
        let save = save.to_string();
        async move {
            match apply_json_save(save).await {
                Err(e) => {
                    error!("ERROR: {:?}", e.to_string());
                    false
                }
                _ => true,
            }
        }
    });
    provide_context(ApplySaveFromJson(upload_save_action));

    // All RecipeLight resource
    let all_recipe_light = create_local_resource(
        move || {
            (
                recipe_action.version().get(),
                upload_save_action.version().get(),
            )
        },
        move |_| get_all_recipes_light(),
    );
    provide_context(RecipesLightResource(all_recipe_light));

    // All Tags signal
    let all_tags_signal = create_rw_signal::<Vec<String>>(vec![]);
    create_effect(move |_| {
        let recipes = all_recipe_light.get();
        let mut tag_list = if let Some(Ok(recipes)) = recipes {
            recipes
                .iter()
                .map(|recipe| recipe.tags.clone().unwrap_or_else(|| Vec::new()))
                .flatten()
                .map(|t| t.name)
                .unique()
                .collect::<Vec<String>>()
        } else {
            vec![]
        };
        tag_list.sort_by_key(|t| t.to_lowercase().clone());
        all_tags_signal.set(tag_list);
    });
    provide_context(AllTagsSignal(all_tags_signal));

    // Selected Tags
    let selected_tags = create_rw_signal::<Vec<String>>(vec![]);
    provide_context(SelectedTagsRwSignal(selected_tags));

    // Delete Infos: If this is Some(id), then display the popup that will delete the recipe with this id
    let delete_popup_info = create_rw_signal::<Option<DeletePopupInfo>>(None);
    provide_context(DeleteInfoSignal(delete_popup_info));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/home-cook-book.css"/>

        // sets the document title
        <Title text="Home Cook Book"/>


        // content for this welcome page
        <Router>

            <main>

                <ServerActionPendingPopup/>

                <CheckLogin/>

                <Routes>
                    <Route path="/"                     view=AllRecipes />
                    <Route path="/new-recipe"           view=NewRecipePage />
                    <Route path="/recipe/:id/:mode"     view=RecipePage />
                    <Route path="/backup"               view=BackupPage />
                    <Route path="/*"                    view=NotFound />
                </Routes>

            </main>

        </Router>
    }
}

pub fn set_page_name(name: &str) {
    use_context::<PageNameSetter>()
        .expect("to find PageNameSetter in context!")
        .0
        .set(name.to_owned());
}

#[component(transparent)]
pub fn CheckLogin() -> impl IntoView {
    let check_login_resource = use_context::<LoginCheckResource>()
        .expect("Expected to find LoginCheckAction in context")
        .0;

    view! {
        <Suspense
            fallback=move || {
                let is_print_page =
                    use_location()
                        .pathname
                        .get()
                        .split('/')
                        .last()
                        .is_some_and(|last_word| last_word == "print");

                if !is_print_page {
                    view! {
                        <p class="login-check-warning" >
                            "Wait for Login Check..."
                        </p>
                    }.into_view()
                } else { ().into_view() }
            }
        >
            {move || {
                let _ = check_login_resource.get();
            }.into_view()}
        </Suspense>
    }
}

pub fn check_login_wall() {
    let is_logged_in = use_context::<IsLoggedIn>()
        .expect("Expected to find IsLoggedIn in context")
        .0;
    create_effect(move |_| {
        if !is_logged_in.get() {
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        }
    });
}
