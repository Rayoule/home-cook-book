use leptos::*;
use leptos::logging::log;
use leptos_router::A;
use serde::{Serialize, Deserialize};

use crate::app::elements::recipe_elements::*;

// Main Recipe Format
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    // The primary key as it is stored into the database
    pub id: Option<u16>,
    pub name: String,
    pub tags: Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub instructions: Option<Vec<String>>,
    pub notes: Option<Vec<String>>,
}

impl Recipe {
    /// Check if the recipe is valid to be added/saved (need only a name)
    pub fn valid_for_save(&self) -> Result<(), String> {
        if self.name.len() < 1 {
            Err("Recipe has no name".to_string())
        } else {
            Ok(())
        }
    }

    /// Check if a recipe has a tag in the given tag list
    pub fn has_tags(&self, tags_to_check: &Vec<String>) -> bool {
        
        let mut out = false;

        // if no tags to check, then all recipes valid
        if tags_to_check.len() < 1 {
            out = true;
        // if there are, then check the tags in recipes and then compare them
        } else if let Some(tags) = &self.tags {

            if tags_to_check.len() < 1 { return true }

            for i in 0..tags.len() {
                if tags_to_check.contains(&tags[i]) {
                    out = true;
                    break;
                }
            }
        }
        out
    }
}


// The Recipe format, without the ID, that will be serialize into JSON
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipe {
    pub name: String,
    pub tags:Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub instructions: Option<Vec<String>>,
    pub notes: Option<Vec<String>>,
}

impl JsonRecipe {

    pub fn from_recipe(recipe: Recipe) -> Self {
        JsonRecipe {
            name:           recipe.name,
            tags:           recipe.tags,
            ingredients:    recipe.ingredients,
            instructions:   recipe.instructions,
            notes:          recipe.notes,
        }
    }

    pub fn to_recipe(self, id: u16) -> Recipe {
        Recipe {
            id: Some(id),
            name:           self.name,
            tags:           self.tags,
            ingredients:    self.ingredients,
            instructions:   self.instructions,
            notes:          self.notes,
        }
    }
}

// Recipe format when it is stored in the DB
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipe {
    pub id: u16,
    pub recipe_name: String,
    pub recipe: String,
}


#[derive(Clone)]
pub enum RecipeContentType {
    Tags,
    Ingredients,
    Instructions,
    Notes,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RecipeAction {
    Add,
    Save,
    Delete,
}


#[component]
pub fn NewRecipe(
    recipe_action: Action<(Recipe, RecipeAction), Result<(), ServerFnError>>
) -> impl IntoView {

    let editable = create_rw_signal(true);

    view! {
        <div>
            <EditableRecipeSheet
                editable=       editable
                is_new_recipe=  true
                recipe_action=  recipe_action
            />
        </div>
    }
}


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
            <RecipeName
                name=recipe.name
            />

            // Tags
            <StringEntryList
                entry_list=recipe.tags
                entry_type=RecipeContentType::Tags
            />

            // Ingredients
            <StringEntryList
                entry_list=recipe.ingredients
                entry_type=RecipeContentType::Ingredients
            />

            // Instructions
            <StringEntryList
                entry_list=recipe.instructions
                entry_type=RecipeContentType::Instructions
            />

            // Notes
            <StringEntryList
                entry_list=recipe.notes
                entry_type=RecipeContentType::Notes
            />
            
        </div>
    }
}


#[component]
pub fn EditableRecipeSheet(
    #[prop(optional)]
    recipe: Option<Recipe>,
    #[prop(optional)]
    editable: Option<RwSignal<bool>>,
    #[prop(optional)]
    is_new_recipe: Option<bool>,
    recipe_action: Action<(Recipe, RecipeAction), Result<(), ServerFnError>>,
) -> impl IntoView {

    let editable = editable.unwrap_or_else(|| create_rw_signal(false));

    let is_new_recipe = is_new_recipe.unwrap_or_else(|| false);

    // Get the recipe
    let recipe = recipe.unwrap_or_else(|| Recipe::default());

    // Create the signal so we can edit the recipe
    let (recipe_getter, recipe_setter) = create_signal(recipe.clone());

    let save_pending = recipe_action.pending();

    let on_delete_click = move |_| {
        let recipe = recipe_getter.get();
        if let Some(_) = recipe.id {
            recipe_action.dispatch((recipe, RecipeAction::Delete));
        }
    };

    let on_save_click = move |_| {

        // Check the recipe
        log!("WARNING -> {:?}", recipe_getter.get());
        let cur_recipe = recipe_getter.get();
        match cur_recipe.valid_for_save() {
            Ok(_) => {

                // Execute the action
                recipe_action.dispatch((
                    recipe_getter.get(),
                    if is_new_recipe {
                        RecipeAction::Add
                    } else {
                        RecipeAction::Save
                    }
                ));
        
                // disable edit mode if new recipe
                if is_new_recipe {
                    editable.set(false);
                }
            },
            Err(e) => {
                log!("{}", e);
            },
        }
    };

    view! {
        <div class="editable-recipe" >

            // Name
            <RecipeName
                name=recipe.name
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Tags
            <StringEntryList
                entry_list=recipe.tags
                entry_type=RecipeContentType::Tags
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Ingredients
            <StringEntryList
                entry_list=recipe.ingredients
                entry_type=RecipeContentType::Ingredients
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Instructions
            <StringEntryList
                entry_list=recipe.instructions
                entry_type=RecipeContentType::Instructions
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Notes
            <StringEntryList
                entry_list=recipe.notes
                entry_type=RecipeContentType::Notes
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            <Show
                when=move || !editable.get()
                fallback=move || {
                    view! {

                        <Show
                            when=move || {save_pending.get()}
                            fallback=move || {view!{

                                <button
                                    on:click=on_save_click
                                >
                                    "Save"
                                </button>
                            }}
                        >
                            <p>wait for save</p>
                        </Show>

                    }
                }
            >

                <Show
                    when=move || {!is_new_recipe}
                >
                    <button
                    on:click=on_delete_click
                    >
                        "Delete"
                    </button>
                </Show>
            
            </Show>

        </div>
    }
}
