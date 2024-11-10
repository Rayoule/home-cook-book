use ev::MouseEvent;
use html::Div;
use leptos::{logging::*, *};
use leptos_use::on_click_outside;
use crate::app::{
    elements::recipe_elements::*, IsLoggedIn, Recipe, RecipeActionDescriptor, RecipeEntry, RecipeEntryType, RecipeIngredient, RecipeInstruction, RecipeLight, RecipeNote, RecipeServerAction, RecipeTag, ThemeColor
};



#[component]
pub fn RecipeCard(
    recipe_light: RecipeLight,
    custom_color_style: ThemeColor,
) -> impl IntoView {

    // Is logged in ?
    let is_logged_in =
        use_context::<IsLoggedIn>()
            .expect("Expected to find IsLoggedIn in context")
            .0;
    
    // Recipe Action
    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    // Setup context with the recipe light getter
    let (recipe_id_getter, _) = create_signal(recipe_light.id.clone());

    let (recipe_id, recipe_name, recipe_tags) = (
        recipe_light.id,
        recipe_light.name,
        recipe_light.tags
    );

    let on_click = move |_| {
        let path = "/recipe/".to_string() + &recipe_id.to_string() + "/display";
        let navigate = leptos_router::use_navigate();
        navigate(&path, Default::default());
    };

    let is_menu_open = create_rw_signal(false);
    let on_menu_click = move |ev: MouseEvent| {
        ev.stop_propagation();
        is_menu_open.update(|b| *b = !*b);
    };

    // Click Outside to close menu
    let card_ref: NodeRef<Div> = create_node_ref();
    let _ = on_click_outside(card_ref, move |_| is_menu_open.set(false));

    let menu_fallback = {move || {
        let tag_list =
            recipe_tags
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
                }})
                .collect_view();

        view!{
            <h3 class="recipe-light name">{ recipe_name.clone() }</h3>

            <ul class= "recipe-light">
                {tag_list}
            </ul>
        }
    }};
    let menu_fallback = store_value(menu_fallback);

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
        <div
            node_ref=card_ref
            class="recipe-card"
            class:into-menu=is_menu_open
            style=recipe_card_style
            on:click=on_click
            /*on:mouseleave=move |_| {
                is_menu_open.set(false);
            }*/
        >

            <button
                class="recipe-card-button"
                style=recipe_card_button_style
                on:click=on_menu_click
            >
            </button>

            <Show
                when=is_menu_open
                fallback=menu_fallback
            >

                <Show
                    when=is_logged_in
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
                            let navigate = leptos_router::use_navigate();
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
pub fn RecipeSheet(
    recipe: Recipe,
) -> impl IntoView {

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
            }.into_view()
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

    let theme_color = create_rw_signal(ThemeColor::random());

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
pub fn PrintRecipeSheet(
    recipe: Recipe,
) -> impl IntoView {
    
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
            }.into_view()
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
    create_effect(|_| {
        let _ = web_sys::window().expect("window should be available").print();
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




type RecipeSignals =
    RwSignal<(
        RwSignal<String>,
        RwSignal<Vec<(u16, (ReadSignal<RecipeTag>, WriteSignal<RecipeTag>))>>,
        RwSignal<Vec<(u16, (ReadSignal<RecipeIngredient>, WriteSignal<RecipeIngredient>))>>,
        (ReadSignal<RecipeInstruction>, WriteSignal<RecipeInstruction>),
        RwSignal<Vec<(u16, (ReadSignal<RecipeNote>, WriteSignal<RecipeNote>))>>
    )>;

#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)]
    recipe: Option<Recipe>,
    #[prop(optional)]
    is_new_recipe: Option<bool>,
) -> impl IntoView {

    let is_new_recipe = is_new_recipe.unwrap_or_else(|| false);

    // Create the recipe if None
    let recipe = recipe.unwrap_or_else(|| Recipe::default());

    // Needed for move into closure view
    // for each category, make a Signal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>
    // 0.tags, 1.ingredients, 2.instructions, 3.notes
    let recipe_signals: RecipeSignals = create_rw_signal((
        create_rw_signal( recipe.name ),
        create_rw_signal( entries_into_signals(recipe.tags) ),
        create_rw_signal( entries_into_signals(recipe.ingredients) ),
        create_signal( recipe.instructions ),
        create_rw_signal( entries_into_signals(recipe.notes) ),
    ));
    let (
        _,
        tags_signal,
        ingredients_signal,
        instructions_signal,
        notes_signal
    ) = recipe_signals.get_untracked();

    let theme_color = create_rw_signal(ThemeColor::random());

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
                <EditableEntryList
                    rw_entries=         tags_signal
                    entry_type=         RecipeEntryType::Tag
                    theme_color=        theme_color
                />
            }}
            
        </div>
    }
}

// helper function for EditableRecipeSheet
fn entries_into_signals<T: RecipeEntry>(entries: Option<Vec<T>>) -> Vec<(u16, (ReadSignal<T>, WriteSignal<T>))> {
    if let Some(entries) = entries {
        let length = entries.len() as u16;
        entries
            .into_iter()
            .zip(0..length)
            .map(|(entry, id)| {
                let new_signal = create_signal(entry);
                (id, new_signal)
            })
            .collect()
    } else { vec![] }
}

pub fn fetch_entries_from_signals<T: RecipeEntry>(signals: Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>) -> Option<Vec<T>> {
    if signals.len() > 0 {
        let entries = signals
            .iter()
            .map(|(_, (get_signal, _))| get_signal.get_untracked())
            .collect();
        Some(entries)

    } else {  None }
}
