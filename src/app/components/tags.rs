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
        tags_states_signals
            .get()
            .into_iter()
            .map(|(tag_state, set_tag_state)| {
                view! {
                    <li>
                        <button
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
    
    view! {
        <div class="tag-list">
            <p>Tags</p>
            <button
                class="clear-tags-button"
                on:click=on_clear_tags_click
            >{"Clear Tags"}</button>
            <div>{tag_elems}</div>
        </div>
    }
}