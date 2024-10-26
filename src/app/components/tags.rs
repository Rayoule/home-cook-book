use leptos::*;
use leptos::ev::MouseEvent;
use crate::app::{
    elements::icons_svg::CloseTagsSVG,
    IsSettingsMenuOpen,
    IsTagsMenuOpen
};


#[component]
pub fn TagList(
    // All tags available
    tags: Vec<String>,
    // Tags that are selected
    selected_tags_signal: RwSignal<Vec<String>>,
) -> impl IntoView {
    
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

    let all_tag_elems = move || {
        // get the signals
        let tags_states_signals = tags_states_signals.get();
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

    let is_tags_menu_open =
        use_context::<IsTagsMenuOpen>()
            .expect("Expected to find IsTagsMenuOpen in context.")
            .0;
    
    let is_settings_menu_open =
        use_context::<IsSettingsMenuOpen>()
            .expect("Expected to find IsSettingsMenuOpen in context.")
            .0;

    let on_unroll_click = move |ev:MouseEvent| {
        ev.stop_propagation();
        is_tags_menu_open.set(true);
    };
    

    view! {

        <Show
            when=move || { !is_settings_menu_open.get() }
        >
            <button
                class="unroll-tags-button"
                class:is-enabled=is_tags_menu_open
                on:click=on_unroll_click
            >
            </button>
        </Show>

        <div
            class="tags-container"
            class:unrolled=is_tags_menu_open
            on:click=move |_| {
                is_tags_menu_open.set(false);
            }
        >

            <button
                class="close-tag-menu-button"
                on:click=move |ev: MouseEvent| {
                    ev.stop_propagation();
                    is_tags_menu_open.set(false);
                }
            >
                <CloseTagsSVG/>
            </button>

            <ul
                class="tag-list"
            >
                {all_tag_elems}
            </ul>

            <button
                class="clear-tags-button"
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
                on:click = move |ev: MouseEvent| {
                    ev.stop_propagation();
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

