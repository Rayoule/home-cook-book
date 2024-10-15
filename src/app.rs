use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::logging::log;

use itertools::Itertools;

use crate::app::{
    components::{
        pages::*,
        recipe::*,
        recipe_server_functions::{get_all_recipes_light, recipe_function}, round_menu::*,
        auth::auth_server_functions::server_login_check,
    },
    elements::popups::*,
};

pub mod components;
pub mod elements;


#[derive(Clone)]
pub struct PageNameSetter(WriteSignal<String>);
/// LoginCheckAction returns None if the login attempt failed
#[derive(Clone)]
pub struct LoginCheckResource(Resource<(), bool>);
#[derive(Clone)]
pub struct CurrrentUsername(RwSignal<String>);
#[derive(Clone)]
pub struct RecipeServerAction(Action<RecipeActionDescriptor, Result<(), ServerFnError>>);
#[derive(Clone)]
pub struct RecipesLightResource(Resource<usize, Result<Vec<RecipeLight>, ServerFnError>>);
#[derive(Clone)]
pub struct AllTagsSignal(RwSignal<Vec<String>>);
#[derive(Clone)]
pub struct AllIngredientsSignal(RwSignal<Vec<String>>);
#[derive(Clone)]
pub struct SelectedTagsRwSignal(RwSignal<Vec<String>>);
#[derive(Clone)]
pub struct DeleteInfoSignal(RwSignal<Option<DeletePopupInfo>>);

#[component]
pub fn App() -> impl IntoView {
    log!("Rendering App");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // PageName signal
    let (get_page_name, set_page_name) = create_signal("".to_owned());
    provide_context(PageNameSetter(set_page_name));

    // Current Username
    let current_username = create_rw_signal("".to_string());
    provide_context(CurrrentUsername(current_username));

    // Login Check Action
    let login_check_resource = 
        create_resource(
            || (),
            |_| {
                let rw_username = use_context::<CurrrentUsername>()
                    .expect("Expected to find CurrrentUsername in context.")
                    .0;
                
                async move {
                    let username = rw_username.get();

                    match server_login_check(username.clone()).await {
                        Ok(succeeded) => {
                            if succeeded {
                                log!("Login Check Succeeded.");
                                rw_username.set(username);
                                true
                            } else {
                                log!("Login Check Failed.");
                                false
                            }
                        },
                        Err(e) => {
                            log!("Error checking login: {:?}", e.to_string());
                            false
                        },
                        }
                    }
            }
        );
    provide_context(LoginCheckResource(login_check_resource));


    // Recipe Action
    let recipe_action = 
        create_action(|desc: &RecipeActionDescriptor| {
            recipe_function(desc.clone())
        });
    provide_context(RecipeServerAction(recipe_action));

    // All RecipeLight resource
    let all_recipe_light = create_local_resource(
        move || recipe_action.version().get(),
        move |_| {
            log!("Fetching all resources!");
            get_all_recipes_light()
        },
    );
    provide_context(RecipesLightResource(all_recipe_light));

    // All Ingredients signal
    let all_ingredients_signal = create_rw_signal::<Vec<String>>(vec![]);
    create_effect(move |_| {
        let recipes = all_recipe_light.get();
        let mut ingr_list =
            if let Some(Ok(recipes)) = recipes {
                recipes
                    .iter()
                    .map(|recipe| recipe.ingredients.clone().unwrap_or_else(|| vec![]) )
                    .flatten()
                    .map(|t| t.content)
                    .unique()
                    .collect::<Vec<String>>()
            } else { vec![] };
        ingr_list.sort_by_key(|t| t.to_lowercase().clone());
        log!("All Ingredients:\n{:?}", &ingr_list);
        all_ingredients_signal.set(ingr_list);
    });
    provide_context(AllIngredientsSignal(all_ingredients_signal));

    // All Tags signal
    let all_tags_signal = create_rw_signal::<Vec<String>>(vec![]);
    create_effect( move |_| {
        let recipes = all_recipe_light.get();
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
        log!("All Tags:\n{:?}", &tag_list);
        all_tags_signal.set(tag_list);
    });
    provide_context(AllTagsSignal(all_tags_signal));

    // Selected Tags
    let selected_tags = create_rw_signal::<Vec<String>>(vec![]);
    create_effect(move |_| {
        log!("Selected tags changed:\n{:?}", selected_tags.get());
    });
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

        <HeaderMenu
            page_name=get_page_name
        />

        // content for this welcome page
        <Router>

            <main>

                <ServerActionPendingPopup/>

                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                >
                    <Route path="/"                     view=|| view! {<CheckLogin> <AllRecipes/> </CheckLogin>} />
                    <Route path="/login"                view=|| view! {<CheckLogin is_login_page=true > <LoginPage/> </CheckLogin>} />
                    <Route path="/new-recipe"           view=|| view! {<CheckLogin> <NewRecipePage/> </CheckLogin>} />
                    <Route path="/recipe/:id/:mode"     view=|| view! {<CheckLogin> <RecipePage/> </CheckLogin>} />
                    <Route path="/*"                    view=NotFound />
                </AnimatedRoutes>

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
pub fn CheckLogin(
    children: ChildrenFn,
    #[prop(optional)]
    is_login_page: bool,
)-> impl IntoView
{
    let check_login_resource = use_context::<LoginCheckResource>().expect("Expected to find LoginCheckAction in context").0;
    let view_stored = store_value(children);

    let rw_login_check_result = create_rw_signal(false);

    // If login check failed, then redirect to login
    create_effect(move |_| {
        if !rw_login_check_result.get() {
            let navigate = leptos_router::use_navigate();
            navigate("/login", Default::default());
        }
    });

    view!{
        <Suspense
            fallback=move || view!{ <p>{"Wait for Login Check..."}</p> }
        >
            <Show
                when=move || {
                    let result = check_login_resource.get().is_some_and(|x| x);
                    rw_login_check_result.set(result);
                    log!("{:?}", result);
                    result || is_login_page
                }
                fallback=move || {
                    view! {<p>{"Login Failed."}</p>}
                }
            >
                {view_stored.with_value(|view| view())}
            </Show>
        </Suspense>
    }
}
