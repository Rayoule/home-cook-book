use leptos::{logging::*, *};
use serde::{Serialize, Deserialize};

use crate::app::elements::recipe_elements::*;

/// Main Recipe Format
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    // The primary key as it is stored into the database
    pub id: Option<u16>,
    pub name: String,
    pub tags:Option<Vec<RecipeTag>>,
    pub ingredients: Option<Vec<RecipeIngredient>>,
    pub instructions: RecipeInstruction,
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
        recipe_text += self.name.as_str();
        // add tags
        if let Some(tags) = &self.tags {
            for t in tags {
                recipe_text += " ";
                recipe_text += t.name.as_str();
            }
        }
        // add ingredients
        if let Some(ingrs) = &self.ingredients {
            for i in ingrs {
                recipe_text += " ";
                recipe_text += i.content.as_str();
            }
        }

        // lowercase all recipe text
        recipe_text = recipe_text.to_lowercase();

        // separate text into words
        let recipe_words: Vec<&str> =
            re
                .find_iter(&recipe_text)
                .map(|mat| mat.as_str())
                .collect();
        
        // Find matching words
        search_words.iter().any(|item| {
            recipe_words.iter().any(|word| {
                // Check if recipe_words contains any of search_words
                // (find exactly matching words)
                word.contains(item)
                // Check if search_words are sub parts of recipe_words
                // (find matchings word parts. ex: find "re" in "recipe")
                || item.contains(word)
            })
        })
    }
}

/// The Recipe format, without the ID, that will be serialize into JSON
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeCollection(pub Vec<JsonRecipe>);

/// The Recipe format, without the ID, that will be serialize into JSON
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipe {
    pub name:           String,
    pub tags:           JsonRecipeTags,
    pub ingredients:    JsonRecipeIngredients,
    pub instructions:   JsonRecipeInstructions,
    pub notes:          JsonRecipeNotes,
}
impl JsonRecipe {
    pub fn to_recipe(self, id: u16) -> Recipe {
        Recipe {
            id:             Some(id),
            name:           self.name,
            tags:           self.tags.to_recipe_tags(),
            ingredients:    self.ingredients.to_recipe_ingredients(),
            instructions:   self.instructions.to_recipe_instructions(),
            notes:          self.notes.to_recipe_notes(),
        }
    }
    pub fn from_recipe(recipe: Recipe) -> JsonRecipe {
        JsonRecipe {
            name:           recipe.name,
            tags:           JsonRecipeTags::from_recipe_tags(recipe.tags),
            ingredients:    JsonRecipeIngredients::from_recipe_ingredients(recipe.ingredients),
            instructions:   JsonRecipeInstructions::from_recipe_instructions(recipe.instructions),
            notes:          JsonRecipeNotes::from_recipe_notes(recipe.notes),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeTags(Option<Vec</*(String, [u8; 3])*/ String>>);
impl JsonRecipeTags {
    pub fn to_recipe_tags(self) -> Option<Vec<RecipeTag>> {
        if let Some(tags) = self.0 {
            Some(
                tags
                    .into_iter()
                    .map(|t| RecipeTag { name: t })
                    .collect()
            )
        } else {
            None
        }
    }
    pub fn from_recipe_tags(recipe_tags: Option<Vec<RecipeTag>>) -> Self {
        JsonRecipeTags(
            if let Some(recipe_tags) = recipe_tags {
                Some(
                    recipe_tags
                        .into_iter()
                        .map(|t| t.name)
                        .collect()
                )
            } else {
                None
            }
        )
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeIngredients(Option<Vec<(String, String)>>);
impl JsonRecipeIngredients {
    pub fn to_recipe_ingredients(self) -> Option<Vec<RecipeIngredient>> {
        if let Some(ingrs) = self.0 {
            Some(
                ingrs
                    .into_iter()
                    .map(|(qty_unit, content)| RecipeIngredient {
                        qty_unit,
                        content,
                    })
                    .collect()
            )
        } else {
            None
        }
    }
    pub fn from_recipe_ingredients(recipe_ingrs: Option<Vec<RecipeIngredient>>) -> Self {
        JsonRecipeIngredients(
            if let Some(recipe_ingrs) = recipe_ingrs {
                Some(
                    recipe_ingrs
                        .into_iter()
                        .map(|i| (i.qty_unit, i.content))
                        .collect()
                )
            } else {
                None
            }
        )
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeInstructions(String);
impl JsonRecipeInstructions {
    pub fn to_recipe_instructions(self) -> RecipeInstruction {
        RecipeInstruction { content: self.0 }
    }
    pub fn from_recipe_instructions(recipe_instrs: RecipeInstruction) -> Self {
        JsonRecipeInstructions(recipe_instrs.content)
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeNotes(Option<Vec<String>>);
impl JsonRecipeNotes {
    pub fn to_recipe_notes(self) -> Option<Vec<RecipeNote>> {
        if let Some(notes) = self.0 {
            Some(
                notes
                    .into_iter()
                    .map(|content| RecipeNote { content })
                    .collect()
            )
        } else {
            None
        }
    }
    pub fn from_recipe_notes(recipe_notes: Option<Vec<RecipeNote>>) -> Self {
        JsonRecipeNotes(
            if let Some(recipe_notes) = recipe_notes {
                Some(
                    recipe_notes
                        .into_iter()
                        .map(|t| t.content)
                        .collect()
                )
            } else {
                None
            }
        )
    }
}



// Recipe format when it is stored in the DB
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipe {
    pub id:                     u16,
    pub recipe_name:            String,
    pub recipe_tags:            String,
    pub recipe_ingredients:     String,
    pub recipe_instructions:    String,
    pub recipe_notes:           String,
}

// All infos needed for AllRecipe page
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
    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View;
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String);
    fn get_string_from_field(&self, field_id: Option<usize>) -> String;
}




/// TAGs and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeTag {
    pub name: String,
    //pub color: [u8; 3],
}

impl IntoView for RecipeTag {

    fn into_view(self) -> View {
        view! {
            <p
                color= {"red"}
            >
                { self.name }
            </p>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeTag {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Tag }

    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        view! {
            <RecipeEntryInput
                class=              "tags".to_owned()
                initial_value=      entry.get_untracked().name
                placeholder=        "Tag Name...".to_owned()
                get_entry_signal=   entry
                set_entry_signal=   set_entry
                is_input=           true
            />
        }
        .into_view()
    }
    
    fn update_field_from_string_input(&mut self, _field_id: Option<usize>, input: String) {
        self.name = input;
    }
    
    fn get_string_from_field(&self, _field_id: Option<usize>) -> String {
        self.name.clone()
    }
}







/// INGREDIENTS and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub qty_unit: String,
    pub content: String,
}

impl IntoView for RecipeIngredient {

    fn into_view(self) -> View {
        view! {
            <p>{self.qty_unit}</p>
            <p>{self.content}</p>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeIngredient {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Ingredients }

    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        let entry_value = entry.get_untracked();
        view! {
            <RecipeEntryInput
                class=              "ingredients quantity".to_owned()
                initial_value=      entry_value.qty_unit.to_string()
                placeholder=        "Quantity".to_owned()
                get_entry_signal=   entry
                set_entry_signal=   set_entry
                field_id=           {0}
                is_input=           true
            />

            <div class="divider ingredients"></div>

            <RecipeEntryInput
                class=              "ingredients ingredients-content".to_owned()
                initial_value=      entry_value.content
                placeholder=        "Ingredient".to_owned()
                get_entry_signal=   entry
                set_entry_signal=   set_entry
                field_id=           {1}
                is_input=           true
            />
        }
        .into_view()
    }
    
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String) {
        match field_id {

            Some(0) => self.qty_unit = input,

            Some(1) => self.content = input,

            None => {
                error!("ERROR: No ID provided.")
            },

            _ => {
                error!("ERROR: Invalid ID.")
            },

        }
    }
    
    fn get_string_from_field(&self, field_id: Option<usize>) -> String {
        match field_id {

            Some(0) => self.qty_unit.to_string().clone(),

            Some(1) => self.content.clone(),

            None => {
                panic!("ERROR: No ID provided.")
            },

            _ => {
                panic!("ERROR: Invalid ID.")
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
    
    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        view! {
            <RecipeEntryInput
                class=              "instructions".to_owned()
                initial_value=      entry.get_untracked().content
                placeholder=        "Instruction content...".to_owned()
                get_entry_signal=   entry
                set_entry_signal=   set_entry
            />
        }
        .into_view()
    }
    
    fn update_field_from_string_input(&mut self, _field_id: Option<usize>, input: String) {
        self.content = input;
    }
    
    fn get_string_from_field(&self, _field_id: Option<usize>) -> String {
        self.content.clone()
    }
}







/// NOTES and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeNote {
    pub content: String,
}

impl IntoView for RecipeNote {
    fn into_view(self) -> View {
        view! {
            <div class= "recipe-note-container" >
                <h1 class="recipe-note" >{self.content}</h1>
            </div>
        }
        .into_view()
    }
}

impl RecipeEntry for RecipeNote {

    fn get_entry_type() -> RecipeEntryType { RecipeEntryType::Notes }
    
    fn into_editable_view(entry: ReadSignal<Self>, set_entry: WriteSignal<Self>) -> View {
        let entry_value = entry.get_untracked();
        view! {
            <div class= "editable-recipe-note-container">
                <RecipeEntryInput
                    class=              "notes note-content".to_owned()
                    initial_value=      entry_value.content
                    placeholder=        "Note content...".to_owned()
                    get_entry_signal=   entry
                    set_entry_signal=   set_entry
                />
            </div>
        }.into_view()
    }
    
    fn update_field_from_string_input(&mut self, _field_id: Option<usize>, input: String) {
        self.content = input;
    }
    
    fn get_string_from_field(&self, _field_id: Option<usize>) -> String {
        self.content.clone()
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
