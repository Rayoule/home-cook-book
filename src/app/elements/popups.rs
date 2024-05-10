use leptos::{*, ev::MouseEvent, logging::log};

use crate::app::{
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


#[derive(Clone, Debug)]
pub struct DeletePopupInfo {
    pub wants_deletion: RwSignal<bool>,
    pub recipe_id: ReadSignal<u16>,
}
#[derive(Clone)]
pub struct DeleteInfoSetter(pub WriteSignal<Option<DeletePopupInfo>>);

#[component]
pub fn DeleteRecipePopup(
    recipe_action: Action<RecipeActionDescriptor, Result<(), ServerFnError>>,
    info: ReadSignal<Option<DeletePopupInfo>>,
) -> impl IntoView {

    let on_sure_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        if let Some(info) = info.get() {
            // dispatch recipe action with recipe ID
            recipe_action.dispatch(RecipeActionDescriptor::Delete(info.recipe_id.get_untracked()));
            // Set signal to end popup
            info.wants_deletion.set(false);
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
        <div
            class="popup"
            class:displayed=move || {
                if let Some(info) = info.get() {
                    info.wants_deletion.get()
                } else {
                    false
                }
            }
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
    }
}
