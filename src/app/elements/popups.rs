use gloo_timers::callback::Timeout;
use leptos::{ev::MouseEvent, leptos_dom, logging::*, prelude::*};

use crate::app::{DeleteInfoSignal, PopupColor, RecipeActionDescriptor, RecipeServerAction};



pub const BODY_STOP_SCROLL_CLASS: &'static str = "prevent-scroll";


#[component]
pub fn ServerActionPendingPopup() -> impl IntoView {
    let action_pending = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0
        .pending();

    let popup_color = RwSignal::new(PopupColor::random());
    Effect::new(move |_| {
        action_pending.track();
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
    let recipe_action = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0;

    let delete_info_signal = use_context::<DeleteInfoSignal>()
        .expect("To find DeleteInfoSignal in context.")
        .0;

    let on_sure_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        if let Some(info) = delete_info_signal.get() {
            // dispatch recipe action with recipe ID
            let recipe_id = info.0;
            delete_info_signal.set(None);
            recipe_action.dispatch(RecipeActionDescriptor::Delete(recipe_id));
            let navigate = leptos_router::hooks::use_navigate();
            navigate("/", Default::default());
        } else {
            error!("ERROR: DeletePopupInfo is None!");
        }
    };

    let on_no_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        // Close the Popup
        delete_info_signal.set(None);
    };

    let popup_color = RwSignal::new(PopupColor::random());
    Effect::new(move |_| {
        delete_info_signal.track();
        popup_color.set(PopupColor::random());
    });

    // Prevent Scrolling when Popup is enabled
    Effect::new(move |_| {
        if delete_info_signal.get().is_some() {
            leptos_dom::helpers::document().body().unwrap().class_list().add_1(BODY_STOP_SCROLL_CLASS)
        } else {
            leptos_dom::helpers::document().body().unwrap().class_list().remove_1(BODY_STOP_SCROLL_CLASS)
        }
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
                    <p class="popup-text"> { "Delete recipe ?" } </p>
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


#[component]
pub fn ServerWarningPopup(text: String) -> impl IntoView {

    let is_visible = RwSignal::new(false);

    // Wait for 0.5s to display the popup
    Effect::new(move |_| {
        let timeout = Timeout::new(500, move || {
            is_visible.set(true);
        });
        timeout.forget();
    });

    view! {
        <p
            class="popin-warning "
            class:visible=move || { is_visible.get() }
        >
            { text }
        </p>
    }
}

