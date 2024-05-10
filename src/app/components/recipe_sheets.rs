use leptos::{ ev::MouseEvent, html::Input, logging::log, *
};
use leptos_router::A;
use serde::{Serialize, Deserialize};

use crate::app::{
    components::recipe_server_functions::recipe_function, elements::{popups::DeletePopupInfo, recipe_elements::*}, Recipe, RecipeActionDescriptor, RecipeEntry, RecipeEntryType, RecipeLight
};


#[component]
pub fn RecipeLightSheet(
    recipe_light: RecipeLight,
    recipe_action: Action<RecipeActionDescriptor, Result<(), ServerFnError>>,
    delete_info: WriteSignal<Option<DeletePopupInfo>>,
) -> impl IntoView {

    // Setup context with the recipe light getter
    let (recipe_id_getter, _) = create_signal(recipe_light.id.clone());

    let (recipe_id, recipe_name, recipe_tags) = (
        recipe_light.id,
        recipe_light.name,
        recipe_light.tags
    );

    let on_click = move |_| {
        let path = "/display-recipe/".to_string() + &recipe_id.to_string();
        let navigate = leptos_router::use_navigate();
        navigate(&path, Default::default());
    };

    view! {
        <div
            class="recipe-light-container"
            on:click=on_click
        >

            <RecipeLightSubMenu
                recipe_id=      recipe_id_getter
                recipe_action=  recipe_action
                delete_info=    delete_info
            />

            <h3 class="recipe-light name">{ recipe_name }</h3>

            // Tag list
            {
                let tag_list =
                    recipe_tags
                        .unwrap_or_else(|| vec![])
                        .into_iter()
                        .map(move |t| view! {
                            <li class= "recipe-light" >
                                <span class= "recipe-light" >{t.name}</span>
                            </li>
                        })
                        .collect_view();

                view!{
                    <ul class= "recipe-light">
                        {tag_list}
                    </ul>
                }
            }
            
        </div>
    }
}




#[component]
pub fn RecipeSheet(
    recipe: Recipe,
    print: bool,
) -> impl IntoView {
    //
    

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
                        <br/>
                        <span class="display-recipe ingredients">{ingredient.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    let instruction_list = {
        recipe
            .instructions
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|instruction| {
                view! {
                    <li class="display-recipe instructions">
                        <span class="display-recipe instructions">{instruction.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    let note_list = {
        recipe
            .notes
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|note| {
                view! {
                    <li class="display-recipe notes">
                        <h4 class="display-recipe notes">{note.title}</h4>
                        <br/>
                        <span class="display-recipe notes">{note.content}</span>
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        <div class="display-recipe-continer">

            <h2 class="display-recipe name">{recipe.name}</h2>

            <div class="display-recipe tags container">
                <h3 class="display-recipe tags">"Tags"</h3>
                <ul class="display-recipe tags">
                    {tag_list}
                </ul>
            </div>

            <div class="display-recipe ingredients container">
                <h3 class="display-recipe ingredients">"Ingredients"</h3>
                <ul class="display-recipe ingredients">
                    {ingredient_list}
                </ul>
            </div>

            <div class="display-recipe instructions container">
                <h3 class="display-recipe instructions">"Instructions"</h3>
                <ul class="display-recipe instructions">
                    {instruction_list}
                </ul>
            </div>

            <div class="display-recipe notes container">
                <h3 class="display-recipe notes">"Notes"</h3>
                <ul class="display-recipe notes">
                    {note_list}
                </ul>
            </div>

        </div>
    }
}



#[component]
pub fn EditableRecipeSheet(
    recipe_action: Action<RecipeActionDescriptor, Result<(), ServerFnError>>,
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
    let recipe_signals = create_rw_signal((
        create_rw_signal(recipe.name),
        create_rw_signal( entries_into_signals(recipe.tags) ),
        create_rw_signal( entries_into_signals(recipe.ingredients) ),
        create_rw_signal( entries_into_signals(recipe.instructions) ),
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

    let on_delete_click = move |_| {
        if let Some(id) = recipe.id {
            recipe_action.dispatch(RecipeActionDescriptor::Delete(id));
        }
    };

    let on_save_click = move |_| {
        // Get recipe
        let signals = recipe_signals.get_untracked();
        // Gather recipe
        let updated_recipe = Recipe {
            id:             recipe.id.clone(),
            name:           signals.0.clone().get_untracked(),
            tags:           fetch_entries_from_signals(signals.1.get_untracked()),
            ingredients:    fetch_entries_from_signals(signals.2.get_untracked()),
            instructions:   fetch_entries_from_signals(signals.3.get_untracked()),
            notes:          fetch_entries_from_signals(signals.4.get_untracked()),
        };

        log!("Gathered Recipe => \n{:?}", updated_recipe.clone());

        // Check recipe
        match updated_recipe.valid_for_save() {
            Ok(_) => {
                // Send recipe to db
                recipe_action.dispatch(
                    if is_new_recipe {
                        RecipeActionDescriptor::Add(updated_recipe)
                    } else {
                        RecipeActionDescriptor::Save(updated_recipe)
                    }
                );
            },
            Err(e) => {
                log!("{}", e);
            },
        }
    };

    view! {
        <div class="editable-recipe" >

            {move || {

                log!("EditableRecipeSheet Rendered ----");
                
                view! {
                
                    // Name
                    <EditableRecipeName
                        name_signal=    name_signal
                        editable=       true
                    />

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
                    <EditableEntryList
                        editable=           true
                        entry_list_signal=  instructions_signal
                        entry_type=         RecipeEntryType::Instructions
                    />

                    // Notes
                    <EditableEntryList
                        editable=           true
                        entry_list_signal=  notes_signal
                        entry_type=         RecipeEntryType::Notes
                    />
                }
            }}

            // Save Button
            <Show
                when= save_pending
                fallback=move || view! {
                    <button
                        on:click=on_save_click
                    >
                        {"Save"}
                    </button>
                }.into_view()
            >
                <p>"wait for save"</p>
            </Show>
            

            {
                if is_new_recipe {
                    Some(view! {

                        <button
                            on:click=on_delete_click
                        >
                            "Delete"
                        </button>

                    }.into_view())
                } else { None }
            }
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
