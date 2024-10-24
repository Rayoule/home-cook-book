use ev::select;
use leptos::ev::MouseEvent;
use leptos::*;
use leptos::logging::log;


#[component]
pub fn TagList(
    // All tags available
    tags: Vec<String>,
    // Tags that are selected
    selected_tags_signal: RwSignal<Vec<String>>,
) -> impl IntoView {

    log!("Rendering TagList");
    
    // Make the signal list from "tags"
    let tags_states_signals: RwSignal<Vec<(ReadSignal<(bool, String)>, WriteSignal<(bool, String)>)>> =  create_rw_signal({
        let already_selected_tags = selected_tags_signal.get_untracked();
        tags
            .iter()
            .map(|t| {
                create_signal((already_selected_tags.contains(t), t.clone()))
            })
            .collect()
    });

    /*let selected_tag_elems = move || {
        // get the signals
        let mut tags_states_signals = tags_states_signals.get();
        // then generate the buttons
        tags_states_signals
            .retain(|(tag_state, set_tag_state)| {
                tag_state.get().0
            });
        tags_states_signals
            .into_iter()
            .map(|(tag_state, set_tag_state)| {
                view_from_tag_state(tag_state, set_tag_state, selected_tags_signal)
            })
            .collect_view()
    };*/

    /*let unselected_tag_elems = move || {
        // get the signals
        let mut tags_states_signals = tags_states_signals.get();
        // then generate the buttons
        tags_states_signals
            .retain(|(tag_state, set_tag_state)| {
                !tag_state.get().0
            });
        tags_states_signals
            .into_iter()
            .map(|(tag_state, set_tag_state)| {
                view_from_tag_state(tag_state, set_tag_state, selected_tags_signal)
            })
            .collect_view()
    };*/

    let all_tag_elems = move || {
        // get the signals
        let mut tags_states_signals = tags_states_signals.get();
        // then generate the buttons
        tags_states_signals
            .into_iter()
            .map(|(tag_state, set_tag_state)| {
                view_from_tag_state(tag_state, set_tag_state, selected_tags_signal)
            })
            .collect_view()
    };

    let on_clear_tags_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        // clear selected_tags_signal
        selected_tags_signal.set(vec![]);
        // update tag_button_state_signals
        tags_states_signals
            .get()
            .iter()
            .for_each(|(_, set_tag_signal)| {
                set_tag_signal.update(|(is_tag_selected, _)| {
                    *is_tag_selected = false;
                });
            });
    };

    let is_unrolled = create_rw_signal(false);

    let on_unroll_click = move |ev:MouseEvent| {
        ev.stop_propagation();
        is_unrolled.set(!is_unrolled.get())
    };
    
    view! {

        <button
            class="unroll-tags-button"
            class:is-enabled=is_unrolled
            on:click=on_unroll_click
        >
        </button>

        <div
            class="tags-container"
            class:unrolled=is_unrolled
        >

            //<p>{"Tags"}</p>

            <ul
                class="tag-list"
            >
                {all_tag_elems}
            </ul>

            <button
                class="clear-tags-button"
                class:is_rolled=move || { !is_unrolled.get() }
                on:click=on_clear_tags_click
            >
                {"Clear"}
            </button>

        </div>
    }
}



fn view_from_tag_state(
    tag_state: ReadSignal<(bool, String)>,
    set_tag_state: WriteSignal<(bool, String)>,
    selected_tags_signal: RwSignal<Vec<String>>,
) -> View {
    view! {
        <li class="tag-list-entry">
            <button
                class="tag-button"
                class:tag-selected = move || tag_state.get().0
                on:click = move |_| {
                    // update the signal
                    set_tag_state.update(|(tag_selected, tag_name)| {
                        let selecting_or_deselecting = !*tag_selected;
                        selected_tags_signal.update(|tags| {
                            if selecting_or_deselecting {
                                // Add tag
                                tags.push(tag_name.clone());
                            } else {
                                // Remove Tag
                                tags.retain(|t| t != tag_name);
                            }
                        });
                        // then update the selected value
                        *tag_selected = selecting_or_deselecting;
                    })
                }
            >
                { move || tag_state.get().1 }
            </button>
        </li>
    }.into_view()
}

