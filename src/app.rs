use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::logging::log;

use itertools::Itertools;

use crate::app::{
    components::{
        pages::*,
        recipe::*,
        recipe_server_functions::{get_all_recipes_light, recipe_function}, round_menu::*
    },
    elements::popups::*,
};

pub mod components;
pub mod elements;


#[derive(Clone)]
pub struct PageNameSetter(WriteSignal<String>);
#[derive(Clone)]
pub struct RecipeServerAction(Action<RecipeActionDescriptor, Result<(), ServerFnError>>);
#[derive(Clone)]
pub struct RecipesLightResource(Resource<usize, Result<Vec<RecipeLight>, ServerFnError>>);
#[derive(Clone)]
pub struct AllTagsMemo(Memo<Vec<String>>);
#[derive(Clone)]
pub struct SelectedTagsRwSignal(RwSignal<Vec<String>>);

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

    // All RecipeLight resource
    let all_recipe_light = create_local_resource(
        move || recipe_action.version().get(),
        move |_| get_all_recipes_light(),
    );
    provide_context(RecipesLightResource(all_recipe_light));

    // All Tags resource
    let all_tags = create_memo(move |_| {
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
        tag_list
    });
    provide_context(AllTagsMemo(all_tags));

    // Selected Tags
    let selected_tags = create_rw_signal::<Vec<String>>(vec![]);
    create_effect(move |_| {
        log!("Selected tags changed:\n{:?}", selected_tags.get());
    });
    provide_context(SelectedTagsRwSignal(selected_tags));
    

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
                    <Route path="/"                     view=AllRecipes />
                    <Route path="/new-recipe"           view=NewRecipePage />
                    <Route path="/recipe/:id/:mode"     view=|| view! { <RecipePage/> }/>
                    <Route path="/*any"                 view=NotFound />
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