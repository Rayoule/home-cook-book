use leptos::*;
use serde::{Serialize, Deserialize};

type SignalList = (ReadSignal<Vec<(ReadSignal<String>, WriteSignal<String>)>>, WriteSignal<Vec<(ReadSignal<String>, WriteSignal<String>)>>);

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipe {
    pub id: u16,
    pub recipe: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecipeSignal {
    pub name: (ReadSignal<String>, WriteSignal<String>),
    pub ingredients: SignalList,
    pub instructions: SignalList,
}

impl Default for RecipeSignal {
    fn default() -> Self {
        Self {
            name: create_signal("".to_owned()),
            ingredients: create_signal(vec![]),
            instructions: create_signal(vec![])
        }
    }
}

#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)]
    recipe: Option<Recipe>,
    #[prop(optional)]
    editable: Option<(ReadSignal<bool>, WriteSignal<bool>)>,
) -> impl IntoView {

    let editable = editable.unwrap_or_else(|| create_signal(false));

    let recipe = recipe.unwrap_or_default();

    view! {
        <div>

            <h1>Name</h1>
            <RecipeName
                name=recipe.name
                editable=editable
            />

            <h2>Ingredients</h2>
            <StringEntryList
                entry_list=recipe.ingredients
                editable=editable
            />

            <h2>Instructions</h2>
            <StringEntryList
                entry_list=recipe.instructions
                editable=editable
            />

            <Show
                when=move || !editable.0.get()
                fallback=move || {
                    view! {
                        <button
                            on:click=move |_| {
                                // TODO Save the edited recipe back to the DB !!
                                editable.1.set(false);
                            }
                        >
                            "Save"
                        </button>
                    }
                }
            >
                <button on:click=move |_| { editable.1.set(true) } >
                    "Edit Recipe"
                </button>
            </Show>

        </div>
    }
}


#[component]
pub fn RecipeName(
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    editable: Option<(ReadSignal<bool>, WriteSignal<bool>)>,
) -> impl IntoView {

    let editable = editable.unwrap_or_else(|| create_signal(true));

    let name = create_signal( name.unwrap_or_else(|| "".to_owned() ) );

    let is_edit = create_signal(false);

    view! {
        <Show
            // Is the entry in edit mode ?
            when=move || { is_edit.0.get() }
            // If not in edit mode:
            fallback= move || {
                view! {
                    <h1>{name.0}</h1>
                    <Show
                        when=move || { editable.0.get() }
                    >
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
                        name.1.set(value);

                        // Remove edit mode
                        is_edit.1.set(false);
                };

                view! {
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
                }
            }
        </Show>
        <h1>{name.0.get()}</h1>
    }
}




#[component]
pub fn StringEntryList(
    #[prop(optional)]
    entry_list: Option<Vec<String>>,
    #[prop(optional)]
    editable: Option<(ReadSignal<bool>, WriteSignal<bool>)>,
) -> impl IntoView {

    let editable = editable.unwrap_or_else(|| create_signal(true));


    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // Create a unique ID
    let mut unique_id = 0_u16;

    // Create the signal of the Vec of signal contents
    type EntryListTuple = Vec<(u16, (ReadSignal<bool>, WriteSignal<bool>), (ReadSignal<String>, WriteSignal<String>))>;
    let (get_entries, set_entries): (ReadSignal<EntryListTuple>, WriteSignal<EntryListTuple>) =
        create_signal(
            entry_list
                .unwrap_or_else(|| vec![] )
                .iter()
                .map(|s| {
                    let new_id: u16 = unique_id;
                    unique_id += 1;
                    let is_edit_signal = create_signal(true);
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
        <div>
            //<p>{"Editable Recipe START"}</p>
            <button on:click=add_entry>
                "Add Entry"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=get_entries
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
                                    when=move || { is_edit.0.get() }

                                    // If not in edit mode:
                                    fallback= move ||
                                    {
                                        view!{ 
                                            <p>{entry}</p>
                                            <Show
                                                when=move || {editable.0.get()}
                                            >
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
                                        };
                                        
                                        view!{
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
                                        }
                                    }
                                </Show>
                            </li>
                        }
                    }
                />
            </ul>
            //<p>{"Editable Recipe END"}</p>
        </div>
    }
}