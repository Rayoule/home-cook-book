use leptos::{ev::{MouseEvent, InputEvent}, logging::log, *};


// Will display on top of each page in the header
#[component]
pub fn RecipeSearchBar(
    search_input: RwSignal<Vec<String>>,
    request_search_clear: RwSignal<bool>,
) -> impl IntoView {

    let input_element: NodeRef<html::Input> = create_node_ref();

    create_effect(move |_| {
        if request_search_clear.get() {
            // Clear search
            input_element()
                .expect("Input to be mounted")
                .set_value("");
            search_input.set(vec![]);
            request_search_clear.set(false);
        }
    });

    let on_search_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            .expect("<input> should be mounted")
            .value()
            .to_lowercase();


        use regex::Regex;
        let re = Regex::new(r"\b\w+\b").unwrap();
        let search_words: Vec<String> =
            re
                .find_iter(&value)
                .map(|mat| mat.as_str())
                .map(|word| word.to_string())
                .collect();

        log!("SEARCH:\n{:?}", search_words);
        search_input.set(search_words)
    };

    view! {
        <form
            class="search-bar"
            on:submit=on_search_submit
        >
            <input
                class="search-bar-input"
                node_ref=input_element
                // On Input, if empty -> clear search
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    if value.len() < 1 {
                        search_input.set(vec![]);
                    }
                }
            >
            </input>
            <button
                type="submit"
                class="search-bar-button"
            >
                {"🔍"}
            </button>
        </form>
    }
}



/// Component that display a clickable list of suggestions
#[component]
pub fn SuggestionList(
    is_input_focused: RwSignal<bool>,
    /// the current input filled-in bu the user
    text_input: ReadSignal<String>,
    /// all possible values for this entry
    possible_values: Option<RwSignal<Vec<String>>>,
    /// the setter so the SuggestionList can fill it
    suggestion_setter: WriteSignal<String>,
) -> impl IntoView {

    view! {
        {move || {

            // return nothing if the input is not focused AND the suggestion menu is not hoverred
            if !is_input_focused.get() {
                ().into_view()
            } else if let Some(possible_values) = possible_values {

                let text_input = text_input.get();

                let mut possible_values = possible_values.get();

                possible_values.retain(|s| s.as_str().contains(&text_input));
                        
                let suggestions = possible_values
                    .clone()
                    .into_iter()
                    .map(|s| {
                        let s_cloned = s.clone();
                        view! {
                            <li
                                class="suggestions-list-entry"
                                on:click=move |ev:MouseEvent| {
                                    ev.stop_propagation();
                                    // update the input with the selected suggestion
                                    suggestion_setter.set(s_cloned.clone());
                                    // then close the suggestion menu
                                    is_input_focused.set(false);
                                }
                            >
                                {s}
                            </li>
                        }
                    })
                    .collect_view();

                let should_show_menu =
                    if possible_values.len() == 0 || text_input.len() == 0 {
                        // if no suggestions or no input -> no menu
                        false
                    } else {
                        // if the only suggestion is the one that is already written, no menu
                        if possible_values.len() == 1 && possible_values[0] == text_input {
                            false
                        } else {
                            true
                        }
                    };

                if should_show_menu {
                    view! {
                        <div>
                            <ul
                                class="suggestions-list"
                            >
                                {suggestions}
                            </ul>
                        </div>
                    }.into_view()
                } else {
                    ().into_view()
                }
            } else {
                log!("not valuuuues :(");
                ().into_view()
            }
        }}
    }
}