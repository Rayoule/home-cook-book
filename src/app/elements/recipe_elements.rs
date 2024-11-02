use leptos::{
    *, logging::*,
};
use components::auth::auth_server_functions::server_logout;
use ev::MouseEvent;
use gloo_timers::callback::Timeout;
use crate::app::{
    *,
    elements::molecules::*
};


#[component]
pub fn RecipeMenu(
    color: RwSignal<ThemeColor>,
    editable: bool,
    #[prop(optional)]
    recipe_name: Option<String>,
    #[prop(optional)]
    name_signal: Option<RwSignal<String>>,
    #[prop(optional)]
    recipe_id: Option<u16>,
) -> impl IntoView {

    // Is logged in
    let is_logged_in =
        use_context::<IsLoggedIn>()
            .expect("Expected to find IsLoggedIn in context.")
            .0;

    let menu_open = create_rw_signal(false);

    let recipe_name = recipe_name.unwrap_or_else(|| String::new());

    if !editable {

        view! {
    
            <div
                class="recipe-menu"
                class:is-open=menu_open
                class:not-logged-in=move || { !is_logged_in.get() }
                style=move || { color.get().as_bg_main_color() }
            >

                <button
                    class="recipe-menu-button back"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        let navigate = leptos_router::use_navigate();
                        navigate("/", Default::default());
                    }
                >
                </button>

                <button
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
                        class="recipe-name"
                        class:menu-open=menu_open
                    >
                        { recipe_name.clone() }
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
                                let recipe_id = recipe_id.expect("to find recipe_id for button Edit.");
                                let edit_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/editable";
                                let navigate = leptos_router::use_navigate();
                                navigate(&edit_path, Default::default());
                            }
                        >
                            "Edit"
                        </button>
                    </Show>
    
                    <button
                        style=move || { color.get().as_alt_color() }
                        class="recipe-menu-option"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            let recipe_id = recipe_id.expect("to find recipe_id for button Edit.");
                            let print_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/print";
                            let window = web_sys::window().expect("window should be available");
                            window
                                .open_with_url_and_target(&print_path, "_blank")
                                .unwrap();
                        }
                    >
                        "Print"
                    </button>
    
                    <Show
                        when=is_logged_in
                    >
                        <button
                            style=move || { color.get().as_alt_color() }
                            class="recipe-menu-option"
                            on:click=move |ev: MouseEvent| {
                                ev.stop_propagation();
                                let recipe_id = recipe_id.expect("to find recipe_id for button Edit.");
                                let delete_info_signal =
                                    use_context::<DeleteInfoSignal>()
                                        .expect("To find DeleteInfoSignal in context.")
                                        .0;
                                delete_info_signal.set( Some( DeletePopupInfo(recipe_id)) );
                            }
                        >
                            "Delete"
                        </button>
                    </Show>
    
                </Show>
    
            </div>
        }.into_view()

    } else {

        view! {

            <div
                class="recipe-menu"
                class:is-open=menu_open
            >

                <button
                    style=move || { color.get().as_alt_color() }
                    class="recipe-menu-button back"
                >
                    "Back"
                </button>

                <button
                    style=move || { color.get().as_alt_color() }
                    class="recipe-menu-button save"
                >
                    "Save"
                </button>

                { move || {
                    if let Some(name_signal) = name_signal {
                        view! {
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
                        }.into_view()
                    } else {
                        error!("ERROR: No Name Signal provided.");
                        ().into_view()
                    }
                }}

            </div>
            
        }.into_view()
    }
}


#[component]
pub fn EditableRecipeName(
    /// Provide this if editable
    #[prop(optional)]
    name_signal: Option<RwSignal<String>>,
) -> impl IntoView {

    if let Some(name_signal) = name_signal {
        view! {
                <input
                    class="text-input name"
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
        }.into_view()
    } else {
        error!("ERROR: No Name Signal provided.");
        ().into_view()
    }
}




#[component]
pub fn EditableEntryList<T: RecipeEntry>( 
    editable: bool,
    entry_type: RecipeEntryType,
    /// Needed if not editable
    #[prop(optional)]
    entry_list: Option<Vec<T>>,
    /// Needed if editable
    #[prop(optional)]
    entry_list_signal: Option<RwSignal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>>,
) -> impl IntoView {

    let (entry_type_title, style_class) = entry_type.title_and_class();

    if !editable {

        let is_empty =
            if let Some(entry_list) = &entry_list {
                entry_list.len() > 0
            } else { false };

        if is_empty {
            return ().into_view();
        } else {

            // Component in non editable mode
            view! {
                <div class={style_class.clone() + " container list"} >
                    <h1>{entry_type_title}</h1>
                    <ul class={style_class.clone()}>
                        {
                            entry_list
                                .into_iter()
                                .map(|entry| {
                                    view! {
                                        <li class={style_class.clone()} id="entry-li">
                                            { entry.into_view() }
                                        </li>
                                    }
                                })
                                .collect_view()
                        }
                    </ul>
                </div>
            }
            .into_view()
        }
    } else {

        let rw_entries: RwSignal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>> =
            entry_list_signal.unwrap_or_else(|| {
                create_rw_signal(vec![])
            });

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

                <h3 id="field-title" class=style_class.clone() >{entry_type_title}</h3>

                <ul class={style_class.clone()}>
                    <For
                        each=move || rw_entries.get()
                        key=|entry| entry.0
                        children=move |(id, (entry, set_entry))| {

                            view! {
                                <li class={style_class.clone()} id="entry-li">
                                    {
                                        if !editable {
                                            entry.into_view()
                                        } else {

                                            view! {

                                                {
                                                    T::into_editable_view(entry, set_entry)
                                                }
        
                                                <button class="remove-button"
                                                    on:click=move |_| {
                                                        // we are going to assign new ids since we remove an entry
                                                        let mut new_id_counter: u16 = 0;

                                                        // iterate in entries
                                                        rw_entries.update(|entries| {
                                                            entries.retain_mut(|(entry_id, (signal, _))| {

                                                                // check if this is the entry to remove
                                                                let keep_this_entry = entry_id != &id;
                                                                //let keep_this_entry = true;

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
                                                    "x"
                                                </button>
                                            }.into_view()
                                        }
                                    }
                                </li>
                            }
                        }
                    />
                </ul>
                {
                    if editable {
                        view! {
                            <button class="add-button" id="fake-button"
                                on:click=add_entry
                            >
                                "+"
                            </button>
                        }.into_view()
                    } else { ().into_view() }
                }
            </div>
        }
        .into_view()
    }
}


#[component]
pub fn EditableInstructions<T: RecipeEntry>( 
    editable: bool,
    entry_type: RecipeEntryType,
    /// Needed if not editable
    #[prop(optional)]
    entry_list: Option<T>,
    /// Needed if editable
    #[prop(optional)]
    entry_signal: Option<(ReadSignal<T>, WriteSignal<T>)>,
) -> impl IntoView {

    let (entry_type_title, style_class) = entry_type.title_and_class();

    if !editable {

        // Component in non editable mode
        view! {
            <div class={style_class.clone() + " container list"} >
                <h1>{entry_type_title}</h1>
                <ul class={style_class.clone()}>
                    {
                        entry_list
                            .into_iter()
                            .map(|entry| {
                                view! {
                                    <li class={style_class.clone()} id="entry-li">
                                        { entry.into_view() }
                                    </li>
                                }
                            })
                            .collect_view()
                    }
                </ul>
            </div>
        }
        .into_view()
    } else {

        let (entry, set_entry) = entry_signal.unwrap_or(create_signal(T::default()));

        view! {
            <div class={style_class.clone() + " container editable"}>

                <h3 id="field-title" class=style_class.clone() >{entry_type_title}</h3>

                <li class={style_class.clone()} id="entry-li">
                    {
                        if !editable {
                            entry.into_view()
                        } else {
                            view! {
                                { T::into_editable_view(entry, set_entry) }
                            }
                        }
                    }
                </li>

            </div>
        }
        .into_view()
    }
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
            .unwrap();
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
    initial_value: String,
    placeholder: String,
    get_entry_signal: ReadSignal<T>,
    set_entry_signal: WriteSignal<T>,
    class: String,
    /// If the entry has multiple fields
    #[prop(optional)]
    field_id: Option<usize>,
    #[prop(optional)]
    is_input: Option<bool>,
    #[prop(optional)]
    is_only_numbers: Option<bool>,
) -> impl IntoView {

    let is_input = is_input.unwrap_or_default();
    let is_only_numbers = is_only_numbers.unwrap_or_default();

    let initial_value = if initial_value.is_empty() { None } else { Some(initial_value) };

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
        }
    });

    if is_input {
        // Input + maxlength
        view! {
            <div
                on:focusin=move |_| {
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
                    type=           { if is_only_numbers {"number"} else {"text"} }
                    id=             "text-input"
                    value=          initial_value
                    placeholder=    placeholder
                    maxlength=      "20"
                    node_ref=       input_element
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
                {initial_value}
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
        ></button>

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
                <button
                    class="settings-button backup"
                    on:click=move |_| {
                        is_settings_menu_open.set(false);

                        let navigate = leptos_router::use_navigate();
                        navigate("/backup", Default::default());
                    }
                >
                    "Backup"
                </button>

                // Logout
                <button
                    class="settings-button logout"
                    on:click=move |_| {
                        is_settings_menu_open.set(false);
                        logout_action.dispatch(());
                    }
                > "Logout" </button>

            </Show>

        </div>
    }
}
