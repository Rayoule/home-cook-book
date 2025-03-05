use crate::app::*;
use components::recipe_sheets::RecipeSignals;
use elements::icons_svg::{
    BackButtonSVG, BackupButtonSVG, CrossButtonSVG, EditButtonSVG, LogoutButtonSVG, PlusIconSVG,
    PrintButtonSVG, RemoveSVG, SortSVG, SortUpDownVG,
};
use leptos::ev::MouseEvent;
use gloo_timers::callback::Timeout;
use leptos::html::{Input, Li};
use leptos::logging::error;




#[component]
pub fn RecipeMenu(
    color: RwSignal<ThemeColor>,
    editable: bool,
    recipe_id: Option<u16>,
    recipe_static_name: String,
    #[prop(optional)] is_new_recipe: Option<bool>,
    #[prop(optional)] recipe_signals: Option<RecipeSignals>,
) -> impl IntoView {

    // Is logged in
    let check_login_resource = use_context::<LoginCheckResource>()
        .expect("Expected to find LoginCheckAction in context")
        .0;

    let menu_open = RwSignal::new(false);

    // Recipe Menu ref
    let recipe_menu_div_ref = NodeRef::<leptos::html::Div>::new();

    if !editable {
        let recipe_id = recipe_id.expect("Expected recipe ID to be Some for non edit mode");

        view! {
            
            <Transition
                fallback=move || { view! {
                    <ServerWarningPopup
                        text="Wait for Login Check...".to_string()
                    />
                }}
            >
                
                {move || {
                    if menu_open.get() {
                        let _ = leptos_use::on_click_outside(
                            recipe_menu_div_ref,
                            move |ev| {
                                menu_open.set(false);
                                ev.stop_propagation();
                            },
                        );
                    }
                }}

                <div
                    node_ref=recipe_menu_div_ref
                    class="recipe-menu"
                    class:is-open=menu_open
                    style=move || { color.get().as_bg_main_color() }
                >

                    <button
                        style=move || { color.get().as_alt_color() }
                        class="recipe-menu-button back"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate("/", Default::default());
                        }
                    >
                        <BackButtonSVG/>
                    </button>

                    <button
                        style=move || { color.get().as_alt_color() }
                        class="recipe-menu-button menu"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            menu_open.update(|b| *b = !*b);
                        }
                    >
                    </button>

                    <Show
                        when=move || { !menu_open.get() }
                    >
                        <h2
                            style=move || { color.get().as_alt_color() }
                            class="display-recipe-name"
                            class:menu-open=menu_open
                        >
                            { recipe_static_name.clone() }
                        </h2>
                    </Show>
        
                    <Show
                        when=menu_open
                    >
                        <Show
                            when=move || { check_login_resource.get() == Some(true) }
                        >
                            <button
                                style=move || { color.get().as_alt_color() }
                                class="recipe-menu-option"
                                on:click=move |ev: MouseEvent| {
                                    ev.stop_propagation();
                                    let recipe_id = recipe_id;
                                    let edit_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/editable";
                                    let navigate = leptos_router::hooks::use_navigate();
                                    navigate(&edit_path, Default::default());
                                }
                            >
                                <EditButtonSVG color=color.get().alt_color() />
                                <p class="recipe-menu-text" >"Edit"</p>
                            </button>
                        </Show>
        
                        <button
                            style=move || { color.get().as_alt_color() }
                            class="recipe-menu-option"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                let recipe_id = recipe_id;
                                let print_path = "/recipe/".to_owned() + &recipe_id.to_string() + "/print";
                                let window = web_sys::window().expect("window should be available");
                                window
                                    .open_with_url_and_target(&print_path, "_blank")
                                    .unwrap_or_else(|_| {
                                        error!("No Window found.");
                                        None
                                    });
                            }
                        >
                            <PrintButtonSVG color=color.get().alt_color() />
                            <p class="recipe-menu-text" >"Print"</p>
                        </button>
        
                        <Show
                            when=move || { check_login_resource.get() == Some(true) }
                        >
                            <button
                                style=move || { color.get().as_alt_color() }
                                class="recipe-menu-option"
                                on:click=move |ev: MouseEvent| {
                                    ev.stop_propagation();
                                    let recipe_id = recipe_id;
                                    let delete_info_signal =
                                        use_context::<DeleteInfoSignal>()
                                            .expect("To find DeleteInfoSignal in context.")
                                            .0;
                                    delete_info_signal.set( Some( DeletePopupInfo(recipe_id)) );
                                }
                            >
                                <CrossButtonSVG color=color.get().alt_color() />
                                <p class="recipe-menu-text" >"Delete"</p>
                            </button>
                        </Show>
        
                    </Show>
        
                </div>
            </Transition>
        }.into_any()
    } else {
        let recipe_action = use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;
        let save_pending = recipe_action.pending();

        let is_page_dirty = use_context::<IsPageDirtySignal>()
            .expect("Expected to find IsPageDirtySignal in context")
            .0;

        let is_new_recipe = is_new_recipe.expect("Expected is_new_recipe to be provided.");

        let recipe_signals = recipe_signals.expect("Expected recipe_signals to be provided.");
        let (name_signal, _, _, _, _) = recipe_signals.get_untracked();

        let on_save_click = move |ev: MouseEvent| {
            ev.stop_propagation();
            // Get recipe
            let signals = recipe_signals.get_untracked();
            // Gather recipe
            use components::recipe_sheets::fetch_entries_from_signals;
            let updated_recipe: Recipe = Recipe {
                id: recipe_id,
                name: signals.0.clone().get_untracked(),
                tags: fetch_entries_from_signals(signals.1.get_untracked()),
                ingredients: fetch_entries_from_signals(signals.2.get_untracked()),
                instructions: signals.3.get_untracked(),
                notes: fetch_entries_from_signals(signals.4.get_untracked()),
            };

            // Check recipe
            match updated_recipe.valid_for_save() {
                Ok(_) => {
                    if is_new_recipe {
                        // Add new recipe
                        recipe_action.dispatch(RecipeActionDescriptor::Add(updated_recipe));
                    } else {
                        // Update the id
                        let id = updated_recipe.id;
                        // Save the recipe
                        recipe_action.dispatch(RecipeActionDescriptor::Save(updated_recipe));
                        // Reroute to display
                        if let Some(id) = id {
                            let path = "/recipe/".to_string() + &id.to_string() + "/display";
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate(&path, Default::default());
                        }
                    }

                    is_page_dirty.set(false);
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        };

        view! {

            <Transition
                fallback=move || { view! {
                    <ServerWarningPopup
                        text="Wait for Login Check...".to_string()
                    />
                }}
            >

                <div
                    class="recipe-menu"
                    class:is-open=menu_open
                    style=move || { color.get().as_bg_main_color() }
                >

                    <Show
                        when=move || { !save_pending.get() }
                    >
                        <Show
                            when=is_page_dirty
                            fallback=move || view! {
                                <button
                                    class="recipe-menu-button back"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        let navigate = leptos_router::hooks::use_navigate();
                                        navigate("/", Default::default());
                                    }
                                >
                                    <BackButtonSVG/>
                                </button>
                            }
                        >
                            <button
                                class="recipe-menu-button save"
                                on:click=on_save_click
                            >
                                "save"
                            </button>
                        </Show>
                    </Show>

                    { move || {
                        view! {
                            <div class="recipe-name-input-container" >
                                <input
                                    class="text-input recipe-name"
                                    class:menu-open=menu_open
                                    type="text"
                                    id="text-input"
                                    placeholder="Recipe Name..."
                                    maxlength="45"
                                    // get_untracked() because this is only initial value
                                    value=name_signal.get_untracked()
                                    // update name_signal on input
                                    on:input=move |ev| {
                                        name_signal.set(event_target_value(&ev))
                                    }
                                />
                            </div>
                        }.into_any()
                    }}

                </div>

            </Transition>

        }
        .into_any()
    }
}

#[derive(Clone)]
pub struct RecipeEntryMenuInfo<T: RecipeEntry> {
    pub mode: RwSignal<RecipeEntryMenuMode>,
    pub all_entries: RwSignal<Vec<(u16, ArcRwSignal<T>)>>,
    pub current_id: u16,
}
#[derive(Clone, PartialEq, Debug)]
pub enum RecipeEntryMenuMode {
    Closed,
    Sort,
    Delete,
}


#[component]
pub fn IngredientMultiplier(
    color: RwSignal<ThemeColor>,
    mult: RwSignal<f32>,
) -> impl IntoView {

    let mult_ref = NodeRef::<leptos::html::Input>::new();

    view! {
        <input
            class="ingredients-multiplier"
            style=move || { color.get().as_border_main_color() }
            node_ref=mult_ref
            type="number"
            step="0.1"
            min="0"
            value="1"
            placeholder="Multiplier"
            on:input=move |ev| {
                match event_target_value(&ev).parse::<f32>() {
                    Ok(m) => mult.set(m),
                    Err(_) => log!("ERROR: Multiplier value: {} - cannot be parsed to f32", event_target_value(&ev)),
                }
            }
        />
    }
}



#[component]
pub fn EditableEntryList<T>(
    entry_type: RecipeEntryType,
    rw_entries: RwSignal<Vec<(u16, ArcRwSignal<T>)>>,
    theme_color: RwSignal<ThemeColor>,
) -> impl IntoView
where
    T: RecipeEntry<S = ArcRwSignal<T>>,
{
    let (entry_type_title, style_class) = entry_type.title_and_class();
    let style_class_clone = style_class.clone();

    // dirty signal
    let is_page_dirty = use_context::<IsPageDirtySignal>()
        .expect("Expected to find IsPageDirtySignal in context")
        .0;

    // Counter to assign new IDs
    let mut id_counter: u16 = rw_entries.read().len().try_into().unwrap();

    // Add Entry closure
    let add_entry = move |_| {
        let new_entry_signal = ArcRwSignal::new(T::default());
        rw_entries.update(move |entries| {
            entries.push((id_counter, new_entry_signal));
        });
        id_counter += 1;
        is_page_dirty.set(true);
    };

    view! {

        <div class={style_class.clone() + " container editable list"}>

            <h3
                id="field-title"
                class=style_class.clone()
                style=move || theme_color.get().as_visible_color()
            >
                { entry_type_title }
            </h3>

            <ul class=style_class.clone() >

                <For
                    each=move || rw_entries.get()
                    key=|entry| entry.0
                    children=move |(id, rw_entry)| {

                        let recipe_entry_menu_signal = RwSignal::new(RecipeEntryMenuMode::Closed);

                        let entry_menu_info = RecipeEntryMenuInfo {
                            mode: recipe_entry_menu_signal,
                            all_entries: rw_entries,
                            current_id: id
                        };

                        // Setup for on_click_outside
                        let card_ref: NodeRef<Li> = NodeRef::new();

                        view! {

                            {move || {
                                if recipe_entry_menu_signal.get() != RecipeEntryMenuMode::Closed {
                                    let _ = leptos_use::on_click_outside(
                                        card_ref,
                                        move |ev| {
                                            recipe_entry_menu_signal.update(|menu_mode| {
                                                if *menu_mode != RecipeEntryMenuMode::Closed {
                                                    *menu_mode = RecipeEntryMenuMode::Closed;
                                                }
                                            });
                                            ev.stop_propagation();
                                        },
                                    );
                                }
                            }}

                            <li
                                class= { style_class_clone.clone() }
                                id="entry-li"
                                node_ref=card_ref
                            >

                                <div
                                    class="sorting-container ".to_string() + &T::get_css_class_name()
                                >

                                    <button
                                        class="sort-up sorting-button"
                                        on:click=move |ev| {
                                            ev.stop_propagation();
                                            recipe_entry_menu_signal.update(|mode| {
                                                *mode = match *mode {
                                                    RecipeEntryMenuMode::Sort => RecipeEntryMenuMode::Closed,
                                                    _ => RecipeEntryMenuMode::Sort,
                                                };
                                            });
                                        }
                                    >
                                        <SortSVG/>
                                    </button>

                                </div>

                                // Entry
                                { T::into_editable_view(rw_entry, Some(entry_menu_info.clone())) }

                                <button
                                    class="remove-button ".to_string() + &T::get_css_class_name()
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        recipe_entry_menu_signal.update(|mode| {
                                            *mode = match *mode {
                                                RecipeEntryMenuMode::Delete => RecipeEntryMenuMode::Closed,
                                                _ => RecipeEntryMenuMode::Delete,
                                            };
                                        });
                                    }
                                >
                                    <RemoveSVG/>
                                </button>

                            </li>
                        }
                    }
                />
            
            </ul>

            <button class="add-button"
                on:click=add_entry
            >
                <PlusIconSVG add_class="add-button".to_string() />
            </button>
        </div>
    }
    .into_any()
}

#[component]
pub fn EditableInstructions(
    entry_type: RecipeEntryType,
    entry_signal: RwSignal<RecipeInstruction>,
    theme_color: RwSignal<ThemeColor>,
) -> impl IntoView {
    let (entry_type_title, style_class) = entry_type.title_and_class();

    view! {
        <div class={style_class.clone() + " container editable"}>

            <h3
                id="field-title"
                class=style_class.clone()
                style=move || theme_color.get().as_visible_color()
            >
                {entry_type_title}
            </h3>

            <li class={style_class.clone()} id="entry-li">
                { RecipeInstruction::into_editable_view(entry_signal, None) }
            </li>

        </div>
    }
    .into_any()
}

#[component]
pub fn EditableTags(
    rw_entries: RwSignal<Vec<(u16, ArcRwSignal<RecipeTag>)>>,
    theme_color: RwSignal<ThemeColor>,
) -> impl IntoView {
    let (entry_type_title, style_class) = RecipeEntryType::Tag.title_and_class();
    let style_class_clone = style_class.clone();

    let input_ref = NodeRef::<Input>::new();

    // dirty signal
    let is_page_dirty = use_context::<IsPageDirtySignal>()
        .expect("Expected to find IsPageDirtySignal in context")
        .0;

    let all_tags = use_context::<AllTagsSignal>()
        .expect("to find AllTagsMemo in context.")
        .0;

    // Counter to assign new IDs
    let mut id_counter: u16 = rw_entries.read().len().try_into().unwrap();

    // Add Entry closure
    let mut add_entry = move |new_tag_name: String| {
        if !new_tag_name.is_empty() {
            let new_entry_signal = ArcRwSignal::new(RecipeTag { name: new_tag_name });
            rw_entries.update(move |entries| {
                entries.push((id_counter, new_entry_signal));
            });
            id_counter += 1;
            is_page_dirty.set(true);
            true
        } else {
            false
        }
    };

    let current_tag_field = RwSignal::new("".to_string());

    view! {

        <div class={style_class.clone() + " container editable list"}>

            <h3
                id="field-title"
                class=style_class.clone()
                style=move || theme_color.get().as_visible_color()
            >
                { entry_type_title }
            </h3>

            <ul class=style_class.clone() >

                <For
                    each=move || rw_entries.get()
                    key=|entry| entry.0
                    children=move |(id, entry_signal)| {

                        let recipe_entry_menu_signal = RwSignal::new(RecipeEntryMenuMode::Closed);

                        let entry_menu_info = RecipeEntryMenuInfo {
                            mode: recipe_entry_menu_signal,
                            all_entries: rw_entries,
                            current_id: id
                        };

                        // Setup for on_click_outside
                        let card_ref: NodeRef<Li> = NodeRef::new();

                        view! {

                            {move || {
                                if recipe_entry_menu_signal.get() != RecipeEntryMenuMode::Closed {
                                    let _ = leptos_use::on_click_outside(card_ref, move |_| recipe_entry_menu_signal.set(RecipeEntryMenuMode::Closed));
                                }
                            }}

                            <li
                                class=style_class_clone.clone()
                                id="entry-li"
                                node_ref=card_ref
                                on:click=move |ev| {
                                    ev.stop_propagation();
                                    recipe_entry_menu_signal.update(|mode| {
                                        *mode = match mode {
                                            RecipeEntryMenuMode::Closed => RecipeEntryMenuMode::Delete,
                                            RecipeEntryMenuMode::Sort   => RecipeEntryMenuMode::Delete,
                                            RecipeEntryMenuMode::Delete => RecipeEntryMenuMode::Closed,
                                        };
                                    });
                                }
                            >

                                // Entry
                                { RecipeTag::into_editable_view(entry_signal, Some(entry_menu_info.clone())) }

                            </li>
                        }
                    }
                />

            </ul>

            <div class="tag-add-container" >
                <form
                    class="tag-add-form"
                    on:input=move |ev| {
                        current_tag_field.set(
                            event_target_value(&ev)
                                .to_lowercase()
                                .trim()
                                .to_string()
                        );
                    }
                    on:submit=move |ev| {
                        ev.prevent_default();
                        let input_node = input_ref.get().expect("Expected Input to be mounted");
                        let value = input_node.value();
                        if add_entry(value) {
                            input_node.set_value("");
                        }
                    }
                >
                    <span class="tag-add-hashtag" > "#" </span>
                    <input
                        class=          "tag-add"
                        type=           "text"
                        autocomplete=   "off"
                        id=             "text-input"
                        node_ref=       input_ref
                    />
                    <button class="tag-add-button" type="submit" >
                        <PlusIconSVG add_class="add-button".to_string() />
                    </button>
                </form>
            </div>

            <div
                class="tags-suggestions-container"
            >

                <ul
                    class="tags-suggestions"
                >
                    { move || {
                        let current_tags = rw_entries
                            .get()
                            .iter()
                            .map(|(_, tag_signal)| {
                                tag_signal.get().name
                            })
                            .collect::<Vec<String>>();
                        
                        // Filter tags to show in suggestion
                        // get all tags
                        let mut tags_to_suggest = all_tags.get();
                        // remove current tags from suggestions
                        tags_to_suggest.retain(|tag| { !current_tags.contains(tag) });
                        // get current search bar input
                        let current_tag_field = current_tag_field.get();
                        // if there is an input, then filter suggestions
                        if !current_tag_field.is_empty() {
                            tags_to_suggest.retain(|tag| { tag.to_lowercase().contains(&current_tag_field) });
                        }
                        // make tag list
                        
                        tags_to_suggest
                                .into_iter()
                                .map(|tag| {
                                    let tag_name = tag.clone();
                                    view! {
                                        <button
                                            class="tag-suggested"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                add_entry(tag_name.clone());
                                            }
                                        >
                                            { tag }
                                        </button>
                                    }
                                })
                                .collect_view()
                    }}
                </ul>
            </div>
        </div>
    }
    .into_any()
}

#[component]
pub fn DeleteButton(recipe_id: ReadSignal<u16>) -> impl IntoView {
    let delete_info_signal = use_context::<DeleteInfoSignal>()
        .expect("To find DeleteInfoReadSignal in context.")
        .0;

    let on_want_delete_click = move |_| {
        delete_info_signal.set(Some(DeletePopupInfo(recipe_id.get())));
    };

    view! {
        <span
            class= "sub-menu-option"
            on:click=on_want_delete_click
        > {"Delete"} </span>
    }
}

#[component]
pub fn DuplicateButton(recipe_id: ReadSignal<u16>) -> impl IntoView {
    let recipe_action = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0;

    let on_duplicate_click = move |_| {
        recipe_action.dispatch(RecipeActionDescriptor::Duplicate(recipe_id.get()));
    };

    view! {
        <span
            class= "sub-menu-option"
            on:click=on_duplicate_click
        >{"Duplicate"}</span>
    }
}

#[component]
pub fn PrintButton(recipe_id: ReadSignal<u16>) -> impl IntoView {
    let on_duplicate_click = move |_| {
        let print_path = "/recipe/".to_owned() + &recipe_id.get().to_string() + "/print";
        let window = web_sys::window().expect("window should be available");
        window
            .open_with_url_and_target(&print_path, "_blank")
            .unwrap_or_else(|_| {
                error!("No Window found.");
                None
            });
    };

    view! {
        <span
            class= "sub-menu-option"
            on:click=on_duplicate_click
        >{"Print"}</span>
    }
}


#[component]
pub fn RecipeEntryInput<T, S>(
    placeholder: String,
    class: String,
    rw_entry: S,
    /// If the entry has multiple fields
    #[prop(optional)]
    field_id: Option<usize>,
    #[prop(optional)] is_input: Option<bool>,
) -> impl IntoView 
where
    T: RecipeEntry,
    S: Get<Value = T> + GetUntracked<Value = T> + Update<Value = T> + Clone + 'static
{

    let is_input = is_input.unwrap_or_default();

    // setup for the SuggestionList
    let is_input_focused = RwSignal::new(false);
    let (get_input, set_input) = signal("".to_string());


    let rw_entry_clone_a = rw_entry.clone();
    let rw_entry_clone_b = rw_entry.clone();
    let rw_entry_clone_c = rw_entry.clone();
    let rw_entry_clone_d = rw_entry.clone();

    if is_input {
        view! {
            <div
                id=         "text-input"
                class=      class.clone() + " wrapper"
                on:focusin= move |_| {
                    set_input.set(
                        rw_entry.get().get_string_from_field(field_id)
                    );
                    is_input_focused.set(true);
                }
                on:focusout=move |_| {
                    let timeout = Timeout::new(250, move || {
                        is_input_focused.set(false);
                    });
                    timeout.forget();
                }
            >

                <input
                    class=          class.clone()
                    type=           "text"
                    id=             "text-input"
                    placeholder=    placeholder
                    value=          rw_entry_clone_a.get_untracked().get_string_from_field(field_id)
                    style=          move || {
                        if T::get_entry_type() == RecipeEntryType::Tag {
                            let input_length = usize::min(get_input.get().len(), 20);
                            "width: ".to_string()
                            + &input_length.to_string()
                            + "ch"
                        } else { "".to_string() }
                    }
                    on:input=       move |ev| {
                        // on input, update entry signal
                        let current_input = event_target_value(&ev);
                        set_input.set(current_input.clone());
                        rw_entry_clone_b.update(|recipe_entry| {
                            recipe_entry.update_field_from_string_input(field_id, current_input);
                        });
                    }
                />
            </div>
        }
        .into_any()
    } else {
        // Textarea
        #[allow(unused)]
        let textarea = NodeRef::<leptos::html::Textarea>::new();

        // setup for textarea autosize
        #[cfg(feature = "hydrate")]
        let leptos_use::UseTextareaAutosizeReturn {
            content: _,
            set_content,
            trigger_resize: _,
        } = leptos_use::use_textarea_autosize(textarea);

        view! {
            <div
                class="text-area-input-wrapper ".to_string() + &class
            >

                <textarea
                    class=          class.clone()
                    node_ref=       textarea
                    id=             "text-input"
                    placeholder=    placeholder
                    prop:value=     rw_entry_clone_c.get_untracked().get_string_from_field(field_id)
        
                    // on input
                    on:input=move |ev| {

                        // resize box to fit text
                        #[cfg(feature= "hydrate")]
                        set_content.set(event_target_value(&ev));

                        // update entry signal
                        rw_entry_clone_d.update(|recipe_entry| {
                            recipe_entry.update_field_from_string_input(field_id, event_target_value(&ev))
                        });
                    }
                ></textarea>

            </div>
        }
        .into_any()
    }
}

#[component]
pub fn SettingsMenu() -> impl IntoView {
    // get settings menu context
    let is_settings_menu_open = use_context::<IsSettingsMenuOpen>()
        .expect("Expected to find IsSettingsMenuOpen in context.")
        .0;

    // Is logged in
    let check_login_resource = use_context::<LoginCheckResource>()
        .expect("Expected to find LoginCheckAction in context")
        .0;

    // Get logout action
    let logout_action = use_context::<LogoutAction>()
        .expect("Expected to find LogoutAction in context")
        .0;

    // Page Name
    let page_name = use_context::<PageName>()
        .expect("Expected to find PageName in context")
        .0;

    view! {
        <button
            class = "settings-menu-button"
            class:menu-open=is_settings_menu_open
            on:click=move |_| is_settings_menu_open.update(|b| *b = !*b)
        >
        </button>

        <div
            class = "background-blocker settings-blocker"
            class:is-enabled=is_settings_menu_open
            on:click=move |ev| {
                ev.stop_propagation();
                is_settings_menu_open.set(false);
            }
        ></div>

        <div
            class = "settings-menu"
            class:is-open=is_settings_menu_open
            on:click=move |ev| {
                ev.stop_propagation();
                is_settings_menu_open.set(false);
            }
        >

            <Transition
                fallback=move || { view! {
                    <ServerWarningPopup
                        text="Waiting for Login Check...".to_string()
                    />
                }}
            >

                <Show
                    when=move || check_login_resource.get() == Some(true)
                    fallback=move || view! {
                        <div
                            class="login-container"
                            on:click=move |ev| {
                                ev.stop_propagation();
                            }
                        >
                            <LoginMenu/>
                        </div>
                    }
                >

                    // Backup
                    <Show
                        when=move || page_name.get() != "Backup"
                    >
                        <button
                            class="settings-button backup"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                ev.prevent_default();
                                is_settings_menu_open.set(false);

                                let navigate = leptos_router::hooks::use_navigate();
                                navigate("/backup", Default::default());
                            }
                        >
                            <BackupButtonSVG/>
                            <p class="settings-button-text backup" >
                                "Backup"
                            </p>
                        </button>
                    </Show>

                    // Logout
                    <button
                        class="settings-button logout"
                        on:click=move |_| {
                            is_settings_menu_open.set(false);
                            logout_action.dispatch(());
                        }
                    >
                        <LogoutButtonSVG/>
                        <p class="settings-button-text logout" >
                            "Logout"
                        </p>
                    </button>
                    
                </Show>

            </Transition>

        </div>
    }
}

#[component]
pub fn RecipeEntryMenu<T: RecipeEntry>(entry_menu_info: RecipeEntryMenuInfo<T>) -> impl IntoView {

    let RecipeEntryMenuInfo {
        mode,
        all_entries,
        current_id,
    } = entry_menu_info;

    view! {

        <div
            class="recipe-entry-menu"
            class:open=move || { mode.get() != RecipeEntryMenuMode::Closed }
            class:delete=move || { mode.get() == RecipeEntryMenuMode::Delete }
            class:sort=move || { mode.get() == RecipeEntryMenuMode::Sort }
        >
            <Show
                when=move || { mode.get() == RecipeEntryMenuMode::Sort }
            >

                {move || {

                    if let Some(index) = all_entries.read().iter().position(|x| x.0 == current_id) {
                        
                        view! {

                            { move || {
                                if index > 0 {
                                    view! {
                                        // Sort up button
                                        <button
                                            class="recipe-entry-menu-button move-up"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                all_entries.update(|entries| {
                                                    entries.swap(index, index - 1);
                                                });
                                            }
                                        >
                                            <SortUpDownVG is_up=true />
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }
                            }}

                            { move || {
                                if index < all_entries.read().len() - 1 {
                                    view! {
                                        // Sort down button
                                        <button
                                            class="recipe-entry-menu-button move-down"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                all_entries.update(|entries| {
                                                    entries.swap(index, index + 1);
                                                });
                                            }
                                        >
                                            <SortUpDownVG is_up=false />
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }
                            }}
                            
                        }.into_any()

                    } else {
                        ().into_any()
                    }
                }}

            </Show>

            <Show
                when=move || { entry_menu_info.mode.get() == RecipeEntryMenuMode::Delete }
            >
                <button
                    class="recipe-entry-menu-button delete"
                    on:click=move |ev| {

                        ev.stop_propagation();

                        // iterate in entries
                        all_entries.update(|entries| {
                            entries.retain_mut(|(entry_id, _)| {

                                // check if this is the entry to remove
                                let keep_this_entry = entry_id != &current_id;

                                if !keep_this_entry {
                                    // Remove the Delete Mode
                                    entry_menu_info.mode.set(RecipeEntryMenuMode::Closed);
                                }

                                keep_this_entry
                            })
                        });
                    }
                >
                    <CrossButtonSVG color="var(--theme-color-bg)".to_string() add_class="tag-delete".to_string() />
                </button>
            </Show>

        </div>
    }
}
