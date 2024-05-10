use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::components::{
    recipe::*,
    pages::*,
};

pub mod components;
pub mod elements;


#[derive(Clone)]
pub struct PageNameSetter(WriteSignal<String>);

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (get_page_name, set_page_name) = create_signal("".to_owned());
    provide_context(PageNameSetter(set_page_name));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/home-cook-book.css"/>

        // sets the document title
        <Title text="Home Cook Book"/>

        // content for this welcome page
        <Router>
            <main>
                <HeaderMenu
                    page_name=get_page_name
                />
                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                 >
                    <Route path="/"                     view=AllRecipes />
                    <Route path="/new-recipe"           view=NewRecipePage />
                    <Route path="/display-recipe/:id"   view=|| view! { <RecipePage editable=RecipePageMode::Display /> }/>
                    <Route path="/edit-recipe/:id"      view=|| view! { <RecipePage editable=RecipePageMode::Editable /> }/>
                    <Route path="/print-recipe/:id"     view=|| view! { <RecipePage editable=RecipePageMode::Print /> }/>
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
