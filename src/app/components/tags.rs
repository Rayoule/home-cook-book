use leptos::ev::MouseEvent;
use leptos::*;
use leptos::logging::log;


#[component]
pub fn TagList(
    // All tags available
    tags: Vec<String>,
    // Tags that are selected
    selected_tags_signal: RwSignal<Vec<String>>,
    // Tags that are already checked (needed because the component might redraw if tags are added or removed)
    // This needs to be updated ONLY if tags are added or removed (through addind/removing recipes)
    //already_selected_tags: ReadSignal<Vec<String>>,
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

    // Iterate over to make the collect_view
    let tag_elems =move || {
        // get the signals
        let mut tags_states_signals = tags_states_signals.get();
        // sort with selected in front, then in alphabetical order
        tags_states_signals
            .sort_by(|a, b| {
                let a = a.0.get();
                let b = b.0.get();

                if a.0 != b.0 {
                    b.0.cmp(&a.0)
                } else {
                    a.1.cmp(&b.1)
                }
            });
        // then generate the buttons
        tags_states_signals
            .into_iter()
            .map(|(tag_state, set_tag_state)| {
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
                }
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
        <div class="tags-container">

            <button
                class="unroll-tags-button"
                class:is-enabled=is_unrolled
                on:click=on_unroll_click
            >
            </button>

            <div
                class="unrollable-tag-panel"
                class:unrolled=is_unrolled
            >

                <p>{"Tags"}</p>

                <button
                    class="clear-tags-button"
                    on:click=on_clear_tags_click
                >
                    {"Clear Tags"}
                </button>
            </div>

            <ul
                class="tag-list"
                class:unrolled=is_unrolled
            >
                {tag_elems}
            </ul>

        </div>
    }
}