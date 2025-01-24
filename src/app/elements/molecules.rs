use leptos::prelude::*;
use leptos::html;
use leptos::ev;

use crate::app::elements::icons_svg::SearchSVG;

// Will display on top of each page in the header
#[component]
pub fn RecipeSearchBar(
    search_input: RwSignal<Vec<String>>,
    request_search_clear: RwSignal<bool>,
) -> impl IntoView {
    let input_element: NodeRef<html::Input> = NodeRef::new();

    Effect::new(move |_| {
        if request_search_clear.get() {
            // Clear search
            input_element.get().expect("Input to be mounted").set_value("");
            search_input.set(vec![]);
            request_search_clear.set(false);
        }
    });

    let on_search_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value()
            .to_lowercase();

        use regex::Regex;
        let re = Regex::new(r"\b\w+\b").unwrap();
        let search_words: Vec<String> = re
            .find_iter(&value)
            .map(|mat| mat.as_str())
            .map(|word| word.to_string())
            .collect();

        search_input.set(search_words)
    };

    view! {
        <form
            class="search-bar"
            on:submit=on_search_submit
        >
            <button
                type="submit"
                class="search-bar-button"
            >
                <SearchSVG/>
            </button>
            <input
                class="search-bar-input"
                node_ref=input_element
                // On Input, if empty -> clear search
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    if value.is_empty() {
                        search_input.set(vec![]);
                    }
                }
            >
            </input>
        </form>
    }
}

#[component]
pub fn LoadingElem(text: String) -> impl IntoView {
    view! {
        <div class="loading-elem" >
            <p class="loading-elem-content" > { text } </p>
        </div>
    }
}
