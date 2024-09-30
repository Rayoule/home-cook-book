use leptos::{
    ev::FocusEvent, html::Input, logging::log, *
};

use gloo_timers::callback::Timeout;

use crate::app::{
    *,
    elements::{
        popups::*, molecules::*
    }
};


#[component]
pub fn EditableRecipeName(
    editable: bool,
    /// Provide this if not editable
    #[prop(optional)]
    name: Option<String>,
    /// Provide this if editable
    #[prop(optional)]
    name_signal: Option<RwSignal<String>>,
) -> impl IntoView {

    if !editable {
        if let Some(name) = name {
            view! {
                <div
                    class= { "name" }
                >
                    <h1>{name}</h1>
                </div>
            }.into_view()
        } else {
            log!("ERROR: No Name provided.");
            ().into_view()
        }
        
    } else {

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
            log!("ERROR: No Name Signal provided.");
            ().into_view()
        }
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
                <div class={style_class.clone() + " container list"}>
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

                            log!("RENDERING LIST");

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

    log!("Et donc du coup ----> {:?}", initial_value);

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
                    log!("BAM");
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

        // setup for textarea autosize
        let textarea = create_node_ref::<html::Textarea>();
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
pub fn RecipeLightSubMenu(
    recipe_id: ReadSignal<u16>,
) -> impl IntoView {

    let is_menu = create_rw_signal(false);

    let edit_path: String = "/recipe/".to_owned() + &recipe_id.get_untracked().to_string() + "/editable";
    let print_path: String = "/recipe/".to_owned() + &recipe_id.get_untracked().to_string() + "/print";

    let on_sub_menu_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        is_menu.set(!is_menu.get());
    };
    
    view! {
        <div
            class="recipe-light-sub-menu"
            class:into-menu=is_menu
            on:click=on_sub_menu_click
        >
            <div
                class="sub-menu-dot"
                class:into-menu=is_menu
            >{"•"}</div>
            <div
                class="sub-menu-dot"
                class:into-menu=is_menu
            >{"•"}</div>
            <div
                class="sub-menu-dot"
                class:into-menu=is_menu
            >{"•"}</div>

            <div
                class= "sub-menu-buttons"
                class:into-menu=is_menu
            >
                <A
                    class= "sub-menu-option"
                    href=edit_path
                >{"Edit"}</A>

                <DuplicateButton
                    recipe_id=      recipe_id
                />

                <A
                    class= "sub-menu-option"
                    href=print_path
                >{"Print"}</A>

                <DeleteButton
                    recipe_id=recipe_id
                />
            </div>

            <div
                class="close-sub-menu"
                class:into-menu=is_menu
                on:click=on_sub_menu_click
            >{"X"}</div>
        </div>
    }
}
