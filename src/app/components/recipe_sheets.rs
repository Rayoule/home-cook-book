use leptos::{ ev::MouseEvent, html::Input, logging::log, *
};
use leptos_router::A;
use serde::{Serialize, Deserialize};

use crate::app::{
    elements::recipe_elements::*,
    Recipe, RecipeAction, RecipeEntry, RecipeEntryType
};



#[component]
pub fn RecipeSheet(
    recipe: Recipe,
    start_expended: bool,
    recipe_action: Action<(ReadSignal<Recipe>, RecipeAction), Result<(), ServerFnError>>,
) -> impl IntoView {

    let (recipe_getter, _) = create_signal(recipe.clone());

    let is_expended = create_signal(start_expended);

    let on_div_click = move |_| {
        is_expended.1.set(!is_expended.0.get());
    };

    view! {
        <div
            class="recipe-container"
            class:expended = move || is_expended.0.get()
            on:click=on_div_click
        >

            // Edit button
            <Show
                when=move || { recipe_getter.get().id.is_some() }
                fallback=move || view!{<p>{"ERROR: Recipe has no ID !"}</p>}
            >
                {move || {
                    let id = recipe_getter.get().id.unwrap_or_default();
                    let path = "/edit-recipe/".to_string() + &id.to_string();
                    view!{
                        <A href=path>{"Edit"}</A>
                        <DeleteButton
                            recipe_getter=recipe_getter
                            recipe_action=recipe_action
                        />
                    }}}
            </Show>

            

            // Name
            <EditableRecipeName
                editable=   false
                name=       recipe.name
            />

            // Tags
            <EditableEntryList
                editable=   false
                entry_list= recipe.tags.unwrap_or_else(|| vec![])
                entry_type= RecipeEntryType::Tag
            />

            // Ingredients
            <EditableEntryList
                editable=   false
                entry_list= recipe.ingredients.unwrap_or_else(|| vec![])
                entry_type= RecipeEntryType::Ingredients
            />

            // Instructions
            <EditableEntryList
                editable=   false
                entry_list= recipe.instructions.unwrap_or_else(|| vec![])
                entry_type= RecipeEntryType::Instructions
            />

            // Notes
            <EditableEntryList
                editable=   false
                entry_list= recipe.notes.unwrap_or_else(|| vec![])
                entry_type= RecipeEntryType::Notes
            />
            
        </div>
    }
}



#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)]
    recipe: Option<Recipe>,
    #[prop(optional)]
    is_new_recipe: Option<bool>,
    recipe_action: Action<(Recipe, RecipeAction), Result<(), ServerFnError>>,
) -> impl IntoView {

    let is_new_recipe = is_new_recipe.unwrap_or_else(|| false);

    // Create the recipe if None
    let recipe = recipe.unwrap_or_else(|| Recipe::default());

    // Create signals for each recipe field
    //let name_signal = create_rw_signal(recipe.name);

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
            // Make up a ghost recipe that only has the ID
            let recipe = Recipe {
                id: Some(id),
                ..Default::default()
            };
            // Then send it
            recipe_action.dispatch((recipe, RecipeAction::Delete));
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
                recipe_action.dispatch((
                    updated_recipe,
                    if is_new_recipe {
                        RecipeAction::Add
                    } else {
                        RecipeAction::Save
                    }
                ));
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
                when=move || { save_pending.get() }
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
