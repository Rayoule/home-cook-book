use leptos::*;

use crate::app::{
    Recipe, RecipeAction
};


#[component]
pub fn PendingPopup(
    #[prop(optional)]
    get_signal: Option<ReadSignal<bool>>,
)  -> impl IntoView {
    if let Some(get_signal) = get_signal {
        view! {
            <div
                class="popup"
                class:action-pending-hidden = move || !get_signal.get()
            >
                <div class="popup-window">
                    <p>{"Please Wait..."}</p>
                </div>
            </div>
        }
    } else {
        view! {
            <div
                class="popup"
            >
                <div class="popup-window">
                    <p>{"Please Wait..."}</p>
                </div>
            </div>
        }
    }
}


#[component]
pub fn DeleteRecipePopup(
    recipe_getter: ReadSignal<Recipe>,
    wants_deletion_setter: WriteSignal<bool>,
    recipe_action: Action<(ReadSignal<Recipe>, RecipeAction), Result<(), ServerFnError>>,
) -> impl IntoView {

    let on_sure_click = move |_| {
        recipe_action.dispatch((recipe_getter, RecipeAction::Delete));
        wants_deletion_setter.set(false);
    };
    let on_no_click = move |_| {
        wants_deletion_setter.set(false);
    };

    view! {
        <div class="popup">
            <div class="popup-window">
                <p> { format!("Do you wish to DELETE the recipe {:?}", recipe_getter.get().name) } </p>
                <button on:click=on_no_click > {"NO, CANCEL"} </button>
                <button on:click=on_sure_click > {"YES, DELETE"} </button>
            </div>
        </div>
    }
}
