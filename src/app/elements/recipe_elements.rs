use leptos::{
    *,
    html::Input,
    logging::log,
};

use crate::app::{elements::popups::DeleteRecipePopup, *};


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
        
                                                <button class="remove-button" id="fake-button"
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
    recipe_getter: ReadSignal<Recipe>,
    recipe_action: Action<(ReadSignal<Recipe>, RecipeAction), Result<(), ServerFnError>>,
) -> impl IntoView {

    let (wants_deletion_getter, wants_deletion_setter) = create_signal(false);

    let on_want_delete_click = move |_| {
        wants_deletion_setter.set(true);
    };

    view!{

        <Show
            when=move || { wants_deletion_getter.get() }
            fallback=move || {view!{
                <button on:click=on_want_delete_click > {"DELETE"} </button>
            }}
        >
            <DeleteRecipePopup
                recipe_getter=          recipe_getter
                wants_deletion_setter=  wants_deletion_setter
                recipe_action=          recipe_action
            />
        </Show>
    }
}



#[component]
pub fn RecipeEntryInput<T: RecipeEntry>(
    initial_value: String,
    placeholder: String,
    set_entry_signal: WriteSignal<T>,
    class: String,
    /// If the entry has multiple fields
    #[prop(optional)]
    field_id: Option<usize>,
    #[prop(optional)]
    is_input: Option<bool>,
) -> impl IntoView {

    let is_input = is_input.unwrap_or_default();

    let initial_value = if initial_value.is_empty() { None } else { Some(initial_value) };

    if is_input {
        // Input + maxlength
        view! {
            <input
                class=          class
                type=           "text"
                id=             "text-input"
                value=          initial_value
                placeholder=    placeholder
                maxlength=      "20"
    
                // on input, update entry signal
                on:input=move |ev| {
                    set_entry_signal.update(|recipe_entry| {
                        recipe_entry.update_field_from_string_input(field_id, event_target_value(&ev));
                    });
                }
            />
        }
        .into_view()
    } else {
        // Textarea
        view! {
            <textarea
                class=          class
                type=           "text"
                id=             "text-input"
                value=          initial_value
                placeholder=    placeholder
    
                // on input, update entry signal
                on:input=move |ev| {
                    set_entry_signal.update(|recipe_entry| {
                        recipe_entry.update_field_from_string_input(field_id, event_target_value(&ev))
                    });
                }
            />
        }
        .into_view()
    }
    
}
