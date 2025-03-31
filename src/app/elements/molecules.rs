use gloo_timers::callback::Timeout;
use leptos::prelude::*;
use leptos::html;
use leptos::ev;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

use crate::app::elements::icons_svg::CrossButtonSVG;


const SEARCH_DELAY_MS: f64 = 500.0;

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


    // Search Timeout
    let current_search_input = RwSignal::new(String::new());
    let should_cancel_timeout = RwSignal::new(false);

    let UseTimeoutFnReturn {
        start, stop, is_pending, ..
    } = use_timeout_fn(
        move |_| {
            search_input.set(
                get_search_words_from_input_value(
                    current_search_input.get()
                )
            );
        },
        SEARCH_DELAY_MS,
    );

    let stop_clone = stop.clone();

    // Reset the timeout on input
    Effect::watch(
        move || current_search_input.track(),
        move |_, _, _| {
            if is_pending.get_untracked() {
                stop();
                start(());
            } else {
                start(());
            }
        },
        false
    );

    // Cancels the timer
    Effect::new(move |_| {
        if should_cancel_timeout.get() {
            stop_clone();
            should_cancel_timeout.set(false);
        }
    });

    // Helper function to split the search into key words
    fn get_search_words_from_input_value(input_value: String) -> Vec<String> {
        let value = input_value.to_lowercase();

        use regex::Regex;
        let re = Regex::new(r"\b\w+\b").unwrap();
        re
            .find_iter(&value)
            .map(|mat| mat.as_str())
            .map(|word| word.to_string())
            .collect()
    }

    let on_search_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        // Cancel the current timeout
        should_cancel_timeout.set(true);

        // Submit search instantly
        let search_words = get_search_words_from_input_value(value);
        search_input.set(search_words);
    };

    view! {
        <form
            class="search-bar"
            on:submit=on_search_submit
        >
            <input
                class="search-bar-input"
                node_ref=input_element
                placeholder="Search"
                // On Input, if empty -> clear search, else -> submit search for timer
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    if value.is_empty() {
                        should_cancel_timeout.set(true);
                        search_input.set(vec![]);
                    }/* else {
                        current_search_input.set(value);
                    }*/
                    current_search_input.set(value);

                }
            >
            </input>
            <Show
                when=move || !current_search_input.get().is_empty()
            >
                <button
                    class="cancel-search-button"
                    on:click=move |_| {
                        request_search_clear.set(true);
                    }
                >
                    <CrossButtonSVG
                        add_class="cancel-search-svg".to_string()
                    />
                </button>
            </Show>
        </form>
    }
}

#[component]
pub fn LoadingElem(text: String) -> impl IntoView {

    let is_visible = RwSignal::new(false);

    // Wait for 0.5s to display the popup
    Effect::new(move |_| {
        let timeout = Timeout::new(500, move || {
            is_visible.set(true);
        });
        timeout.forget();
    });

    view! {
        <div
            class="loading-elem "
            class:visible=move || { is_visible.get() }
        >
            <p
                class="loading-elem-content"
                class:visible=move || { is_visible.get() }
            > { text } </p>
        </div>
    }
}
