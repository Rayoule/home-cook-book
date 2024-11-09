use leptos::{*, ev::MouseEvent, logging::*};

use crate::app::{
    DeleteInfoSignal, PopupColor, RecipeActionDescriptor, RecipeServerAction, ThemeColor
};


#[component]
pub fn ServerActionPendingPopup()  -> impl IntoView {

    let action_pending =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0
            .pending();
    
    let popup_color = create_rw_signal(PopupColor::random());
    create_effect( move |_| {
        let _ = action_pending.track();
        popup_color.set(PopupColor::random());
    });
    
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
                <div
                    class="popup-window server-action"
                    style=popup_color.get().window_background_color()
                >
                    <p class="wait-for-server" > "Wait for server ..." </p>
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
            error!("ERROR: DeletePopupInfo is None!");
        }
    };

    let on_no_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        delete_info_signal.set(None);
    };

    let popup_color = create_rw_signal(PopupColor::random());
    create_effect( move |_| {
        let _ = delete_info_signal.track();
        popup_color.set(PopupColor::random());
    });

    view! {
        <Show
            when=move || { delete_info_signal.get().is_some() }
        >
            <div
                class="popup"
                on:click=on_no_click
            >
                <div
                    class="popup-window"
                    style=popup_color.get().window_background_color()
                >
                    <p class="popup-text"> { "Do you wish to DELETE this recipe ?" } </p>
                    <div class="popup-option-container" >
                        <button
                            class="popup-option"
                            style=popup_color.get().button_left_style()
                            on:click=on_no_click
                        >
                            <p class="popup-option-text" >"no"</p>
                        </button>
                        <button
                            class="popup-option"
                            style=popup_color.get().button_right_style()
                            on:click=on_sure_click
                        >
                            <p class="popup-option-text" >"yes"</p>
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
