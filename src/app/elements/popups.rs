use leptos::{*, ev::MouseEvent, logging::log};

use crate::app::{
    DeleteInfoSignal, RecipeActionDescriptor, RecipeServerAction
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
pub struct DeletePopupInfo(pub u16);

#[component]
pub fn DeleteRecipePopup() -> impl IntoView {

    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    let delete_info_signal =
        use_context::<DeleteInfoSignal>()
            .expect("To find DeleteInfoSignal in context.")
            .0;

    let on_sure_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        if let Some(info) = delete_info_signal.get() {
            // dispatch recipe action with recipe ID
            let recipe_id = info.0;
            delete_info_signal.set(None);
            recipe_action.dispatch(RecipeActionDescriptor::Delete(recipe_id));
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        } else {
            log!("ERROR: DeletePopupInfo is None!");
        }
    };

    let on_no_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        delete_info_signal.set(None);
    };

    view! {
        <Show
            when=move || { delete_info_signal.get().is_some() }
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
