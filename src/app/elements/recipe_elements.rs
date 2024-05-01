use leptos::*;
use serde::{Serialize, Deserialize};

use crate::app::*;


#[component]
pub fn RecipeName(
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    recipe_setter: Option<WriteSignal<Recipe>>,
    #[prop(optional)]
    editable: Option<(ReadSignal<bool>, WriteSignal<bool>)>,
) -> impl IntoView {

    let editable = editable.unwrap_or_else(|| create_signal(false));

    let name = create_signal( name.unwrap_or_else(|| "".to_owned() ) );

    let is_edit = create_signal(false);

    view! { class = style_class,
        <Show
            // Is the entry in edit mode ?
            when=move || { editable.0.get() }
            // If not in edit mode:
            fallback= move || {
                view! {
                    <h1 class="recipe-title">{name.0.get()}</h1>
                }
            }
        >
            {
                // we'll use a NodeRef to store a reference to the input element
                // this will be filled when the element is created
                let input_element: NodeRef<html::Input> = create_node_ref();

                // fires when the form `submit` event happens
                // this will store the value of the <input> in our signal
                let on_submit = move |ev: ev::SubmitEvent| {
                    // stop the page from reloading!
                    ev.prevent_default();
                                            
                    // here, we'll extract the value from the input
                    let value = input_element()
                        // event handlers can only fire after the view
                        // is mounted to the DOM, so the `NodeRef` will be `Some`
                        .expect("<input> to exist")
                        // `NodeRef` implements `Deref` for the DOM element type
                        // this means we can call`HtmlInputElement::value()`
                        // to get the current value of the input
                        .value();
                    // Set the value in the name signal
                    name.1.set(value.clone());
                    // Update the recipe in the parent signal
                    if let Some(recipe_setter) = recipe_setter {
                        recipe_setter.update(|current_recipe| {
                            current_recipe.name = value;
                        });
                    }

                    // Remove edit mode
                    is_edit.1.set(false);
                };

                view! {

                    <Show
                        when=move || { !is_edit.0.get() }
                        fallback=move || {view!{
                            <form on:submit=on_submit>
                                <input type="text"
                                    // here, we use the `value` *attribute* to set only
                                    // the initial value, letting the browser maintain
                                    // the state after that
                                    value=name.0
                    
                                    // store a reference to this input in `input_element`
                                    node_ref=input_element
                                /><br/>
                                <input
                                    type="submit"
                                    value="☑"
                                />
                            </form>
                        }}
                    >
                        <h1 class="recipe-title">{name.0.get()}</h1>
                        <button
                            on:click=move |_| {
                                is_edit.1.set(true);
                            }
                        >
                            "Edit"
                        </button>
                    </Show>
                }
            }
        </Show>
    }
}



#[component]
pub fn StringEntryList(
    entry_list: Option<Vec<String>>,
    entry_type: RecipeContentType,
    #[prop(optional)]
    recipe_setter: Option<WriteSignal<Recipe>>,
    #[prop(optional)]
    editable: Option<(ReadSignal<bool>, WriteSignal<bool>)>,
) -> impl IntoView {

    let editable = editable.unwrap_or_else(|| {
        create_signal(false)
    });

    let ingredients_or_instructions = match entry_type {
        RecipeContentType::Tags => "Tags".to_owned(),
        RecipeContentType::Ingredients => "Ingredients".to_owned(),
        RecipeContentType::Instructions => "Instructions".to_owned(),
        RecipeContentType::Notes => "Notes".to_owned(),
    };

    let style_class = match entry_type {
        RecipeContentType::Tags => "recipe-tags".to_owned(),
        RecipeContentType::Ingredients => "recipe-ingredients".to_owned(),
        RecipeContentType::Instructions => "recipe-instructions".to_owned(),
        RecipeContentType::Notes => "recipe-notes".to_owned(),
    };

    // Needed
    let entry_type = create_signal(entry_type);


    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // Create a unique ID
    let mut unique_id = 0_u16;

    let entry_list = entry_list.unwrap_or_else(|| vec![]);

    // Create the signal of the Vec of signal contents
    type EntryListTuple = Vec<(u16, (ReadSignal<bool>, WriteSignal<bool>), (ReadSignal<String>, WriteSignal<String>))>;
    let (get_entries, set_entries): (ReadSignal<EntryListTuple>, WriteSignal<EntryListTuple>) =
        create_signal(
            entry_list
                .iter()
                .map(|s| {
                    let new_id: u16 = unique_id;
                    unique_id += 1;
                    let is_edit_signal = create_signal(editable.0.get_untracked());
                    let content_signal = create_signal(s.clone());

                    (new_id, is_edit_signal, content_signal)
                })
                .collect()
        );

    let add_entry = move |_| {
        // create a signal for the new ingredient
        let new_entry_signal = create_signal("".to_owned());
        let is_edit_signal = create_signal(true);
        // add this counter to the list of counters
        set_entries.update(move |entries| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            let new_id: u16 = unique_id;

            entries.push((new_id, is_edit_signal, new_entry_signal));
        });

        unique_id += 1;
    };

    view! {
        <div class={style_class}>
            <h3>{ingredients_or_instructions}</h3>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=move || get_entries.get()
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|entry| entry.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, is_edit, (entry, set_entry))| {
                        view! {
                            <li>
                                <Show
                                    // Is the entry in edit mode ?
                                    when=move || { editable.0.get() }

                                    // If not in edit mode:
                                    fallback= move ||
                                    {
                                        view!{ 
                                            <p>{entry}</p>
                                        }
                                    }
                                > // If in edit mode:
                                    {
                                        // we'll use a NodeRef to store a reference to the input element
                                        // this will be filled when the element is created
                                        let input_element: NodeRef<html::Input> = create_node_ref();

                                        // fires when the form `submit` event happens
                                        // this will store the value of the <input> in our signal
                                        let on_submit = move |ev: ev::SubmitEvent| {
                                            // stop the page from reloading!
                                            ev.prevent_default();
                                            
                                            // here, we'll extract the value from the input
                                            let value = input_element()
                                                // event handlers can only fire after the view
                                                // is mounted to the DOM, so the `NodeRef` will be `Some`
                                                .expect("<input> to exist")
                                                // `NodeRef` implements `Deref` for the DOM element type
                                                // this means we can call`HtmlInputElement::value()`
                                                // to get the current value of the input
                                                .value();
                                            set_entry.set(value);
                                            

                                            // Then remove the edit mode
                                            set_entries.update(|entries| {
                                                // Set edit mode for this entry
                                                entries.iter_mut().for_each(|i| {
                                                    if i.0 == id {
                                                        i.1.1.set(false);
                                                    }
                                                });
                                            });

                                            // Update the recipe in the parent
                                            let new_entries: Vec<String> =
                                                get_entries
                                                    .get()
                                                    .into_iter()
                                                    .map(|e| e.2.0.get()) // We get the entry, then the string content signal, then its getter
                                                    .collect();

                                            let new_entries: Option<Vec<String>> = if new_entries.len() < 1 {
                                                None
                                            } else {
                                                Some(new_entries)
                                            };

                                            if let Some(recipe_setter) = recipe_setter {
                                                recipe_setter.update(|current_recipe| {
                                                    match entry_type.0.get() {
                                                        RecipeContentType::Tags => current_recipe.tags = new_entries,
                                                        RecipeContentType::Ingredients => current_recipe.ingredients = new_entries,
                                                        RecipeContentType::Instructions => current_recipe.instructions = new_entries,
                                                        RecipeContentType::Notes => current_recipe.notes = new_entries,
                                                    }
                                                });
                                            }
                                        };
                                        
                                        view! {
                                            <Show
                                                when=move || { !is_edit.0.get() }

                                                fallback=move || { view! {
                                                    <form on:submit=on_submit>
                                                        <input type="text"
                                                            // here, we use the `value` *attribute* to set only
                                                            // the initial value, letting the browser maintain
                                                            // the state after that
                                                            value=entry
                                            
                                                            // store a reference to this input in `input_element`
                                                            node_ref=input_element
                                                        /><br/>
                                                        <input
                                                            type="submit"
                                                            value="☑"
                                                        />
                                                    </form>
                                                }}
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
                                        }
                                    }
                                </Show>
                            </li>
                        }
                    }
                />
            </ul>
            <Show
                when=move || editable.0.get()
            >
                <button on:click=add_entry>
                    "+"
                </button>
            </Show>
        </div>
    }
}
