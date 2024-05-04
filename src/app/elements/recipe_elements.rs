use leptos::{html::Input, *};

use crate::app::{elements::popups::DeleteRecipePopup, *};


#[component]
pub fn EditableRecipeName(
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    recipe_setter: Option<WriteSignal<Recipe>>,
    #[prop(optional)]
    editable: Option<bool>,
) -> impl IntoView {

    let editable = editable.unwrap_or_default();

    let name = create_signal( name.unwrap_or_else(|| "".to_owned() ) );

    let is_edit = create_signal(false);

    if !editable {
        view! {
            <div
                class= { "name container" }
            >
                <h1>{ move || name.0.get() }</h1>
            </div>
        }.into_view()
    } else {
        let input_element: NodeRef<html::Input> = create_node_ref();

        let on_submit = move |ev: ev::SubmitEvent| {

            ev.prevent_default();
                                    
            let value = input_element()
                .expect("<input> to exist")
                .value();

            name.1.set(value.clone());

            if let Some(recipe_setter) = recipe_setter {
                recipe_setter.update(|current_recipe| {
                    current_recipe.name = value;
                });
            }

            // Remove edit mode
            is_edit.1.set(false);
        };

        view! {
            <div class={"name container editable"}>
                <Show
                    when=move || { !is_edit.0.get() }
                    fallback=move || {view!{
                        <form on:submit=on_submit>
                            <input type="text" id="input-name"
                                value=name.0
                                node_ref=input_element
                            /><br/>
                            <input
                                type="submit"
                                name="name-validation"
                                value="☑"
                            />
                        </form>
                    }}
                >
                    <h1>{name.0.get()}</h1>
                    <button
                        on:click=move |_| {
                            is_edit.1.set(true);
                        }
                    >
                        "Edit"
                    </button>
                </Show>
            </div>
        }.into_view()
    }
}





#[component]
pub fn EntryList<T: RecipeEntry>(
    entry_list: Option<Vec<T>>,
    entry_type: RecipeEntryType,
) -> impl IntoView {

    let (entry_type_title, style_class) = entry_type.title_and_class();

    // Component empty if entry list None or empty
    let entry_list = entry_list.unwrap_or_else(|| vec![]);
    if entry_list.len() == 0 { return ().into_view(); }

    view! {
        <div class={style_class + " container list"}>
            <h1>{entry_type_title}</h1>
            <ul>
                {
                    entry_list
                        .into_iter()
                        .map(|entry| {
                            view! {
                                <li>
                                    { entry.into_view() }
                                </li>
                            }
                        })
                        .collect_view()
                }
            </ul>
        </div>
    }.into_view()
}







#[component]
pub fn EditableEntryList<T: RecipeEntry>(
    entry_list: Option<Vec<T>>,
    entry_type: RecipeEntryType,
    #[prop(optional)]
    recipe_setter: Option<WriteSignal<Recipe>>,
    #[prop(optional)]
    editable: Option<bool>,
) -> impl IntoView {

    let editable = editable.unwrap_or_default();

    let (entry_type_title, style_class) = entry_type.title_and_class();

    // Create a unique ID
    let mut unique_id = 0_u16;

    let entry_list = entry_list.unwrap_or_else(|| vec![]);

    // Create the signal of the Vec of signal contents
    type EntryListTuple<T> = Vec<(u16, (ReadSignal<bool>, WriteSignal<bool>), (ReadSignal<T>, WriteSignal<T>))>;
    let (get_entries, set_entries): (ReadSignal<EntryListTuple<T>>, WriteSignal<EntryListTuple<T>>) =
        create_signal(
            entry_list
                .iter()
                .map(|s| {
                    let new_id: u16 = unique_id;
                    unique_id += 1;
                    let is_edit_signal = create_signal(false);
                    let content_signal = create_signal(s.clone());

                    (new_id, is_edit_signal, content_signal)
                })
                .collect()
        );

    let add_entry = move |_| {
        let new_entry_signal = create_signal(T::default());
        let is_edit_signal = create_signal(true);
        set_entries.update(move |entries| {
            let new_id: u16 = unique_id;

            entries.push((new_id, is_edit_signal, new_entry_signal));
        });

        unique_id += 1;
    };

    view! {
        <div class={style_class + " container editable list"}>
            <h3>{entry_type_title}</h3>
            <ul>
                <For
                    each=move || get_entries.get()
                    key=|entry| entry.0
                    children=move |(id, is_edit, (entry, set_entry))| {
                        view! {
                            <li>
                                {
                                    if !editable {
                                        entry.into_view()
                                    } else {
                                        view! {
                                            <Show
                                                when=move || { !is_edit.0.get() }
                                                fallback=move || {
    
                                                    let node_refs = T::create_inputs_node_refs();
                                                    let node_refs_for_submit = node_refs.clone();
    
                                                    // fires when the form `submit` event happens
                                                    // this will store the value of the <input> in our signal
                                                    let on_submit = move |ev: ev::SubmitEvent| {
    
                                                        // stop the page from reloading!
                                                        ev.prevent_default();
    
                                                        // Extract the value from input and update the signal
                                                        let value = T::extract_value(node_refs_for_submit.clone());
                                                        set_entry.set(value);
                                                        
                                                        // Removes the edit mode
                                                        set_entries.update(|entries| {
                                                            // Set edit mode for this entry
                                                            entries.iter_mut().for_each(|i| {
                                                                if i.0 == id {
                                                                    i.1.1.set(false);
                                                                }
                                                            });
                                                        });
    
                                                        // Update the recipe in the parent
                                                        let new_entries: Vec<T> =
                                                            get_entries
                                                                .get()
                                                                .into_iter()
                                                                .map(|e| e.2.0.get()) // entry -> signal -> getter -> value
                                                                .collect();
    
                                                        let new_entries: Option<Vec<T>> = if new_entries.len() < 1 {
                                                            None
                                                        } else {
                                                            Some(new_entries)
                                                        };
    
                                                        if let Some(recipe_setter) = recipe_setter {
                                                            recipe_setter.update(|current_recipe| {
                                                                T::update_recipe(new_entries, current_recipe);
                                                            });
                                                        }
                                                    };
                                                    view! {
                                                        <form on:submit=on_submit>
                                                            { T::into_editable_view(entry, node_refs) }
                                                            <br/>
                                                            <input
                                                                type="submit"
                                                                id="entry-validation"
                                                                value="☑"
                                                            />
                                                        </form>
                                                    }
                                                }
                                            >
                                                <p>{entry}</p>
                                                
                                                <button
                                                    on:click=move |_| {
                                                        set_entries.update(|entries| {
                                                            // Set edit mode for this entry
                                                            entries.iter_mut().for_each(|i| {
                                                                if i.0 == id {
                                                                    i.1.1.set(true);
                                                                }
                                                            });
                                                        });
                                                    }
                                                >
                                                    "Edit"
                                                </button>
                                            </Show>
    
                                            <button
                                                on:click=move |_| {
                                                    set_entries.update(|entries| {
                                                        entries.retain(|(entry_id, _, (signal, _))| {
                                                            // NOTE: in this example, we are creating the signals
                                                            // in the scope of the parent. This means the memory used to
                                                            // store them will not be reclaimed until the parent component
                                                            // is unmounted. Here, we're removing the signal early (i.e, before
                                                            // the DynamicList is unmounted), so we manually dispose of the signal
                                                            // to avoid leaking memory.
                                                            //
                                                            // This is only necessary with nested signals like this one.
                                                            if entry_id == &id {
                                                                signal.dispose();
                                                            }
                                                            entry_id != &id
                                                        })
                                                    });
                                                }
                                            >
                                                "☒"
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
                        <button on:click=add_entry>
                            "+"
                        </button>
                    }.into_view()
                } else { ().into_view() }
            }
        </div>
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
pub fn RecipeEntryInput(
    elem_ref: NodeRef<Input>,
    input: String,
    placeholder: String,
) -> impl IntoView {

    let input = if input.is_empty() { None } else { Some(input) };

    view! {
        <input 
            type={"text"}
            id={"recipe-entry"}
            value = {input}
            placeholder={placeholder}
            node_ref={elem_ref}
        />
    }
}
