use leptos::{ *,
    html::Input,
    logging::log
};
use leptos_router::A;
use serde::{Serialize, Deserialize};

use crate::app::elements::recipe_elements::*;

// Main Recipe Format
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    // The primary key as it is stored into the database
    pub id: Option<u16>,
    pub name: String,
    pub tags:Option<Vec<RecipeTag>>,
    pub ingredients: Option<Vec<RecipeIngredient>>,
    pub instructions: Option<Vec<RecipeInstruction>>,
    pub notes: Option<Vec<RecipeNote>>,
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
                if tags_to_check.contains(&tags[i].name) {
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
    pub tags:Option<Vec<RecipeTag>>,
    pub ingredients: Option<Vec<RecipeIngredient>>,
    pub instructions: Option<Vec<RecipeInstruction>>,
    pub notes: Option<Vec<RecipeNote>>,
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


pub enum RecipeEntryType {
    Tag,
    Ingredients,
    Instructions,
    Notes,
}

impl RecipeEntryType {
    pub fn title_and_class(&self) -> (String, String) {
        (
            match self {
                RecipeEntryType::Tag => "Tags".to_owned(),
                RecipeEntryType::Ingredients => "Ingredients".to_owned(),
                RecipeEntryType::Instructions => "Instructions".to_owned(),
                RecipeEntryType::Notes => "Notes".to_owned(),
            },
            match self {
                RecipeEntryType::Tag => "recipe-tags".to_owned(),
                RecipeEntryType::Ingredients => "recipe-ingredients".to_owned(),
                RecipeEntryType::Instructions => "recipe-instructions".to_owned(),
                RecipeEntryType::Notes => "recipe-notes".to_owned(),
            }
        )
    }
}

pub trait RecipeEntry: IntoView + Clone + Default + 'static {

    fn get_entry_type() -> RecipeEntryType;
    fn create_inputs_node_refs() -> Vec<NodeRef<Input>>;
    fn extract_value(node_refs: Vec<NodeRef<Input>>) -> Self;
    fn into_editable_view(self, node_refs: Vec<NodeRef<Input>>) -> View;
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe);
}


/// Tags and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeTag {
    pub name: String,
}

impl IntoView for RecipeTag {

    fn into_view(self) -> View {
        view! {
            <p>{self.name}</p>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeTag {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Tag }

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        vec![create_node_ref::<Input>()]
    }

    fn extract_value(node_refs: Vec<NodeRef<Input>>) -> Self {
        if node_refs.len() != 1 { panic!("NodeRefs number is not matching !") }
        RecipeTag {
            name: node_refs[0].get().expect("<input> to exist").value(),
        }
    }

    fn into_editable_view(self, node_refs: Vec<NodeRef<Input>>) -> View {
        view! {
            <input 
                type="text"
                id="recipe-entry"
                placeholder="Instruction content..."
                node_ref=node_refs[0]
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<RecipeTag>>, recipe: &mut Recipe) {
        recipe.tags = entries;
    }
}


/// Instructions and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub quantity: u16,
    pub unit: String,
    pub content: String,
}

impl IntoView for RecipeIngredient {

    fn into_view(self) -> View {
        view! {
            <p>{self.content}</p>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeIngredient {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Instructions }

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        (0..3)
            .map(|_| create_node_ref::<Input>())
            .collect()
    }

    fn extract_value(node_refs: Vec<NodeRef<Input>>) -> Self {
        if node_refs.len() != 3 { panic!("NodeRefs number is not matching !") }
        RecipeIngredient {
            quantity:   {
                node_refs[0]
                    .get()
                    .expect("<input> to exist")
                    .value()
                    .parse::<u16>()
                    .expect("to parse into a number !")
            },
            unit:       node_refs[1].get().expect("<input> to exist").value(),
            content:    node_refs[2].get().expect("<input> to exist").value(),
        }
    }

    fn into_editable_view(self, node_refs: Vec<NodeRef<Input>>) -> View {
        view! {
            <input 
                node_ref=node_refs[0]
                type="text"
                id="recipe-entry"
                placeholder="Instruction content..."
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe) {
        recipe.ingredients = entries;
    }
}


/// Instructions and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeInstruction {
    pub content: String,
}

impl IntoView for RecipeInstruction {
    fn into_view(self) -> View {
        view! {
            <p>{self.content}</p>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeInstruction {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Instructions }

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        vec![create_node_ref::<Input>()]
    }

    fn extract_value(node_refs: Vec<NodeRef<Input>>) -> Self {
        if node_refs.len() != 1 { panic!("NodeRefs number is not matching !") }
         
        RecipeInstruction {
            content: node_refs[0].get().expect("<input> to exist").value()
        }
    }
    
    fn into_editable_view(self, node_refs: Vec<NodeRef<Input>>) -> View {
        view! {
            <input 
                type={"text"}
                id={"recipe-entry"}
                placeholder={"Instruction content..."}
                node_ref={node_refs[0]}
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe) {
        recipe.instructions = entries;
    }
}


/// Notes and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeNote {
    pub title: String,
    pub content: String,
}

impl IntoView for RecipeNote {
    fn into_view(self) -> View {
        view! {
            <div class= "recipe-note-container" >
                <h1 class="recipe-note" >{self.title}</h1>
                <h1 class="recipe-note" >{self.content}</h1>
            </div>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeNote {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Notes }

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        (0..2)
            .map(|_| create_node_ref::<Input>())
            .collect()
    }

    fn extract_value(node_refs: Vec<NodeRef<Input>>) -> Self {

        if node_refs.len() != 2 { panic!("NodeRefs number is not matching !") }
         
        RecipeNote {
            title:      node_refs[0].get().expect("<input> to exist").value(),
            content:    node_refs[1].get().expect("<input> to exist").value(),
        }
    }
    
    fn into_editable_view(self, node_refs: Vec<NodeRef<Input>>) -> View {
        view! {
            <input 
                type="text"
                id="recipe-entry"
                placeholder="Note Title..."
                node_ref=node_refs[0]
            />
            <input
                type="text"
                id="recipe-entry"
                placeholder="Note Content..."
                node_ref=node_refs[1]
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe) {
        recipe.notes = entries;
    }
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
            <EditableEntryList
                entry_list=recipe.tags
                entry_type=RecipeEntryType::Tag
            />

            // Ingredients
            <EditableEntryList
                entry_list=recipe.ingredients
                entry_type=RecipeEntryType::Ingredients
            />

            // Instructions
            <EditableEntryList
                entry_list=recipe.instructions
                entry_type=RecipeEntryType::Instructions
            />

            // Notes
            <EditableEntryList
                entry_list=recipe.notes
                entry_type=RecipeEntryType::Notes
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
            <EditableEntryList
                entry_list=recipe.tags
                entry_type=RecipeEntryType::Tag
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Ingredients
            <EditableEntryList
                entry_list=recipe.ingredients
                entry_type=RecipeEntryType::Ingredients
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Instructions
            <EditableEntryList
                entry_list=recipe.instructions
                entry_type=RecipeEntryType::Instructions
                recipe_setter=recipe_setter.clone()
                editable=editable
            />

            // Notes
            <EditableEntryList
                entry_list=recipe.notes
                entry_type=RecipeEntryType::Notes
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
