use leptos::{logging::log, ev::SubmitEvent, *};


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