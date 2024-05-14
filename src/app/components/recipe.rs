use leptos::{ ev::MouseEvent, html::Input, logging::log, *
};
use leptos_router::A;
use regex::Replacer;
use serde::{Serialize, Deserialize};

use crate::app::{components::recipe, elements::recipe_elements::*};

use super::{recipe_server_functions::recipe_function, recipe_sheets::EditableRecipeSheet};

/// Main Recipe Format
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
}

/// Lightweight recipe format
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeLight {
    pub id: u16,
    pub name: String,
    pub tags: Option<Vec<RecipeTag>>,
    pub ingredients: Option<Vec<RecipeIngredient>>,
}

impl RecipeLight {
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

    pub fn is_in_search(&self, search_words: &Vec<String>) -> bool {
        use regex::Regex;
        let re = Regex::new(r"\b\w+\b").unwrap();

        // gather all recipe text
        let mut recipe_text: String = "".to_string();
        // add name
        log!("Name: {:?}", self.name.clone());
        recipe_text += self.name.as_str();
        // add tags
        if let Some(tags) = &self.tags {
            log!("Bim tags");
            /*let _ = tags.iter().map(|t| {
                log!("Tags: {:?}", t.name.clone());
                recipe_text += " ";
                recipe_text += t.name.as_str();
            });*/
            for t in tags {
                log!("Tags: {:?}", t.name.clone());
                recipe_text += " ";
                recipe_text += t.name.as_str();
            }
        }
        // add ingredients
        if let Some(ingrs) = &self.ingredients {
            log!("Bim Ings");
            /*let _ = ingrs.iter().map(|t| {
                log!("Ingredient: {:?}", t.content.clone());
                recipe_text += " ";
                recipe_text += t.content.as_str();
            });*/
            for i in ingrs {
                log!("Ingredient: {:?}", i.content.clone());
                recipe_text += " ";
                recipe_text += i.content.as_str();
            }
        }

        // to lowercase to match the
        recipe_text = recipe_text.to_lowercase();

        log!("RecipeText: {:?}", recipe_text.clone());

        // separate text into words
        let recipe_words: Vec<&str> =
            re
                .find_iter(&recipe_text)
                .map(|mat| mat.as_str())
                .collect();
        
        // check if recipe_words contains any of search_words
        search_words.iter().any(|item| recipe_words.contains(&item.as_str()))
    }
}


/// The Recipe format, without the ID, that will be serialize into JSON
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

/// The Recipe format, without the ID, that will be serialize into JSON
/*#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeTags {
    pub tags: Option<Vec<RecipeTag>>,
}*/

// Recipe format when it is stored in the DB
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipe {
    pub id:                 u16,
    pub recipe_name:        String,
    pub recipe_tags:        String,
    pub recipe_ingredients: String,
    pub recipe:             String,
}

// All row without the recipe
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipeLight {
    pub id:                 u16,
    pub recipe_name:        String,
    pub recipe_tags:        String,
    pub recipe_ingredients: String,
}

// Only ID
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipeID {
    pub id: u16,
}




pub enum RecipeEntryType {
    Tag,
    Ingredients,
    Instructions,
    Notes,
}


/// Returns asssociated ( Title ( Class, Editable-Class ))
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
                RecipeEntryType::Tag => "tags".to_owned(),
                RecipeEntryType::Ingredients => "ingredients".to_owned(),
                RecipeEntryType::Instructions => "instructions".to_owned(),
                RecipeEntryType::Notes => "notes".to_owned(),
            }
        )
    }
}



/// RecipeEntry Trait --------
pub trait RecipeEntry: IntoView + std::fmt::Debug + Clone + Default + 'static {

    fn get_entry_type() -> RecipeEntryType;
    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self;
    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View;
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String);
}




/// TAGs and implementions -----
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

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {
        if nodes_refs.len() != 1 { panic!("NodeRefs number is not matching !") }
        RecipeTag {
            name: nodes_refs[0].get().expect("<input> to exist").value(),
        }
    }

    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        view! {
            <RecipeEntryInput
                class=              "tags".to_owned()
                initial_value=      entry.get_untracked().name
                placeholder=        "Tag Name...".to_owned()
                set_entry_signal=   set_entry
                is_input=           true
            />
        }
        .into_view()
    }
    
    fn update_field_from_string_input(&mut self, _field_id: Option<usize>, input: String) {
        self.name = input;
    }
}







/// INGREDIENTS and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub quantity: u16,
    pub unit: String,
    pub content: String,
}

impl IntoView for RecipeIngredient {

    fn into_view(self) -> View {
        view! {
            <p>{self.quantity.to_string()}</p>
            <p>{self.unit}</p>
            <p>{self.content}</p>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeIngredient {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Instructions }

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {
        if nodes_refs.len() != 3 { panic!("NodeRefs number is not matching !") }
        RecipeIngredient {
            quantity:   {
                nodes_refs[0]
                    .get()
                    .expect("<input> to exist")
                    .value()
                    .parse::<u16>()
                    .expect("to parse into a number !")
            },
            unit:       nodes_refs[1].get().expect("<input> to exist").value(),
            content:    nodes_refs[2].get().expect("<input> to exist").value(),
        }
    }

    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        let entry = entry.get_untracked();
        view! {
            <RecipeEntryInput
                class=              "ingredients quantity".to_owned()
                initial_value=      entry.quantity.to_string()
                placeholder=        "Quantity...".to_owned()
                set_entry_signal=   set_entry
                field_id=           {0}
                is_input=           true
                is_only_numbers=    true
            />
            <RecipeEntryInput
                class=              "ingredients unit".to_owned()
                initial_value=      entry.unit
                placeholder=        "Unit...".to_owned()
                set_entry_signal=   set_entry
                field_id=           {1}
                is_input=           true
            />
            <RecipeEntryInput
                class=              "ingredients ingredients-content".to_owned()
                initial_value=      entry.content
                placeholder=        "Ingredient Content...".to_owned()
                set_entry_signal=   set_entry
                field_id=           {2}
                is_input=           true
            />
        }
        .into_view()
    }
    
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String) {
        match field_id {

            Some(0) => self.quantity = {
                match input.parse::<u16>() {
                    Ok(n) => n,
                    Err(e) => {
                        log!("{}", "ERROR: Could not parse input because: -> ".to_owned() + &e.to_string());
                        0
                    },
                }
            },

            Some(1) => self.unit = input,

            Some(2) => self.content = input,

            None => {
                log!("ERROR: No ID provided.")
            },

            _ => {
                log!("ERROR: Invalid ID.")
            },

        }
    }
}






/// INSTRUCTIONS and implementions -----
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

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {
        if nodes_refs.len() != 1 { panic!("NodeRefs number is not matching !") }
         
        RecipeInstruction {
            content: nodes_refs[0].get().expect("<input> to exist").value()
        }
    }
    
    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        view! {
            <RecipeEntryInput
                class=              "instructions".to_owned()
                initial_value=      entry.get_untracked().content
                placeholder=        "Instruction content...".to_owned()
                set_entry_signal=   set_entry
            />
        }
        .into_view()
    }
    
    fn update_field_from_string_input(&mut self, _field_id: Option<usize>, input: String) {
        self.content = input;
    }
}







/// NoOTES and implementions -----
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

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {

        if nodes_refs.len() != 2 { panic!("NodeRefs number is not matching !") }
         
        RecipeNote {
            title:      nodes_refs[0].get().expect("<input> to exist").value(),
            content:    nodes_refs[1].get().expect("<input> to exist").value(),
        }
    }
    
    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        let entry = entry.get_untracked();
        view! {
            <div class= "editable-recipe-note-container">
                <RecipeEntryInput
                    class=              "notes title".to_owned()
                    initial_value=      entry.title
                    placeholder=        "Note Title...".to_owned()
                    set_entry_signal=   set_entry
                    field_id=           {0}
                    is_input=           true
                />
                <RecipeEntryInput
                    class=              "notes note-content".to_owned()
                    initial_value=      entry.content
                    placeholder=        "Note Content...".to_owned()
                    set_entry_signal=   set_entry
                    field_id=           {1}
                />
            </div>
        }.into_view()
    }
    
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String) {
        match field_id {

            Some(0) => self.title = input,

            Some(1) => self.content = input,

            None => {
                log!("ERROR: No ID provided.")
            },

            _ => {
                log!("ERROR: Invalid ID.")
            },

        }
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
pub enum RecipeActionDescriptor {
    Add(Recipe),
    Save(Recipe),
    // With recipe ID
    Delete(u16),
    // With recipe ID
    Duplicate(u16),
}
