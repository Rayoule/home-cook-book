use components::recipe_server_functions::apply_json_save;
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
        auth::{
            auth_utils::LoginAccount, auth_server_functions::{
                server_try_login, server_login_check
            },
        },
    },
    elements::popups::*,
};

pub mod components;
pub mod elements;


#[derive(Clone)]
pub struct PageNameSetter(WriteSignal<String>);
#[derive(Clone)]
pub struct IsLoggedIn(RwSignal<bool>);
#[derive(Clone)]
pub struct LoginCheckResource(Resource<(usize, usize, usize), bool>);
#[derive(Clone)]
pub struct IsPrintMode(RwSignal<bool>);
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


    // Recipe Action
    let recipe_action = 
        create_action(|desc: &RecipeActionDescriptor| {
            recipe_function(desc.clone())
        });
    provide_context(RecipeServerAction(recipe_action));




    // LOGIN

    // Add Is Logged In in context
    let is_logged_in_signal = create_rw_signal(false);
    provide_context(IsLoggedIn(is_logged_in_signal));
    create_effect(move |_| {
        log!("IsLoggedIn changed to -> {:?}", is_logged_in_signal.get());
    });


    // Redirect to "/" if logged in
    let rw_wants_redirect = create_rw_signal(false);
    create_effect(move |_| {
        if rw_wants_redirect.get() {
            log!("GOING to navigate to home !");
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        }
    });

    // Try login action
    let try_login_action = create_action( move |input: &LoginAccount| {
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
                },
                Err(e) => {
                    log!("Error trying login: {:?}", e.to_string());
                    false
                },
            }
        }
    });
    provide_context(TryLoginAction(try_login_action));

    
    // reload action
    let reload_action = create_action( |_: &()| {
        //login_check_resource.refetch();
        async { log!("RELOAD!"); }
    });
    // This is needed so the login_check_resource is evaluated again on refresh
    reload_action.dispatch(());

    // Login Check Resource
    let login_check_resource = create_resource(
        move || (
            try_login_action.version().get(),
            recipe_action.version().get(),
            reload_action.version().get()
        ),
        move |_| {
            log!("RESOURCE -> Checking login"); 
            async move {
                match server_login_check().await {
                    Ok(succeeded) => {
                        if succeeded {
                            log!("Login Check RESOURCE Succeeded.");
                            is_logged_in_signal.set(true);
                            true
                        } else {
                            log!("Login Check RESOURCE Failed.");
                            is_logged_in_signal.set(false);
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


    // Apply save from JSON
    let upload_save_action = create_action(|save: &String| {
        let save = save.to_string();
        async move {
            match apply_json_save(save).await {
                Err(e) => {
                    log!("ERROR: {:?}", e.to_string());
                    false
                },
                _ => true,
            }
        }
    });
    provide_context(ApplySaveFromJson(upload_save_action));
    

    // All RecipeLight resource
    let all_recipe_light = create_local_resource(
        move || (
            recipe_action.version().get(),
            upload_save_action.version().get()
        ),
        move |_| { get_all_recipes_light() },
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
        //log!("All Ingredients:\n{:?}", &ingr_list);
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
        //log!("All Tags:\n{:?}", &tag_list);
        all_tags_signal.set(tag_list);
    });
    provide_context(AllTagsSignal(all_tags_signal));

    // Selected Tags
    let selected_tags = create_rw_signal::<Vec<String>>(vec![]);
    create_effect(move |_| {
        log!("Selected tags changed:\n{:?}", selected_tags.get());
    });
    provide_context(SelectedTagsRwSignal(selected_tags));

    // Print Mode
    let is_print_mode = create_rw_signal(false);
    create_effect(move |_| {
        log!("BOUUUUUM");
        let print_mode = use_params::<RecipeModeParam>()
            .get()
            .unwrap_or_default()
            .mode
            .is_some_and(|page_mode| page_mode == RecipePageMode::Print);
        let print_mode = 
            use_params_map().get().get("mode").is_some_and(|mode| {
                mode.to_owned() == "print".to_string()
            });
        is_print_mode.set(print_mode);
    });

    provide_context(IsPrintMode(is_print_mode));

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

            <HeaderMenu
                page_name=get_page_name
            />

            <main>

                <ServerActionPendingPopup/>
                <CheckLogin/>

                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                >
                    /*<Route path="/"                     view=|| view! {<CheckLogin> <AllRecipes/> </CheckLogin>} />
                    <Route path="/login"                view=|| view! {<CheckLogin is_login_page=true > <LoginPage/> </CheckLogin>} />
                    <Route path="/new-recipe"           view=|| view! {<CheckLogin> <NewRecipePage/> </CheckLogin>} />
                    <Route path="/recipe/:id/:mode"     view=|| view! {<CheckLogin> <RecipePage/> </CheckLogin>} />
                    <Route path="/download-all"         view=|| view! {<CheckLogin> <SavePage/> </CheckLogin>} />*/
                    <Route path="/"                     view=AllRecipes />
                    <Route path="/login"                view=LoginPage />
                    <Route path="/new-recipe"           view=NewRecipePage />
                    <Route path="/recipe/:id/:mode"     view=RecipePage />
                    <Route path="/download-all"         view=SavePage />
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
    //children: ChildrenFn,
)-> impl IntoView
{
    let check_login_resource =
        use_context::<LoginCheckResource>()
            .expect("Expected to find LoginCheckAction in context")
            .0;

    view!{
        <Suspense
            fallback=move || view!{ <p>{"Wait for Login Check..."}</p> }
        >
            {move || {
                match check_login_resource.get() {
                    Some(login_result) => {
                        if login_result {
                            log!("Login check: Success!");
                        } else {
                            log!("Login check: Fail.");
                        }
                    },
                    None    => log!("Login check: Login Pending..."),
                }
            }.into_view()}
        </Suspense>
    }
}


/*#[component(transparent)]
pub fn CheckLogin(
    children: ChildrenFn,
    #[prop(optional)]
    is_login_page: bool,
)-> impl IntoView
{
    let check_login_resource = use_context::<LoginCheckResource>().expect("Expected to find LoginCheckAction in context").0;
    let children_stored = store_value(children);

    let rw_enable_login = create_rw_signal(true);

    // If login check failed, then redirect to login
    create_effect(move |_| {
        if !rw_enable_login.get() {
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
                    let login_check = check_login_resource.get();
                    // We validate login if the resource is None so it can work on the server.
                    // On the client, the Suspense will prevent the children to display anyway.
                    let login_check_result = login_check.is_none_or(|x| x);

                    // Debug
                    //let login_check_result = if let Some(res) = login_check {res} else {true};
                    //log!("LOGIN CHECK PROCESS start ------------------------------------");
                    //log!("Check Login Resource: --> {:?}", login_check);
                    //log!("Is Login Page:        --> {:?}", is_login_page);
                    //log!("LOGIN CHECK PROCESS end ------------------------------------");

                    let check = is_login_page || login_check_result; //|| is_logged_in.get();
                    rw_enable_login.set(check);
                    check
                }
                fallback=move || {
                    view! {<p>{"Login Failed."}</p>}
                }
            >
                { children_stored.with_value(|children| children()) }
            </Show>
        </Suspense>
    }
}*/
