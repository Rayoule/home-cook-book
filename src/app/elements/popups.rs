use leptos::*;

use crate::app::{
    components::recipe_sheets::{
        RecipeIdGetter, RecipeNameGetter
    },
    Recipe, RecipeActionDescriptor
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
    recipe_action: Action<RecipeActionDescriptor, Result<(), ServerFnError>>,
    wants_deletion_setter: WriteSignal<bool>,
) -> impl IntoView {

    // fetch recipe Name from context
    let recipe_name =
        use_context::<RecipeNameGetter>()
            .expect("to find recipe Name in context.")
            .0
            .get();

    let on_sure_click = move |_| {
        // fetch recipe ID from context
        let recipe_id =
            use_context::<RecipeIdGetter>()
                .expect("to find Recipe ID in context.")
                .0
                .get();
        // dispatch recipe action with recipe ID
        recipe_action.dispatch(RecipeActionDescriptor::Delete(recipe_id));
        // Set signal to end popup
        wants_deletion_setter.set(false);
    };

    let on_no_click = move |_| {
        wants_deletion_setter.set(false);
    };

    view! {
        <div class="popup">
            <div class="popup-window">
                <p> { format!("Do you wish to DELETE the recipe {:?}", recipe_name) } </p>
                <button on:click=on_no_click > {"NO, CANCEL"} </button>
                <button on:click=on_sure_click > {"YES, DELETE"} </button>
            </div>
        </div>
    }
}
