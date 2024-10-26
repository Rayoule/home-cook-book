use leptos::{
    *,
    ev::MouseEvent,
};

use crate::app::{
    elements::popups::DeletePopupInfo,
    DeleteInfoSignal,
    IsLoggedIn,
    IsPrintMode,
    RecipeActionDescriptor,
    RecipeServerAction,
};


/// The bool indicated if the function needs admin rights
#[derive(Clone)]
pub enum RoundMenuButton {
    Display,
    New,
    Edit,
    Duplicate,
    Print,
    Delete,
}

#[derive(Clone, Default)]
pub struct RoundMenuInfo {
    pub buttons: Option<Vec<RoundMenuButton>>,
    pub recipe_id: Option<u16>,
    pub hide_return_button: bool,
}

#[component]
pub fn RoundMenu(
    info: ReadSignal<RoundMenuInfo>,
) -> impl IntoView {

    // Is logged in
    let is_logged_in =
        use_context::<IsLoggedIn>()
            .expect("Expected to find IsLoggedIn in context.")
            .0;


    // Print mode ?
    let is_print_mode = 
        use_context::<IsPrintMode>()
            .expect("Expected to find IsPrintMode in context.")
            .0;

    // Recipe action
    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    // Unfolded Signal
    let is_unfolded = create_signal(false);

    let on_return_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    };

    view! {
        <Show
            when = move || { !is_print_mode.get() }
        >
            <Show
                when=move || !info.get().hide_return_button
            >
                <button
                    class="round-menu-return-button"
                    on:click=on_return_click
                >
                    {"↜"}
                </button>
            </Show>

            <div
                class="round-menu"
            >
                // in closure to make the signal responsive
                { move || {

                    let mut button_count: usize = 0;
                    let mut is_first_button = true;

                    let buttons_iter = info
                        .get()
                        .buttons
                        .unwrap_or_else(|| vec![])
                        .into_iter()
                        .map(|b| {

                            button_count += 1;

                            let button_class = if is_first_button {
                                is_first_button = false;
                                "round-menu-first-button rm-button".to_owned()
                            } else {
                                "round-menu-button rm-button".to_owned()
                            };

                            match b {
                                RoundMenuButton::Display => {
                                    let on_button_click = move |ev: MouseEvent| {
                                        ev.stop_propagation();
                                        let recipe_id = info.get().recipe_id.expect("to find recipe_id for button Display.");
                                        let print_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/display";
                                        let navigate = leptos_router::use_navigate();
                                        navigate(&print_path, Default::default());
                                    };

                                    view! {
                                        <Show
                                            when=move || { info.get().recipe_id.is_some() }
                                            fallback=move || ().into_view()
                                        >
                                            <button
                                                class=      { button_class.clone() + " display" }
                                                class:unfolded=is_unfolded.0
                                                on:click=   on_button_click
                                            >
                                                {"🔍 Display"}
                                            </button>
                                        </Show>
                                    }.into_view()
                                },

                                RoundMenuButton::New => {
                                    let on_button_click = move |ev: MouseEvent| {
                                        ev.stop_propagation();
                                        let navigate = leptos_router::use_navigate();
                                        navigate("/new-recipe", Default::default());
                                    };

                                    view! {
                                        <Show
                                            when=is_logged_in
                                        >
                                            <button
                                                class=      { button_class.clone() + " new" }
                                                class:unfolded=is_unfolded.0
                                                on:click=   on_button_click
                                            >
                                                {"+"}
                                            </button>
                                        </Show>
                                    }.into_view()
                                },

                                RoundMenuButton::Edit => {
                                    let on_button_click = move |ev: MouseEvent| {
                                        ev.stop_propagation();
                                        let recipe_id = info.get().recipe_id.expect("to find recipe_id for button Edit.");
                                        let edit_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/editable";
                                        let navigate = leptos_router::use_navigate();
                                        navigate(&edit_path, Default::default());
                                    };

                                    view! {
                                        <Show
                                            when=move || {
                                                info.get().recipe_id.is_some()
                                                && is_logged_in.get()
                                            }
                                            //fallback=move || ().into_view()
                                        >
                                            <button
                                                class=      { button_class.clone() + " edit" }
                                                class:unfolded=is_unfolded.0
                                                on:click=   on_button_click
                                            >
                                                {"✏️ Edit"}
                                            </button>
                                        </Show>
                                    }.into_view()
                                },

                                RoundMenuButton::Duplicate => {
                                    let on_button_click = move |ev: MouseEvent| {
                                        ev.stop_propagation();
                                        let info = info.get();
                                        let recipe_id = info.recipe_id.expect("to find recipe_id for button Duplicate.");
                                        recipe_action.dispatch(RecipeActionDescriptor::Duplicate(recipe_id));
                                    };

                                    view! {
                                        <Show
                                            when=move || {
                                                info.get().recipe_id.is_some()
                                                && is_logged_in.get()
                                            }
                                            //fallback=move || ().into_view()
                                        >
                                            <button
                                                class=      { button_class.clone() + " duplicate" }
                                                class:unfolded=is_unfolded.0
                                                on:click=   on_button_click
                                            >
                                                {"⧉ Duplicate"}
                                            </button>
                                        </Show>
                                    }.into_view()
                                },

                                RoundMenuButton::Print => {
                                    let on_button_click = move |ev: MouseEvent| {
                                        ev.stop_propagation();
                                        let recipe_id = info.get().recipe_id.expect("to find recipe_id for button Duplicate.");
                                        let print_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/print";
                                        let window = web_sys::window().expect("window should be available");
                                        window
                                            .open_with_url_and_target(&print_path, "_blank")
                                            .unwrap();
                                    };

                                    view! {
                                        <Show
                                            when=move || { info.get().recipe_id.is_some() }
                                            //fallback=move || ().into_view()
                                        >
                                            <button
                                                class=      { button_class.clone() + " print" }
                                                class:unfolded=is_unfolded.0
                                                on:click=   on_button_click
                                            >
                                                {"📄 Print"}
                                            </button>
                                        </Show>
                                    }.into_view()
                                },

                                RoundMenuButton::Delete => {

                                    let delete_info_signal =
                                            use_context::<DeleteInfoSignal>()
                                                .expect("To find DeleteInfoSignal in context.")
                                                .0;
                                    let recipe_id = info.get().recipe_id.expect("to find recipe_id for button Delete.");
                                    
                                    let on_button_click = move |ev: MouseEvent| {
                                        ev.stop_propagation();
                                        delete_info_signal.set( Some( DeletePopupInfo(recipe_id)) );
                                    };

                                    view!{
                                        <Show
                                            when=is_logged_in
                                        >
                                            <button
                                                    class=      { button_class.clone() + " delete" }
                                                    class:unfolded=is_unfolded.0
                                                    on:click=   on_button_click
                                            >
                                                {"Delete 🗑️"}
                                            </button>
                                        </Show>
                                    }.into_view()
                                },
                            }
                        });
                    
                    
                    
                    // Store 1st button
                    //let first_button = buttons_iter.next();
                    // Collect all other
                    //let other_buttons = buttons_iter.collect_view();

                    buttons_iter.collect_view()

                    /*view! {

                        // First button
                        <div
                            class="rm-butons-container"
                            class:not-needed={ move || button_count < 1 }
                        >
                            //{ first_button.clone() }
                            
                        </div>

                        // Other buttons
                        <button
                            class="round-menu-unfold-button rm-button"
                            class:unfolded=is_unfolded.0
                            class:not-needed={ move || button_count < 2 }
                            on:click=on_unfold_click
                        >
                            { move ||
                                if is_unfolded.0.get() {
                                    "v"
                                } else {
                                    "• • •"
                                }
                            }
                        </button>
                        <div
                            class="round-menu-other-buttons"
                            class:unfolded=is_unfolded.0
                            class:not-needed={ move || button_count < 2 }
                        >
                            { other_buttons.clone() }
                        </div>

                    }*/
                }}
            </div>
        </Show>
    }
}

