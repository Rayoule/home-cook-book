use leptos::*;


#[component]
pub fn TagList(
    // All tags available
    tags: ReadSignal<Vec<String>>,
    // Tags that are selected
    selected_tags_signal: WriteSignal<Vec<String>>,
    // Tags that are already checked (needed because the component might redraw if tags are added or removed)
    // This needs to be updated ONLY if tags are added or removed (through addind/removing recipes)
    already_selected_tags: ReadSignal<Vec<String>>,
) -> impl IntoView {

    // Make the signal list from "tags"
    let tags_states_signals: Vec<(ReadSignal<(bool, String)>, WriteSignal<(bool, String)>)> =
        tags
            .get_untracked()
            .iter()
            .map(|t| {
                create_signal((already_selected_tags.get().contains(t), t.clone()))
            })
            .collect();

    // Iterate over to make the collect_view
    let tag_elems =
        tags_states_signals
            .clone()
            .into_iter()
            .map(|(tag_state, set_tag_state)| {
                view! {
                    <li>
                        <div
                            on:click=move |_| set_tag_state.update(|(tag_selected, _)| *tag_selected = !*tag_selected )
                        >
                            <p>{tag_state.get().1}</p>
                        </div>
                    </li>
                }
            })
            .collect_view();
    
    // Create effect that updates the list signal when a tag has changed
    create_render_effect(move |_| {
        selected_tags_signal.set(
            tags_states_signals
                .iter()
                .filter_map(|(state, _)| {
                    let state = state.get();
                    //leptos_dom::logging::console_log(&format!("{:?}", state.1.clone()));
                    if state.0 {
                        Some(state.1)
                    } else {
                        None
                    }
                })
                .collect()
        );
    });
    
    view! {
        <div class="tag-list">
            <p>Tags</p>
            <div>{tag_elems}</div>
        </div>
    }
}