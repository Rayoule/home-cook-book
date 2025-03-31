use crate::app::elements::popups::ServerWarningPopup;
use crate::app::{IsPageDirtySignal, LoginCheckResource, PageColor};
use crate::app::{
    elements::recipe_elements::*, Recipe, RecipeActionDescriptor, RecipeEntry,
    RecipeEntryType, RecipeIngredient, RecipeInstruction, RecipeLight, RecipeNote,
    RecipeServerAction, RecipeTag, ThemeColor,
};
use leptos::ev::MouseEvent;
use leptos::html::Div;
use leptos::{logging::*, prelude::*};

#[component]
pub fn RecipeCard(recipe_light: RecipeLight, color: ThemeColor) -> impl IntoView {
    
    // Is logged in ?
    let check_login_resource = use_context::<LoginCheckResource>()
        .expect("Expected to find LoginCheckAction in context")
        .0;

    // Recipe Action
    let recipe_action = use_context::<RecipeServerAction>()
        .expect("To find RecipeServerAction in context.")
        .0;

    // Setup context with the recipe light getter
    let (recipe_id_getter, _) = signal(recipe_light.id);

    let (recipe_id, recipe_name, recipe_tags) =
        (recipe_light.id, recipe_light.name, recipe_light.tags);


    // Closure that updates PageColor in context
    let update_page_color_context = move |new_color: ThemeColor| {
        // Fetch page color
        use_context::<PageColor>()
            .expect("To find PageColor in context.")
            .0
            // Update it
            .set(new_color);
    };

    // On RecipeCard click
    let on_click = move |_| {
        // Update page color
        update_page_color_context(color);
        // Navigate to display page
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
                .unwrap_or(Vec::new())
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
                + &color.as_border_main_color()
        } else {
            color.as_bg_main_color() + &color.as_alt_color()
        }
    };

    let recipe_card_button_style = move || {
        if is_menu_open.get() {
            color.as_bg_main_color()
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
                        <ServerWarningPopup
                            text="Wait for Login Check...".to_string()
                        />
                    }}
                >
                    <Show
                        when=move || { check_login_resource.get() == Some(true) }
                    >
                        <span
                            class= "sub-menu-option"
                            style=color.as_visible_color()
                            on:click=move |ev| {
                                ev.stop_propagation();
                                // Update page color
                                update_page_color_context(color);
                                // Navigate to edit page
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
                            style=color.as_visible_color()
                            on:click=move |ev| {
                                ev.stop_propagation();
                                recipe_action.dispatch(RecipeActionDescriptor::Duplicate(recipe_id_getter.get()));
                            }
                        >{"Duplicate"}</span>


                    </Show>
                </Transition>

                <span
                    class= "sub-menu-option"
                    style=color.as_visible_color()
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

    // Fetch page color
    let theme_color = use_context::<PageColor>()
        .expect("To find PageColor in context.")
        .0;

    // Extract a prefix, a number as f32, then a suffix.
    use regex::Regex;
    fn extract_number(input: &str) -> Option<(&str, f32, &str)> {
        let re = Regex::new(r"^([^0-9.,]*)([0-9]+(?:[.,][0-9]+)?)(.*)$").unwrap();
    
        if let Some(caps) = re.captures(input) {
            let prefix = caps.get(1).map_or("", |m| m.as_str());
            let num_str = caps.get(2).unwrap().as_str().replace(',', "."); // Normalize commas to dots
            let suffix = caps.get(3).map_or("", |m| m.as_str());
    
            if let Ok(num) = num_str.parse::<f32>() {
                return Some((prefix, num, suffix));
            }
        }
    
        None
    }


    let are_ingrs_empty: bool = recipe.ingredients.is_none();

    let multiplier: RwSignal<f32> = RwSignal::new(1.0);

    let ingredient_list = move || {
        let mult_value = multiplier.get();
        recipe
            .ingredients
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .map(|ingredient| {
                
                let qty_unit: String = if mult_value != 1.0 {
                    match extract_number(&ingredient.qty_unit) {
                        Some((pre, num, suf)) => {
                            let mult_num = num * mult_value;

                            pre.to_string()
                            + format!("{:.2}", mult_num)
                                .trim_end_matches('0')
                                .trim_end_matches('.')
                            + suf
                        },
                        None => {
                            log!("Could not parse f32 value.");
                            ingredient.qty_unit
                        },
                    }
                } else {
                    ingredient.qty_unit
                };

                view! {
                    <li class="display-recipe ingredients">
                        <span class="display-recipe ingredients units">{ qty_unit }</span>
                        <span class="display-recipe ingredients content">{ ingredient.content }</span>
                    </li>
                }.into_any()
            })
            .collect_view()
    };
    

    let are_insts_empty = recipe.instructions.content.is_empty();

    let mut are_notes_empty: bool = false;
    let note_list = {
        recipe
            .notes
            .unwrap_or_else(|| {
                are_notes_empty = true;
                vec![]
            })
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

    let mut are_tags_empty = false;
    let tag_list = {
        recipe
            .tags
            .unwrap_or_else(|| {
                are_tags_empty = true;
                vec![]
            })
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

    view! {
        <RecipeMenu
            editable=false
            recipe_static_name=recipe.name
            recipe_id=recipe.id
        />

        <div class="display-recipe-container">

            <Show
                when=move || { !are_ingrs_empty }
            >
                <div class="display-recipe ingredients container">
                    <div>
                        <h3
                            style=move || { theme_color.get().as_visible_color() + "margin-bottom: 0;" }
                            class="display-recipe ingredients title"
                        >"Ingredients"</h3>

                        <IngredientMultiplier
                            color=theme_color
                            mult=multiplier
                        />
                    </div>

                    <ul class="display-recipe ingredients">
                        { ingredient_list() }
                    </ul>
                </div>
            </Show>
            

            <Show
                when=move || { !are_insts_empty }
            >
                <div class="display-recipe instructions container" >
                    <h3
                        style=move || { theme_color.get().as_visible_color() }
                        class="display-recipe instructions title"
                    >"Instructions"</h3>
                    <ul class="display-recipe instructions">
                        <li class="display-recipe instructions content">
                            { recipe.instructions.content.clone() }
                        </li>
                    </ul>
                </div>
            </Show>

            <Show
                when=move || { !are_notes_empty }
            >
                <div class="display-recipe notes container">
                    <h3
                        style=move || { theme_color.get().as_visible_color() }
                        class="display-recipe notes title"
                    >"Notes"</h3>
                    <ul class="display-recipe notes">
                        {note_list.clone()}
                    </ul>
                </div>
            </Show>

            <Show
                when=move || { !are_tags_empty }
            >
                <div class="display-recipe tags container">
                    <h3
                        style=move || { theme_color.get().as_visible_color() }
                        class="display-recipe tags title"
                    >"Tags"</h3>
                    <ul class="display-recipe tags">
                        {tag_list.clone()}
                    </ul>
                </div>
            </Show>

        </div>
    }
}

#[component]
pub fn PrintRecipeSheet(recipe: Recipe) -> impl IntoView {

    let mut are_ingrs_empty = false;
    let ingredient_list = {
        recipe
            .ingredients
            .unwrap_or_else(|| {
                are_ingrs_empty = true;
                vec![]
            })
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

    let are_insts_empty = recipe.instructions.content.is_empty();

    let mut are_notes_empty = false;
    let note_list = {
        recipe
            .notes
            .unwrap_or_else(|| {
                are_notes_empty = true;
                vec![]
            })
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

    view! {

        <div class="print-recipe-container">

            <h2 class="print-recipe-name" >
                { recipe.name }
            </h2>

            <Show
                when=move || { !are_ingrs_empty }
            >
                <div class="print-recipe ingredients container">
                    <h3 class="print-recipe ingredients title" >
                        "Ingredients"
                    </h3>
                    <ul class="print-recipe ingredients">
                        { ingredient_list.clone() }
                    </ul>
                </div>
            </Show>

            <Show
                when=move || { !are_insts_empty }
            >
                <div class="print-recipe instructions container" >
                    <h3 class="print-recipe instructions title" >
                        "Instructions"
                    </h3>
                    <ul class="print-recipe instructions">
                        <li class="display-recipe instructions content">
                            { recipe.instructions.content.clone() }
                        </li>
                    </ul>
                </div>
            </Show>

            <Show
                when=move || { !are_notes_empty }
            >
                <div class="print-recipe notes container">
                    <h3 class="print-recipe notes title" >
                        "Notes"
                    </h3>
                    <ul class="print-recipe notes">
                        { note_list.clone() }
                    </ul>
                </div>
            </Show>

        </div>

        {move || {
            // Triggers Print Dialog
            Effect::new(|_| {
                let _ = web_sys::window()
                    .expect("window should be available")
                    .print();
            });
        }}
    }
}

pub type RecipeSignals = RwSignal<(
    RwSignal<String>,
    RwSignal<Vec<(u16, ArcRwSignal<RecipeTag>)>>,
    RwSignal<
        Vec<(
            u16,
            ArcRwSignal<RecipeIngredient>,
        )>,
    >,
    RwSignal<RecipeInstruction>,
    RwSignal<Vec<(u16, ArcRwSignal<RecipeNote>)>>,
)>;

#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)] recipe: Option<Recipe>,
    #[prop(optional)] is_new_recipe: Option<bool>,
) -> impl IntoView {

    let is_new_recipe = is_new_recipe.unwrap_or(false);

    // Create the recipe if None
    let mut recipe = recipe.unwrap_or_default();

    // If this is a new recipe, then add a default empty ingredient
    if is_new_recipe {
        recipe.ingredients = Some(vec![
            RecipeIngredient {
                qty_unit: "".to_string(),
                content: "".to_string()
            }
        ])
    }

    // Fetch page color
    let theme_color = use_context::<PageColor>()
        .expect("To find PageColor in context.")
        .0;

    // Needed for move into closure view
    // for each category, make a Signal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>
    // 0.tags, 1.ingredients, 2.instructions, 3.notes
    let recipe_signals: RecipeSignals = RwSignal::new((
        RwSignal::new(recipe.name),
        RwSignal::new(entries_into_signals(recipe.tags)),
        RwSignal::new(entries_into_signals(recipe.ingredients)),
        RwSignal::new(recipe.instructions),
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
                    let sigs = recipe_signals.read();
                    //Subscribe to signals
                    let (_, t, ig, _, no) = (
                        sigs.0.track(),
                        sigs.1.read(),
                        sigs.2.read(),
                        sigs.3.track(),
                        sigs.4.read()
                    );
                    // Subscribe to every inner signal
                    t.iter().for_each(|s| { s.1.track(); });
                    ig.iter().for_each(|s| { s.1.track(); });
                    no.iter().for_each(|s| { s.1.track(); });
                },
                // Make the page dirty whenever they change
                move |_, _, _| {
                    is_page_dirty.set(true);
                },
                false,
            );
        }
    });

    view! {

        <RecipeMenu
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
) -> Vec<(u16, ArcRwSignal<T>)> {
    if let Some(entries) = entries {
        let length = entries.len() as u16;
        entries
            .into_iter()
            .zip(0..length)
            .map(|(entry, id)| {
                let new_signal = ArcRwSignal::new(entry);
                (id, new_signal)
            })
            .collect()
    } else {
        vec![]
    }
}

pub fn fetch_entries_from_signals<T: RecipeEntry>(
    signals: Vec<(u16, ArcRwSignal<T>)>,
) -> Option<Vec<T>> {
    if !signals.is_empty() {
        let entries = signals
            .iter()
            .map(|(_, rw_signal)| rw_signal.get_untracked())
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<T>>();
        Some(entries)
    } else {
        None
    }
}
