use elements::icons_svg::{BackButtonSVG, BackupButtonSVG, CrossButtonSVG, EditButtonSVG, LogoutButtonSVG, PrintButtonSVG, SortDownSVG, SortUpSVG};
use leptos::{
    *, logging::*,
};
use components::{auth::auth_server_functions::server_logout, recipe};
use ev::MouseEvent;
use gloo_timers::callback::Timeout;
use crate::app::{
    *,
    elements::molecules::*
};


type RecipeSignals =
    RwSignal<(
        RwSignal<String>,
        RwSignal<Vec<(u16, (ReadSignal<RecipeTag>, WriteSignal<RecipeTag>))>>,
        RwSignal<Vec<(u16, (ReadSignal<RecipeIngredient>, WriteSignal<RecipeIngredient>))>>,
        (ReadSignal<RecipeInstruction>, WriteSignal<RecipeInstruction>),
        RwSignal<Vec<(u16, (ReadSignal<RecipeNote>, WriteSignal<RecipeNote>))>>
    )>;

#[component]
pub fn RecipeMenu(
    color: RwSignal<ThemeColor>,
    editable: bool,
    recipe_id: Option<u16>,
    recipe_static_name: String,
    #[prop(optional)]
    is_new_recipe: Option<bool>,
    #[prop(optional)]
    recipe_signals: Option<RecipeSignals>,
) -> impl IntoView {

    // Is logged in
    let is_logged_in =
        use_context::<IsLoggedIn>()
            .expect("Expected to find IsLoggedIn in context.")
            .0;

    let menu_open = create_rw_signal(false);

    if !editable {

        let recipe_id = recipe_id.expect("Expected recipe ID to be Some for non edit mode");

        view! {
    
            <div
                class="recipe-menu"
                class:is-open=menu_open
                class:not-logged-in=move || { !is_logged_in.get() }
                style=move || { color.get().as_bg_main_color() }
            >

                <button
                    style=move || { color.get().as_alt_color() }
                    class="recipe-menu-button back"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        let navigate = leptos_router::use_navigate();
                        navigate("/", Default::default());
                    }
                >
                    <BackButtonSVG/>
                </button>

                <button
                    style=move || { color.get().as_alt_color() }
                    class="recipe-menu-button menu"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        menu_open.update(|b| *b = !*b);
                    }
                >
                </button>

                <Show
                    when=move || { !menu_open.get() }
                >
                    <h2
                        style=move || { color.get().as_alt_color() }
                        class="display-recipe-name"
                        class:menu-open=menu_open
                    >
                        { recipe_static_name.clone() }
                    </h2>
                </Show>
    
                <Show
                    when=menu_open
                >
                    <Show
                        when=is_logged_in
                    >
                        <button
                            style=move || { color.get().as_alt_color() }
                            class="recipe-menu-option"
                            on:click=move |ev: MouseEvent| {
                                ev.stop_propagation();
                                let recipe_id = recipe_id;
                                let edit_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/editable";
                                let navigate = leptos_router::use_navigate();
                                navigate(&edit_path, Default::default());
                            }
                        >
                            <EditButtonSVG color=color.get().alt_color() />
                            <p class="recipe-menu-text" >"Edit"</p>
                        </button>
                    </Show>
    
                    <button
                        style=move || { color.get().as_alt_color() }
                        class="recipe-menu-option"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            let recipe_id = recipe_id;
                            let print_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/print";
                            let window = web_sys::window().expect("window should be available");
                            window
                                .open_with_url_and_target(&print_path, "_blank")
                                .unwrap_or_else(|_| {
                                    error!("No Window found.");
                                    None
                                });
                        }
                    >
                        <PrintButtonSVG color=color.get().alt_color() />
                        <p class="recipe-menu-text" >"Print"</p>
                    </button>
    
                    <Show
                        when=is_logged_in
                    >
                        <button
                            style=move || { color.get().as_alt_color() }
                            class="recipe-menu-option"
                            on:click=move |ev: MouseEvent| {
                                ev.stop_propagation();
                                let recipe_id = recipe_id;
                                let delete_info_signal =
                                    use_context::<DeleteInfoSignal>()
                                        .expect("To find DeleteInfoSignal in context.")
                                        .0;
                                delete_info_signal.set( Some( DeletePopupInfo(recipe_id)) );
                            }
                        >
                            <CrossButtonSVG color=color.get().alt_color() />
                            <p class="recipe-menu-text" >"Delete"</p>
                        </button>
                    </Show>
    
                </Show>
    
            </div>
        }.into_view()

    } else {

        let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;
        let save_pending = recipe_action.pending();

        let is_new_recipe = is_new_recipe.expect("Expected is_new_recipe to be provided.");

        let recipe_signals = recipe_signals.expect("Expected recipe_signals to be provided.");
        let ( name_signal, _, _, _, _ ) = recipe_signals.get_untracked();

        let on_save_click = move |ev: MouseEvent| {
            ev.stop_propagation();
            // Get recipe
            let signals = recipe_signals.get_untracked();
            // Gather recipe
            use components::recipe_sheets::fetch_entries_from_signals;
            let updated_recipe: Recipe = Recipe {
                id:             recipe_id,
                name:           signals.0.clone().get_untracked(),
                tags:           fetch_entries_from_signals(signals.1.get_untracked()),
                ingredients:    fetch_entries_from_signals(signals.2.get_untracked()),
                instructions:   signals.3.0.get_untracked(),
                notes:          fetch_entries_from_signals(signals.4.get_untracked()),
            };
    
            // Check recipe
            match updated_recipe.valid_for_save() {
                Ok(_) => {
                    if is_new_recipe {
                        recipe_action.dispatch(RecipeActionDescriptor::Add(updated_recipe));
                    } else {
                        let id = updated_recipe.id;
                        recipe_action.dispatch(RecipeActionDescriptor::Save(updated_recipe));
                        if let Some(id) = id {
                            let path = "/recipe/".to_string() + &id.to_string() + "/display";
                            let navigate = leptos_router::use_navigate();
                            navigate(&path, Default::default());
                        }
                    }
                },
                Err(e) => {
                    error!("{}", e);
                },
            }
        };

        view! {

            <div
                class="recipe-menu"
                class:is-open=menu_open
                class:not-logged-in=move || { !is_logged_in.get() }
                style=move || { color.get().as_bg_main_color() }
            >

                <Show
                    when=move || { !save_pending.get() }
                >
                    <button
                        class="recipe-menu-button back"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            let navigate = leptos_router::use_navigate();
                            navigate("/", Default::default());
                        }
                    >
                        <BackButtonSVG/>
                    </button>

                    <button
                        class="recipe-menu-button save"
                        on:click=on_save_click
                    >
                        "save"
                    </button>
                </Show>

                { move || {
                    view! {
                        <div class="recipe-name-input-container" >
                            <input
                                class="text-input recipe-name"
                                class:menu-open=menu_open
                                type="text"
                                id="text-input"
                                placeholder="Recipe Name..."
                                maxlength="45"
                                // get_untracked() because this is only initial value
                                value=name_signal.get_untracked()
                                // update name_signal on input
                                on:input=move |ev| {
                                    name_signal.set(event_target_value(&ev))
                                }
                            />
                        </div>
                    }.into_view()
                }}

            </div>
            
        }.into_view()
    }
}




#[component]
pub fn EditableEntryList<T: RecipeEntry>( 
    entry_type: RecipeEntryType,
    rw_entries: RwSignal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>,
    theme_color: RwSignal<ThemeColor>,
) -> impl IntoView {

    let (entry_type_title, style_class) = entry_type.title_and_class();

    let add_entry = move |_| {
        let new_entry_signal = create_signal(T::default());
        rw_entries.update(move |entries| {
            
            // Make sure to set new ID = pushed index
            let new_id: u16 = entries.len().try_into().expect("to convert usize into u16.");

            entries.push((new_id, new_entry_signal));
        });
    };


    view! {

        <div class={style_class.clone() + " container editable list"}>

            <h3
                id="field-title"
                class=style_class.clone()
                style=move || theme_color.get().as_visible_color()
            >
                { entry_type_title }
            </h3>

            <ul class=style_class.clone() >

                {move || {
                    rw_entries
                        .get()
                        .into_iter()
                        .map(|(id, (entry, set_entry))| {
                            view! {
                                <li class={style_class.clone()} id="entry-li">

                                    <div
                                        class="sorting-container ".to_string() + &T::get_css_class_name()
                                    >

                                        <Show
                                            when=move || { id != rw_entries.get()[0].0 } // don't show if this is the first entry
                                        >
                                            <button
                                                class="sort-up sorting-button"
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    rw_entries.update(|entries| {
                                                        if let Some(index) = entries.iter().position(|&x| x.0 == id) {
                                                            if index > 0 {
                                                                entries.swap(index, index - 1);
                                                            }
                                                        }
                                                    });
                                                }
                                            >
                                                <SortUpSVG/>
                                            </button>
                                        </Show>
                                        <Show
                                            when=move || {
                                                let entries = rw_entries.get();
                                                let last_index = entries.len() - 1;
                                                id != entries[last_index].0
                                            } // don't show if this is the last entry
                                        >
                                            <button
                                                class="sort-down sorting-button"
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    rw_entries.update(|entries| {
                                                        if let Some(index) = entries.iter().position(|&x| x.0 == id) {
                                                            if index < entries.len() - 1 {
                                                                entries.swap(index, index + 1);
                                                            }
                                                        }
                                                    });
                                                }
                                            >
                                                <SortDownSVG/>
                                            </button>
                                        </Show>
                                    </div>

                                    {move || {
                                        T::into_editable_view(entry, set_entry)
                                    }}

                                    <button class="remove-button ".to_string() + &T::get_css_class_name()
                                        on:click=move |ev| {
                                            ev.stop_propagation();
                                            // we are going to assign new ids since we remove an entry
                                            let mut new_id_counter: u16 = 0;

                                            // iterate in entries
                                            rw_entries.update(|entries| {
                                                entries.retain_mut(|(entry_id, (signal, _))| {

                                                    // check if this is the entry to remove
                                                    let keep_this_entry = entry_id != &id;

                                                    if keep_this_entry {
                                                        // set the new id
                                                        *entry_id = new_id_counter;
                                                        // increment counter
                                                        new_id_counter += 1;
                                                    } else {
                                                        // NOTE: in this example, we are creating the signals
                                                        // in the scope of the parent. This means the memory used to
                                                        // store them will not be reclaimed until the parent component
                                                        // is unmounted. Here, we're removing the signal early (i.e, before
                                                        // the DynamicList is unmounted), so we manually dispose of the signal
                                                        // to avoid leaking memory.
                                                        //
                                                        // This is only necessary with nested signals like this one.
                                                        signal.dispose();
                                                    }

                                                    keep_this_entry
                                                })
                                            });
                                        }
                                    >
                                    </button>

                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>

            <button class="add-button"
                on:click=add_entry
            >
                "+"
            </button>
        </div>
    }
    .into_view()
}


#[component]
pub fn EditableInstructions<T: RecipeEntry>( 
    entry_type: RecipeEntryType,
    entry_signal: (ReadSignal<T>, WriteSignal<T>),
    theme_color: RwSignal<ThemeColor>
) -> impl IntoView {

    let (entry_type_title, style_class) = entry_type.title_and_class();

    view! {
        <div class={style_class.clone() + " container editable"}>

            <h3
                id="field-title"
                class=style_class.clone()
                style=move || theme_color.get().as_visible_color()
            >
                {entry_type_title}
            </h3>

            <li class={style_class.clone()} id="entry-li">
                { T::into_editable_view(entry_signal.0, entry_signal.1) }
            </li>

        </div>
    }
    .into_view()
}




#[component]
pub fn DeleteButton(
    recipe_id: ReadSignal<u16>,
) -> impl IntoView {

    let delete_info_signal =
        use_context::<DeleteInfoSignal>()
            .expect("To find DeleteInfoReadSignal in context.")
            .0;

    let on_want_delete_click = move |_| {
        delete_info_signal.set(Some(DeletePopupInfo(recipe_id.get())));
    };

    view!{
        <span
            class= "sub-menu-option"
            on:click=on_want_delete_click
        > {"Delete"} </span>
    }
}

#[component]
pub fn DuplicateButton(
    recipe_id: ReadSignal<u16>,

) -> impl IntoView {

    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    let on_duplicate_click = move |_| {
        recipe_action.dispatch(RecipeActionDescriptor::Duplicate(recipe_id.get()));
    };

    view!{
        <span
            class= "sub-menu-option"
            on:click=on_duplicate_click
        >{"Duplicate"}</span>
    }
}

#[component]
pub fn PrintButton(
    recipe_id: ReadSignal<u16>,
) -> impl IntoView {

    let on_duplicate_click = move |_| {
        let print_path =
            "/recipe/".to_owned() + &recipe_id.get().to_string() + "/print";
        let window = web_sys::window().expect("window should be available");
        window
            .open_with_url_and_target(&print_path, "_blank")
            .unwrap_or_else(|_| {
                error!("No Window found.");
                None
            });
    };

    view!{
        <span
            class= "sub-menu-option"
            on:click=on_duplicate_click
        >{"Print"}</span>
    }
}



#[component]
pub fn RecipeEntryInput<T: RecipeEntry>(
    placeholder: String,
    get_entry_signal: ReadSignal<T>,
    set_entry_signal: WriteSignal<T>,
    class: String,
    /// If the entry has multiple fields
    #[prop(optional)]
    field_id: Option<usize>,
    #[prop(optional)]
    is_input: Option<bool>,
) -> impl IntoView {

    let is_input = is_input.unwrap_or_default();

    // setup for the SuggestionList
    let is_input_focused = create_rw_signal(false);
    let (get_input, set_input) = create_signal("".to_string());
    let all_possible_values =
        match T::get_entry_type() {
            RecipeEntryType::Tag => {
                Some(
                    use_context::<AllTagsSignal>()
                        .expect("to find AllTagsMemo in context.")
                        .0
                )
            },
            RecipeEntryType::Ingredients => {
                Some(
                    use_context::<AllIngredientsSignal>()
                        .expect("to find AllIngredientsSignal in context.")
                        .0
                )
            },
            _ => None,
        };
    
    let (get_suggestion, set_suggestion) = create_signal("".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    create_effect( move |_| {
        let new_suggestion = get_suggestion.get();
        if new_suggestion.len() > 0 {
            input_element()
                .expect("<input> should be mounted")
                .set_value(&new_suggestion);
            set_input(new_suggestion);
        }
    });

    if is_input {

        view! {
            <div
                id=         "text-input"
                class=      class.clone() + " wrapper"
                on:focusin= move |_| {
                    set_input.set(
                        get_entry_signal.get().get_string_from_field(field_id)
                    );
                    is_input_focused.set(true);
                }
                on:focusout=move |_| {
                    let timeout = Timeout::new(250, move || {
                        is_input_focused.set(false);
                    });
                    timeout.forget();
                }
            >

                <input
                    class=          class
                    type=           "text"
                    id=             "text-input"
                    placeholder=    placeholder
                    value=          move || { get_entry_signal.get_untracked().get_string_from_field(field_id) }
                    //maxlength=      "20"
                    node_ref=       input_element
                    style=          move || {
                        if T::get_entry_type() == RecipeEntryType::Tag {
                            let input_length = usize::min(get_input.get().len(), 20);
                            "width: ".to_string()
                            + &input_length.to_string()
                            + "ch"
                        } else { "".to_string() }
                    }
                    on:input=       move |ev| {
                        // on input, update entry signal
                        let current_input = event_target_value(&ev);
                        set_input.set(current_input.clone());
                        set_entry_signal.update(|recipe_entry| {
                            recipe_entry.update_field_from_string_input(field_id, current_input);
                        });
                    }
                />

                {move || {

                    set_input.set(
                        get_entry_signal.get().get_string_from_field(field_id)
                    );

                    view! {
                        <SuggestionList
                            is_input_focused=       is_input_focused
                            text_input=             get_input
                            possible_values=        all_possible_values
                            suggestion_setter=      set_suggestion
                        />
                    }
                    .into_view()
                }}
            </div>
        }
        .into_view()
    } else {

        // Textarea
        #[allow(unused)]
        let textarea = create_node_ref::<html::Textarea>();

        // setup for textarea autosize
        #[cfg(feature= "hydrate")]
        let leptos_use::UseTextareaAutosizeReturn {
            content: _,
            set_content,
            trigger_resize: _
        } = leptos_use::use_textarea_autosize(textarea);

        view! {
            <textarea
                class=          class
                node_ref=       textarea
                type=           "text"
                id=             "text-input"
                placeholder=    placeholder
                prop:value=          move || { get_entry_signal.get_untracked().get_string_from_field(field_id) }
    
                // on input
                on:input=move |ev| {

                    // resize box to fit text
                    #[cfg(feature= "hydrate")]
                    set_content.set(event_target_value(&ev));

                    // update entry signal
                    set_entry_signal.update(|recipe_entry| {
                        recipe_entry.update_field_from_string_input(field_id, event_target_value(&ev))
                    });
                }
            >
                //{move || { get_entry_signal.get().get_string_from_field(field_id) }}
            </textarea>
        }
        .into_view()
    }
    
}


#[component]
pub fn SettingsMenu() -> impl IntoView {

    // get settings menu context
    let is_settings_menu_open =
        use_context::<IsSettingsMenuOpen>()
            .expect("Expected to find IsSettingsMenuOpen in context.")
            .0;

    // Is logged in
    let is_logged_in =
        use_context::<IsLoggedIn>()
            .expect("Expected to find IsLoggedIn in context.")
            .0;
    
    // Logout action
    let logout_action = create_action(move |_: &()| {
        async move {
            match server_logout().await {
                Ok(_) => {
                    is_logged_in.set(false);
                },
                Err(e) => error!("Error: {:?}", e.to_string()),
            }
        }
    });


    view! {
        <button
            class = "settings-menu-button"
            class:menu-open=is_settings_menu_open
            on:click=move |_| is_settings_menu_open.update(|b| *b = !*b)
        >
        </button>

        <div
            class = "background-blocker settings-blocker"
            class:is-enabled=is_settings_menu_open
            on:click=move |ev| {
                ev.stop_propagation();
                is_settings_menu_open.set(false);
            }
        ></div>

        <div
            class = "settings-menu"
            class:is-open=is_settings_menu_open
            on:click=move |ev| {
                ev.stop_propagation();
                is_settings_menu_open.set(false);
            }
        >

            <Show
                when=is_logged_in
                fallback=move || view! {
                    <div
                        class="login-container"
                        on:click=move |ev| {
                            ev.stop_propagation();
                        }
                    >
                        <LoginMenu/>
                    </div>
                } // Login
            >

                // Backup
                <Show
                    when=move || {
                        let path = use_location().pathname.get();
                        let is_backup =
                            path
                                .split('/')
                                .last()
                                .is_some_and(|last_word| last_word == "backup");
                        !is_backup
                    }
                >
                    <button
                        class="settings-button backup"
                        on:click=move |_| {
                            is_settings_menu_open.set(false);

                            let navigate = leptos_router::use_navigate();
                            navigate("/backup", Default::default());
                        }
                    >
                        <BackupButtonSVG/>
                        <p class="settings-button-text backup" >
                            "Backup"
                        </p>
                    </button>
                </Show>

                // Logout
                <button
                    class="settings-button logout"
                    on:click=move |_| {
                        is_settings_menu_open.set(false);
                        logout_action.dispatch(());
                    }
                >
                    <LogoutButtonSVG/>
                    <p class="settings-button-text logout" >
                        "Logout"
                    </p>
                </button>

            </Show>

        </div>
    }
}
