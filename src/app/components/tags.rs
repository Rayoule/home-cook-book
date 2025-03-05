use crate::app::IsTagsMenuOpen;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[component]
pub fn TagList(
    // All tags available
    all_tags: RwSignal<Vec<String>>,
    // Tags that are selected
    selected_tags_signal: RwSignal<Vec<String>>,
) -> impl IntoView {

    let tags_state = move || {
        all_tags
            .get()
            .iter()
            .map(|t| RwSignal::new((selected_tags_signal.read().contains(t), t.clone())))
            .collect::<Vec<RwSignal<(bool, String)>>>()
    };

    let all_tag_elems = move || {
        // then generate the buttons
        tags_state()
            .into_iter()
            .map(|tag_state_signal| {
                view_from_tag_state(tag_state_signal, selected_tags_signal)
            })
            .rev()
            .collect_view()
    };

    let is_tags_menu_open = use_context::<IsTagsMenuOpen>()
        .expect("Expected to find IsTagsMenuOpen in context.")
        .0;

    let on_clear_tags_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        // clear selected_tags_signal
        selected_tags_signal.set(vec![]);
        // update tag_button_state_signals
        tags_state()
            .iter()
            .for_each(|tag_state_signal| {
                tag_state_signal.update(|(is_tag_selected, _)| {
                    *is_tag_selected = false;
                });
            });
    };

    let on_unroll_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        is_tags_menu_open.set(true);
    };

    view! {

        <div class="unroll-tags-button-container">
            <p
                class="unroll-tags-button-notifier"
                class:hide-notifier=move || selected_tags_signal.read().is_empty()
            >
                { move || selected_tags_signal.read().len() }
            </p>
            <button
                class="unroll-tags-button"
                class:hide-notifier=move || selected_tags_signal.read().is_empty()
                on:click=on_unroll_click
            >
            </button>
        </div>

        <div
            class="background-blocker tags-blocker"
            class:is-enabled=is_tags_menu_open
            on:click=move |ev| {
                ev.stop_propagation();
                is_tags_menu_open.set(false);
            }
        ></div>

        <div
            class="tags-container"
            class:unrolled=is_tags_menu_open
            on:click=move |_| {
                is_tags_menu_open.set(false);
            }
        >

            <button
                class="close-tags-button"
                on:click=move |ev| {
                    ev.stop_propagation();
                    is_tags_menu_open.set(false);
                }
            ></button>

            <ul
                class="tag-list"
            >
                {all_tag_elems}
                <br/>
                <button
                    class="clear-tags-button"
                    on:click=on_clear_tags_click
                >
                    {"Clear"}
                </button>
            </ul>

            /*<button
                class="clear-tags-button"
                on:click=on_clear_tags_click
            >
                {"Clear"}
            </button>*/

        </div>
    }
}

fn view_from_tag_state(
    tag_state_signal: RwSignal<(bool, String)>,
    selected_tags_signal: RwSignal<Vec<String>>,
) -> AnyView {
    view! {
        <li class="tag-list-entry">
            <button
                class="tag-button"
                class:tag-selected = move || tag_state_signal.get().0
                on:click = move |ev: MouseEvent| {
                    ev.stop_propagation();
                    // update the signal
                    tag_state_signal.update(|(tag_selected, tag_name)| {
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
                { move || tag_state_signal.get().1 }
            </button>
        </li>
    }
    .into_any()
}
