use leptos::{
    *,
    ev::MouseEvent, logging::log};

use crate::app::{elements::popups::DeletePopupInfo, RecipeActionDescriptor, RecipeServerAction};


#[derive(Clone)]
pub enum RoundMenuButton {
    HomePage,
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
    pub delete_info: Option<WriteSignal<Option<DeletePopupInfo>>>,
}
#[derive(Clone)]
pub struct RoundMenuWriteSignal(pub WriteSignal<Option<RoundMenuInfo>>);
#[derive(Clone)]
pub struct RoundMenuReadSignal(pub ReadSignal<Option<RoundMenuInfo>>);

#[component]
pub fn RoundMenu(
    info: ReadSignal<RoundMenuInfo>,
) -> impl IntoView {

    log!("Rendering round menu");

    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    // Unfolded Signal
    let is_unfolded = create_signal(false);
    // Toggle unfolded on click
    let on_unfold_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        is_unfolded.1.set(!is_unfolded.0.get())
    };

    view! {
        <div
            class="round-menu"
        >
            // in closure to make the signal responsive
            { move || {

                let mut button_count: usize = 0;
                let mut is_first_button = true;

                let mut buttons_iter = info
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

                            RoundMenuButton::HomePage => {
                                let on_button_click = move |ev: MouseEvent| {
                                    ev.stop_propagation();
                                    let navigate = leptos_router::use_navigate();
                                    navigate("/", Default::default());
                                };

                                view! {
                                    <div
                                        class=      { button_class.clone() + " homepage" }
                                        class:unfolded=is_unfolded.0
                                        on:click=   on_button_click
                                    >
                                        {"↩"}
                                    </div>
                                }.into_view()
                            },

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
                                        <div
                                            class=      { button_class.clone() + " display" }
                                            class:unfolded=is_unfolded.0
                                            on:click=   on_button_click
                                        >
                                            {"🔍"}
                                        </div>
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
                                    <div
                                        class=      { button_class.clone() + " new" }
                                        class:unfolded=is_unfolded.0
                                        on:click=   on_button_click
                                    >
                                        {"➕"}
                                    </div>
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
                                        when=move || { info.get().recipe_id.is_some() }
                                        fallback=move || ().into_view()
                                    >
                                        <div
                                            class=      { button_class.clone() + " edit" }
                                            class:unfolded=is_unfolded.0
                                            on:click=   on_button_click
                                        >
                                            {"✏️"}
                                        </div>
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
                                        when=move || info.get().recipe_id.is_some()
                                        fallback=move || ().into_view()
                                    >
                                        <div
                                            class=      { button_class.clone() + " duplicate" }
                                            class:unfolded=is_unfolded.0
                                            on:click=   on_button_click
                                        >
                                            {"⧉"}
                                        </div>
                                    </Show>
                                }.into_view()
                            },

                            RoundMenuButton::Print => {
                                let on_button_click = move |ev: MouseEvent| {
                                    ev.stop_propagation();
                                    let recipe_id = info.get().recipe_id.expect("to find recipe_id for button Duplicate.");
                                    let print_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/print";
                                    let navigate = leptos_router::use_navigate();
                                    navigate(&print_path, Default::default());
                                };

                                view! {
                                    <Show
                                        when=move || { info.get().recipe_id.is_some() }
                                        fallback=move || ().into_view()
                                    >
                                        <div
                                            class=      { button_class.clone() + " print" }
                                            class:unfolded=is_unfolded.0
                                            on:click=   on_button_click
                                        >
                                            {"📄"}
                                        </div>
                                    </Show>
                                }.into_view()
                            },

                            RoundMenuButton::Delete => {
                                let wants_deletion = create_rw_signal(false);
                                let on_button_click = move |ev: MouseEvent| {
                                    ev.stop_propagation();
                                    let info = info.get();
                                    let recipe_id = info.recipe_id.expect("to find recipe_id for button Delete.");
                                    let delete_info = info.delete_info.expect("to find delete_info for button Delete.");
                                    wants_deletion.set(true);
                                    delete_info.set(Some(
                                        DeletePopupInfo {
                                            wants_deletion: wants_deletion,
                                            recipe_id:      create_signal(recipe_id).0,
                                        }
                                    ));
                                };

                                view!{
                                    <Show
                                        when=move || {
                                            let info = info.get();
                                            info.recipe_id.is_some() && info.delete_info.is_some()
                                        }
                                        fallback=move || ().into_view()
                                    >
                                        <div
                                            class=      { button_class.clone() + " delete" }
                                            class:unfolded=is_unfolded.0
                                            on:click=   on_button_click
                                        >
                                            {"🗑️"}
                                        </div>
                                    </Show>
                                }.into_view()
                            },
                        }
                    });
                
                
                
                // Store 1st button
                let first_button = buttons_iter.next();
                // Collect all other
                let other_buttons = buttons_iter.collect_view();

                view! {

                    // First button
                    <div
                        class:not-needed={ move || button_count < 1 }
                    >
                        {first_button.clone()}
                    </div>

                    // Other buttons
                    <div
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
                    </div>
                    <div
                        class="round-menu-other-buttons"
                        class:unfolded=is_unfolded.0
                        class:not-needed={ move || button_count < 2 }
                    >
                        { other_buttons.clone() }
                    </div>

                }
            }}
        </div>
    }
}

