use leptos::{ev::{MouseEvent, SubmitEvent}, logging::log, *};


// Will display on top of each page in the header
#[component]
pub fn RecipeSearchBar(
    set_search_input: WriteSignal<Vec<String>>,
) -> impl IntoView {

    let input_element: NodeRef<html::Input> = create_node_ref();

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
        set_search_input.set(search_words)
    };

    view! {
        <form
            class="search-bar"
            on:submit=on_search_submit
        >
            <input
                class="search-bar-input"
                node_ref=input_element
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
    /// the current input filled-in bu the user
    text_input: ReadSignal<String>,
    /// all possible values for this entry
    possible_values: ReadSignal<Vec<String>>,
    /// the setter so the SuggestionList can fill it
    option_setter: WriteSignal<String>,
) -> impl IntoView {

    view! {
        {move || {
            let text_input = text_input.get();

            let mut possible_values = possible_values.get();

            possible_values.retain(|s| s.as_str().contains(&text_input));
                    
            let options = possible_values
                .clone()
                .into_iter()
                .map(|s| {
                    let s_cloned = s.clone();
                    view! {
                        <li
                            class="options-list-entry"
                            on:click=move |ev:MouseEvent| {
                                ev.stop_propagation();
                                option_setter.set(s_cloned.clone());
                            }
                        >
                            {s}
                        </li>
                    }
                })
                .collect_view();

            let should_show_menu =
                if possible_values.len() < 1 {
                    // if no suggestions, no menu
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
                    <ul
                        class="options-list"
                    >
                        {options}
                    </ul>
                }.into_view()
            } else {
                ().into_view()
            }
        }}
    }
}