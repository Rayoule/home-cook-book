use crate::app::{IsPageDirtySignal, LoginCheckResource};
use crate::app::{
    elements::recipe_elements::*, Recipe, RecipeActionDescriptor, RecipeEntry,
    RecipeEntryType, RecipeIngredient, RecipeInstruction, RecipeLight, RecipeNote,
    RecipeServerAction, RecipeTag, ThemeColor,
};
use leptos::ev::MouseEvent;
use leptos::html::Div;
use leptos::{logging::*, prelude::*};

#[component]
pub fn RecipeCard(recipe_light: RecipeLight, custom_color_style: ThemeColor) -> impl IntoView {
    
    // Is logged in ?
    let check_login_resource = use_context::<LoginCheckResource>()
        .expect("Expected to find LoginCheckAction in context")
        .0;

    // Recipe Action
    let recipe_action = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0;

    // Setup context with the recipe light getter
    let (recipe_id_getter, _) = signal(recipe_light.id.clone());

    let (recipe_id, recipe_name, recipe_tags) =
        (recipe_light.id, recipe_light.name, recipe_light.tags);

    let on_click = move |_| {
        let path = "/recipe/".to_string() + &recipe_id.to_string() + "/display";
        let navigate = leptos_router::hooks::use_navigate();
        navigate(&path, Default::default());
    };

    let is_menu_open = RwSignal::new(false);
    let on_menu_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        is_menu_open.update(|b| *b = !*b);
    };

    // Setup for on_click_outside
    let card_ref: NodeRef<Div> = NodeRef::new();

    let menu_fallback = {
        move || {
            let tag_list = recipe_tags
                .clone()
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(move |t| {
                    view! {
                            <li class= "recipe-light">
                                <span
                                    class= "recipe-light"
                                >
                                    {t.name}
                                </span>
                            </li>
                    }
                })
                .collect_view();

            view! {
                <h3 class="recipe-light name">{ recipe_name.clone() }</h3>

                <ul class= "recipe-light">
                    {tag_list}
                </ul>
            }
        }
    };
    let menu_fallback = StoredValue::new(menu_fallback);

    let recipe_card_style = move || {
        if is_menu_open.get() {
            "background-color: var(--theme-color-bg);".to_string()
                + &custom_color_style.as_border_main_color()
        } else {
            custom_color_style.as_bg_main_color() + &custom_color_style.as_alt_color()
        }
    };

    let recipe_card_button_style = move || {
        if is_menu_open.get() {
            custom_color_style.as_bg_main_color()
        } else {
            "background-color: var(--theme-color-bg);".to_string()
        }
    };

    view! {

        {move || {
            if is_menu_open.get() {
                Effect::new(move || {
                    let _ = leptos_use::on_click_outside(card_ref, move |_| is_menu_open.set(false));
                });
            }
        }}

        <div
            node_ref=card_ref
            class="recipe-card"
            class:into-menu=is_menu_open
            style=recipe_card_style
            on:click=on_click
        >

            <button
                class="recipe-card-button"
                style=recipe_card_button_style
                on:click=on_menu_click
            >
            </button>

            <Show
                when=is_menu_open
                fallback= move || menu_fallback.read_value()()
            >

                <Transition
                    fallback=move || { view! {
                        <p class="popin-warning" >
                            "Wait for Login Check..."
                        </p>
                    }}
                >
                    <Show
                        when=move || { check_login_resource.get() == Some(true) }
                    >
                        <span
                            class= "sub-menu-option"
                            style=custom_color_style.as_visible_color()
                            on:click=move |ev| {
                                ev.stop_propagation();
                                let path =
                                    "/recipe/".to_owned()
                                    + &recipe_id_getter.get_untracked().to_string()
                                    + "/editable";
                                let navigate = leptos_router::hooks::use_navigate();
                                navigate(&path, Default::default());
                            }
                        >{"Edit"}</span>


                        <span
                            class= "sub-menu-option"
                            style=custom_color_style.as_visible_color()
                            on:click=move |ev| {
                                ev.stop_propagation();
                                recipe_action.dispatch(RecipeActionDescriptor::Duplicate(recipe_id_getter.get()));
                            }
                        >{"Duplicate"}</span>


                    </Show>
                </Transition>

                <span
                    class= "sub-menu-option"
                    style=custom_color_style.as_visible_color()
                    on:click=move |ev| {
                        ev.stop_propagation();
                        let print_path =
                            "/recipe/".to_owned()
                            + &recipe_id_getter.get().to_string()
                            + "/print";
                        let window = web_sys::window().expect("window should be available");
                        window
                            .open_with_url_and_target(&print_path, "_blank")
                            .unwrap_or_else(|_| {
                                error!("No Window found.");
                                None
                            });
                    }
                >{"Print"}</span>

            </Show>

        </div>
    }
}

#[component]
pub fn RecipeSheet(recipe: Recipe) -> impl IntoView {
    let tag_list = {
        recipe
            .tags
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|tag| {
                view! {
                    <li class="display-recipe tags">
                        { tag.name }
                    </li>
                }
            })
            .collect_view()
    };

    let ingredient_list = {
        recipe
            .ingredients
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|ingredient| {
                view! {
                    <li class="display-recipe ingredients">
                        <span class="display-recipe ingredients units">{ingredient.qty_unit}</span>
                        <span class="display-recipe ingredients content">{ingredient.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    let instructions = {
        view! {
            <li class="display-recipe instructions content">
                {recipe.instructions.content}
            </li>
        }
        .into_any()
    };

    let note_list = {
        recipe
            .notes
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|note| {
                view! {
                    <li
                        style=move || { ThemeColor::random().as_border_main_color() }
                        class="display-recipe notes"
                    >
                        <span class="display-recipe notes">{note.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    let theme_color = RwSignal::new(ThemeColor::random());

    view! {
        <RecipeMenu
            color=theme_color
            editable=false
            recipe_static_name=recipe.name
            recipe_id=recipe.id
        />

        <div class="display-recipe-container">

            <div class="display-recipe ingredients container">
                <h3
                    style=move || { theme_color.get().as_visible_color() }
                    class="display-recipe ingredients title"
                >"Ingredients"</h3>
                <ul class="display-recipe ingredients">
                    {ingredient_list}
                </ul>
            </div>

            <div class="display-recipe instructions container" >
                <h3
                    style=move || { theme_color.get().as_visible_color() }
                    class="display-recipe instructions title"
                >"Instructions"</h3>
                <ul class="display-recipe instructions">
                    {instructions}
                </ul>
            </div>

            <div class="display-recipe notes container">
                <h3
                    style=move || { theme_color.get().as_visible_color() }
                    class="display-recipe notes title"
                >"Notes"</h3>
                <ul class="display-recipe notes">
                    {note_list}
                </ul>
            </div>

            <div class="display-recipe tags container">
                <h3
                    style=move || { theme_color.get().as_visible_color() }
                    class="display-recipe tags title"
                >"Tags"</h3>
                <ul class="display-recipe tags">
                    {tag_list}
                </ul>
            </div>

        </div>
    }
}

#[component]
pub fn PrintRecipeSheet(recipe: Recipe) -> impl IntoView {
    let ingredient_list = {
        recipe
            .ingredients
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|ingredient| {
                view! {
                    <li class="display-recipe ingredients">
                        <span class="display-recipe ingredients units">{ingredient.qty_unit}</span>
                        <span class="display-recipe ingredients content">{ingredient.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    let instructions = {
        view! {
            <li class="display-recipe instructions content">
                { recipe.instructions.content }
            </li>
        }
        .into_any()
    };

    let note_list = {
        recipe
            .notes
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|note| {
                view! {
                    <li class="display-recipe notes" >
                        <span class="display-recipe notes">{note.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    // Triggers Print Dialog
    Effect::new(|_| {
        let _ = web_sys::window()
            .expect("window should be available")
            .print();
    });

    view! {

        <div class="print-recipe-container">

            <h2 class="print-recipe-name" >
                { recipe.name }
            </h2>

            <div class="print-recipe ingredients container">
                <h3 class="print-recipe ingredients title" >
                    "Ingredients"
                </h3>
                <ul class="print-recipe ingredients">
                    { ingredient_list }
                </ul>
            </div>

            <div class="print-recipe instructions container" >
                <h3 class="print-recipe instructions title" >
                    "Instructions"
                </h3>
                <ul class="print-recipe instructions">
                    { instructions }
                </ul>
            </div>

            <div class="print-recipe notes container">
                <h3 class="print-recipe notes title" >
                    "Notes"
                </h3>
                <ul class="print-recipe notes">
                    { note_list }
                </ul>
            </div>

        </div>
    }
}

type RecipeSignals = RwSignal<(
    RwSignal<String>,
    RwSignal<Vec<(u16, (ReadSignal<RecipeTag>, WriteSignal<RecipeTag>))>>,
    RwSignal<
        Vec<(
            u16,
            (ReadSignal<RecipeIngredient>, WriteSignal<RecipeIngredient>),
        )>,
    >,
    (
        ReadSignal<RecipeInstruction>,
        WriteSignal<RecipeInstruction>,
    ),
    RwSignal<Vec<(u16, (ReadSignal<RecipeNote>, WriteSignal<RecipeNote>))>>,
)>;

#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)] recipe: Option<Recipe>,
    #[prop(optional)] is_new_recipe: Option<bool>,
) -> impl IntoView {

    let is_new_recipe = is_new_recipe.unwrap_or_else(|| false);

    // Create the recipe if None
    let recipe = recipe.unwrap_or_else(|| Recipe::default());

    // Needed for move into closure view
    // for each category, make a Signal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>
    // 0.tags, 1.ingredients, 2.instructions, 3.notes
    let recipe_signals: RecipeSignals = RwSignal::new((
        RwSignal::new(recipe.name),
        RwSignal::new(entries_into_signals(recipe.tags)),
        RwSignal::new(entries_into_signals(recipe.ingredients)),
        signal(recipe.instructions),
        RwSignal::new(entries_into_signals(recipe.notes)),
    ));
    let (_, tags_signal, ingredients_signal, instructions_signal, notes_signal) =
        recipe_signals.get_untracked();

    // Is page Dirty Signal (to know if we need to save it before leaving)
    let is_page_dirty = use_context::<IsPageDirtySignal>()
    .expect("Expected to find IsPageDirtySignal in context")
    .0;
    // Subscribe to all recipe signals to set the page dirty if any changes, but only if the page is currently clean
    Effect::new( move |_| {
        if !is_page_dirty.get() {
            Effect::watch(
                move || {
                    // Subscribe to all events
                    //let (name, tags, ingrs, insts, notes) = *recipe_signals.read();
                    let sigs = recipe_signals.read();
                    //Subscribe to signals
                    let (_, t, ig, _, no) = (
                        sigs.0.read(),
                        sigs.1.read(),
                        sigs.2.read(),
                        sigs.3.0.read(),
                        sigs.4.read()
                    );
                    // Subscribe to every inner signal
                    t.iter().for_each(|s| { s.1.0.read(); });
                    ig.iter().for_each(|s| { s.1.0.read(); });
                    no.iter().for_each(|s| { s.1.0.read(); });
                },
                // Make the page dirty whenever they change
                move |_, _, _| {
                    is_page_dirty.set(true);
                },
                false,
            );
        }
    });

    let theme_color = RwSignal::new(ThemeColor::random());

    view! {

        <RecipeMenu
            color=theme_color
            editable=true
            recipe_static_name="".to_string()
            recipe_id=recipe.id
            is_new_recipe=is_new_recipe
            recipe_signals=recipe_signals
        />

        <div class="editable-recipe" >

            {move || view! {

                // Ingredients
                <EditableEntryList
                    rw_entries=         ingredients_signal
                    entry_type=         RecipeEntryType::Ingredients
                    theme_color=        theme_color
                />

                // Instructions
                <EditableInstructions
                    entry_signal=       instructions_signal
                    entry_type=         RecipeEntryType::Instructions
                    theme_color=        theme_color
                />

                // Notes
                <EditableEntryList
                    rw_entries=         notes_signal
                    entry_type=         RecipeEntryType::Notes
                    theme_color=        theme_color
                />

                // Tags
                <EditableTags
                    rw_entries=         tags_signal
                    theme_color=        theme_color
                />
            }}

        </div>
    }
}

// helper function for EditableRecipeSheet
fn entries_into_signals<T: RecipeEntry>(
    entries: Option<Vec<T>>,
) -> Vec<(u16, (ReadSignal<T>, WriteSignal<T>))> {
    if let Some(entries) = entries {
        let length = entries.len() as u16;
        entries
            .into_iter()
            .zip(0..length)
            .map(|(entry, id)| {
                let new_signal = signal(entry);
                (id, new_signal)
            })
            .collect()
    } else {
        vec![]
    }
}

pub fn fetch_entries_from_signals<T: RecipeEntry>(
    signals: Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>,
) -> Option<Vec<T>> {
    if signals.len() > 0 {
        let entries = signals
            .iter()
            .map(|(_, (get_signal, _))| get_signal.clone().get_untracked())
            .collect();
        Some(entries)
    } else {
        None
    }
}
