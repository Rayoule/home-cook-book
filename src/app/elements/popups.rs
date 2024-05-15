use leptos::{*, ev::MouseEvent, logging::log};

use crate::app::{
    Recipe, RecipeActionDescriptor, RecipeServerAction
};


#[component]
pub fn ServerActionPendingPopup()  -> impl IntoView {

    let action_pending =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0
            .pending();
    
    view! {
        <Show
            when=move || action_pending.get()
        >
            <div
                class="popup"
                on:click= move |ev: MouseEvent| {
                    ev.stop_propagation();
                }
            >
                <div class="popup-window">
                    <p>{"Please Wait..."}</p>
                </div>
            </div>
        </Show>
    }
}


#[derive(Clone, Debug)]
pub struct DeletePopupInfo {
    pub wants_deletion: RwSignal<bool>,
    pub recipe_id: ReadSignal<u16>,
}
#[derive(Clone)]
pub struct DeleteInfoSetter(pub WriteSignal<Option<DeletePopupInfo>>);

#[component]
pub fn DeleteRecipePopup(
    info: ReadSignal<Option<DeletePopupInfo>>,
) -> impl IntoView {

    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    let on_sure_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        if let Some(info) = info.get() {
            // Set signal to end popup
            info.wants_deletion.set(false);
            // dispatch recipe action with recipe ID
            recipe_action.dispatch(RecipeActionDescriptor::Delete(info.recipe_id.get_untracked()));
        } else {
            log!("ERROR: DeletePopupInfo is None!");
        }
    };

    let on_no_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        if let Some(info) = info.get() {
            info.wants_deletion.set(false)
        } else {
            log!("ERROR: DeletePopupInfo is None!");
        }
    };

    view! {
        <Show
            when=move || {
                if let Some(info) = info.get() {
                    info.wants_deletion.get()
                } else {
                    false
                }
            }
        >
            <div
                class="popup"
                on:click=move |ev| {
                    ev.stop_propagation();
                }
            >
                <div class="popup-window">
                    <p> { "Do you wish to DELETE this recipe ?" } </p>
                    <button on:click=on_no_click > {"NO, CANCEL"} </button>
                    <button on:click=on_sure_click > {"YES, DELETE"} </button>
                </div>
            </div>
        </Show>
    }
}
