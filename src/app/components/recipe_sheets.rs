use ev::MouseEvent;
use leptos::{logging::*, *};
use crate::app::{
    elements::recipe_elements::*, IsLoggedIn, Recipe, RecipeActionDescriptor, RecipeEntry, RecipeEntryType, RecipeLight, RecipeServerAction, ThemeColor
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
            class="recipe-card"
            class:into-menu=is_menu_open
            style=recipe_card_style
            on:click=on_click
            on:mouseleave=move |_| {
                is_menu_open.set(false);
            }
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
                        on:click=move |_| {
                            recipe_action.dispatch(RecipeActionDescriptor::Duplicate(recipe_id_getter.get()));
                        }
                    >{"Duplicate"}</span>


                </Show>

                <span
                    class= "sub-menu-option"
                    style=custom_color_style.as_visible_color()
                    on:click=move |_| {
                        let print_path =
                            "/recipe/".to_owned()
                            + &recipe_id_getter.get().to_string()
                            + "/print";
                        let window = web_sys::window().expect("window should be available");
                        window
                            .open_with_url_and_target(&print_path, "_blank")
                            .unwrap();
                    }
                >{"Print"}</span>

            </Show>

        </div>
    }
}





#[component]
pub fn RecipeSheet(
    recipe: Recipe,
    //print: bool,
) -> impl IntoView {

    let tag_list = {
        recipe
            .tags
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|tag| {
                view! {
                    <li class="display-recipe tags">
                        <span class="display-recipe tags">{tag.name}</span>
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
                        <span class="display-recipe ingredients">{ingredient.quantity} {ingredient.unit}</span>
                        <span class="display-recipe ingredients">{ingredient.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    let instructions = {
            view! {
                <li class="display-recipe instructions">
                    <span class="display-recipe instructions">{recipe.instructions.content}</span>
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
                    <li class="display-recipe notes">
                        <span class="display-recipe notes">{note.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    view! {

        <RecipeMenu
            color=ThemeColor::random()
            editable=false
            recipe_name=recipe.name
            recipe_id=recipe.id.expect("Expected Recipe to have a recipe_id")
        />

        <div class="display-recipe-container">

            <div class="display-recipe tags container">
                <h3 class="display-recipe tags title">"Tags"</h3>
                <ul class="display-recipe tags">
                    {tag_list}
                </ul>
            </div>

            <div class="display-recipe ingredients container">
                <h3 class="display-recipe ingredients title">"Ingredients"</h3>
                <ul class="display-recipe ingredients">
                    {ingredient_list}
                </ul>
            </div>

            <div class="display-recipe instructions container">
                <h3 class="display-recipe instructions title">"Instructions"</h3>
                <ul class="display-recipe instructions">
                    {instructions}
                </ul>
            </div>

            <div class="display-recipe notes container">
                <h3 class="display-recipe notes title">"Notes"</h3>
                <ul class="display-recipe notes">
                    {note_list}
                </ul>
            </div>

        </div>
    }
}



#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)]
    recipe: Option<Recipe>,
    #[prop(optional)]
    is_new_recipe: Option<bool>,
) -> impl IntoView {

    let recipe_action =
        use_context::<RecipeServerAction>()
            .expect("To find RecipeServerAction in context.")
            .0;

    let is_new_recipe = is_new_recipe.unwrap_or_else(|| false);

    // Create the recipe if None
    let recipe = recipe.unwrap_or_else(|| Recipe::default());

    // Needed for move into closure view
    // for each category, make a Signal<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>
    // 0.tags, 1.ingredients, 2.instructions, 3.notes
    let recipe_signals = create_rw_signal((
        create_rw_signal( recipe.name ),
        create_rw_signal( entries_into_signals(recipe.tags) ),
        create_rw_signal( entries_into_signals(recipe.ingredients) ),
        create_signal( recipe.instructions ),
        create_rw_signal( entries_into_signals(recipe.notes) ),
    ));
    let (
        name_signal,
        tags_signal,
        ingredients_signal,
        instructions_signal,
        notes_signal
    ) = recipe_signals.get_untracked();


    let save_pending = recipe_action.pending();

    let on_save_click = move |_| {
        // Get recipe
        let signals = recipe_signals.get_untracked();
        // Gather recipe
        let updated_recipe = Recipe {
            id:             recipe.id.clone(),
            name:           signals.0.clone().get_untracked(),
            tags:           fetch_entries_from_signals(signals.1.get_untracked()),
            ingredients:    fetch_entries_from_signals(signals.2.get_untracked()),
            instructions:   signals.3.0.get_untracked(),
            notes:          fetch_entries_from_signals(signals.4.get_untracked()),
        };

        // Check recipe
        match updated_recipe.valid_for_save() {
            Ok(_) => {
                if is_new_recipe {
                    recipe_action.dispatch(RecipeActionDescriptor::Add(updated_recipe));
                } else {
                    let id = updated_recipe.id;
                    recipe_action.dispatch(RecipeActionDescriptor::Save(updated_recipe));
                    if let Some(id) = id {
                        let path = "/recipe/".to_string() + &id.to_string() + "/display";
                        let navigate = leptos_router::use_navigate();
                        navigate(&path, Default::default());
                    }
                }
            },
            Err(e) => {
                error!("{}", e);
            },
        }
    };

    view! {

        <RecipeMenu
            color=ThemeColor::random()
            editable=true
            name_signal=name_signal
        />

        <div class="editable-recipe" >

            {move || view! {

                // Tags
                <EditableEntryList
                    editable=           true
                    entry_list_signal=  tags_signal
                    entry_type=         RecipeEntryType::Tag
                />

                // Ingredients
                <EditableEntryList
                    editable=           true
                    entry_list_signal=  ingredients_signal
                    entry_type=         RecipeEntryType::Ingredients
                />

                // Instructions
                <EditableInstructions
                    editable=           true
                    entry_signal=       instructions_signal
                    entry_type=         RecipeEntryType::Instructions
                />

                // Notes
                <EditableEntryList
                    editable=           true
                    entry_list_signal=  notes_signal
                    entry_type=         RecipeEntryType::Notes
                />
            }}

            // Save Button
            <Show
                when=save_pending
                fallback=move || view! {
                    <div
                        class="save-button-container"
                    >
                        <button
                            class="round-menu-first-button"
                            on:click=on_save_click
                        >
                            {"Save"}
                        </button>
                    </div>
                }.into_view()
            >
                <p>"wait for save"</p>
            </Show>
            
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

fn fetch_entries_from_signals<T: RecipeEntry>(signals: Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>) -> Option<Vec<T>> {
    if signals.len() > 0 {
        let entries = signals
            .iter()
            .map(|(_, (get_signal, _))| get_signal.get_untracked())
            .collect();
        Some(entries)

    } else {  None }
}
