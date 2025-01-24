use leptos::{logging::*, prelude::*};
use leptos::prelude::IntoRender;
use serde::{Deserialize, Serialize};

use crate::app::elements::recipe_elements::*;
use leptos::prelude::ElementChild;
use leptos::prelude::ClassAttribute;
use leptos::prelude::AnyView;

/// Main Recipe Format
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    // The primary key as it is stored into the database
    pub id: Option<u16>,
    pub name: String,
    pub tags: Option<Vec<RecipeTag>>,
    pub ingredients: Option<Vec<RecipeIngredient>>,
    pub instructions: RecipeInstruction,
    pub notes: Option<Vec<RecipeNote>>,
}

impl Recipe {
    /// Check if the recipe is valid to be added/saved (need only a name)
    pub fn valid_for_save(&self) -> Result<(), String> {
        if self.name.is_empty() {
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
        if tags_to_check.is_empty() {
            out = true;
        // if there are, then check the tags in recipes and then compare them
        } else if let Some(tags) = &self.tags {
            if tags_to_check.is_empty() {
                return true;
            }
            for tag in tags {
                if tags_to_check.contains(&tag.name) {
                    out = true;
                    break;
                }
            }
        }
        out
    }

    pub fn is_in_search(&self, search_words: &[String]) -> bool {
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
        let recipe_words: Vec<&str> = re.find_iter(&recipe_text).map(|mat| mat.as_str()).collect();

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
    pub name: String,
    pub tags: JsonRecipeTags,
    pub ingredients: JsonRecipeIngredients,
    pub instructions: JsonRecipeInstructions,
    pub notes: JsonRecipeNotes,
}
impl JsonRecipe {
    pub fn to_recipe(self, id: u16) -> Recipe {
        Recipe {
            id: Some(id),
            name: self.name,
            tags: self.tags.to_recipe_tags(),
            ingredients: self.ingredients.to_recipe_ingredients(),
            instructions: self.instructions.to_recipe_instructions(),
            notes: self.notes.to_recipe_notes(),
        }
    }
    pub fn from_recipe(recipe: Recipe) -> JsonRecipe {
        JsonRecipe {
            name: recipe.name,
            tags: JsonRecipeTags::from_recipe_tags(recipe.tags),
            ingredients: JsonRecipeIngredients::from_recipe_ingredients(recipe.ingredients),
            instructions: JsonRecipeInstructions::from_recipe_instructions(recipe.instructions),
            notes: JsonRecipeNotes::from_recipe_notes(recipe.notes),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeTags(Option<Vec<String>>);
impl JsonRecipeTags {
    pub fn to_recipe_tags(self) -> Option<Vec<RecipeTag>> {
        self.0.map(|tags| tags.into_iter().map(|t| RecipeTag { name: t }).collect())
    }
    pub fn from_recipe_tags(recipe_tags: Option<Vec<RecipeTag>>) -> Self {
        JsonRecipeTags(recipe_tags.map(|recipe_tags| recipe_tags.into_iter().map(|t| t.name).collect()))
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonRecipeIngredients(Option<Vec<(String, String)>>);
impl JsonRecipeIngredients {
    pub fn to_recipe_ingredients(self) -> Option<Vec<RecipeIngredient>> {
        self.0.map(|ingrs| ingrs
                    .into_iter()
                    .map(|(qty_unit, content)| RecipeIngredient { qty_unit, content })
                    .collect())
    }
    pub fn from_recipe_ingredients(recipe_ingrs: Option<Vec<RecipeIngredient>>) -> Self {
        JsonRecipeIngredients(recipe_ingrs.map(|recipe_ingrs| recipe_ingrs
                    .into_iter()
                    .map(|i| (i.qty_unit, i.content))
                    .collect()))
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
        self.0.map(|notes| notes
                    .into_iter()
                    .map(|content| RecipeNote { content })
                    .collect())
    }
    pub fn from_recipe_notes(recipe_notes: Option<Vec<RecipeNote>>) -> Self {
        JsonRecipeNotes(recipe_notes.map(|recipe_notes| recipe_notes.into_iter().map(|t| t.content).collect()))
    }
}

// Recipe format when it is stored in the DB
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipe {
    pub id: u16,
    pub recipe_name: String,
    pub recipe_tags: String,
    pub recipe_ingredients: String,
    pub recipe_instructions: String,
    pub recipe_notes: String,
}

// All infos needed for AllRecipe page
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipeLight {
    pub id: u16,
    pub recipe_name: String,
    pub recipe_tags: String,
    pub recipe_ingredients: String,
}

// Only ID
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbRowRecipeID {
    pub id: u16,
}

#[derive(PartialEq, Debug)]
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
            },
        )
    }
}

/// RecipeEntry Trait --------
pub trait RecipeEntry: std::fmt::Debug + Clone + Default + std::marker::Sync + std::marker::Send + 'static {
    type S: Get<Value = Self> + GetUntracked<Value = Self> + Update<Value = Self> + Clone + 'static;

    fn get_entry_type() -> RecipeEntryType;
    fn get_css_class_name() -> String;
    fn into_editable_view(
        rw_entry: Self::S,
        menu_info: Option<RecipeEntryMenuInfo<Self>>,
    ) -> AnyView;
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String);
    fn get_string_from_field(&self, field_id: Option<usize>) -> String;
}

/// INGREDIENTS and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub qty_unit: String,
    pub content: String,
}

impl IntoRender for RecipeIngredient {
    type Output = AnyView;
    fn into_render(self) -> AnyView {
        view! {
            <p>{self.qty_unit}</p>
            <p>{self.content}</p>
        }
        .into_any()
    }
}

impl RecipeEntry for RecipeIngredient {
    type S = ArcRwSignal<Self>;

    fn get_entry_type() -> RecipeEntryType {
        RecipeEntryType::Ingredients
    }

    fn get_css_class_name() -> String {
        "ingredients".to_string()
    }

    fn into_editable_view(
        rw_entry: Self::S,
        menu_info: Option<RecipeEntryMenuInfo<Self>>,
    ) -> AnyView {

        view! {
            <div
                class="editable-ingredients-wrapper"
            >
                <RecipeEntryMenu
                    entry_menu_info=     menu_info.expect("Expected to find menu_signal for ingredient entry.")
                />

                <RecipeEntryInput
                    class=              "ingredients quantity".to_owned()
                    placeholder=        "Qty".to_owned()
                    rw_entry=           rw_entry.clone()
                    field_id=           {0}
                    is_input=           true
                />

                <RecipeEntryInput
                    class=              "ingredients ingredients-content".to_owned()
                    placeholder=        "Ingredient".to_owned()
                    rw_entry=           rw_entry
                    field_id=           {1}
                    is_input=           true
                />
            </div>
        }
        .into_any()
    }

    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String) {
        match field_id {
            Some(0) => self.qty_unit = input,

            Some(1) => self.content = input,

            None => {
                error!("ERROR: No ID provided.")
            }

            _ => {
                error!("ERROR: Invalid ID.")
            }
        }
    }

    fn get_string_from_field(&self, field_id: Option<usize>) -> String {
        match field_id {
            Some(0) => self.qty_unit.to_string().clone(),

            Some(1) => self.content.clone(),

            None => {
                panic!("ERROR: No ID provided.")
            }

            _ => {
                panic!("ERROR: Invalid ID.")
            }
        }
    }
}

/// INSTRUCTIONS and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeInstruction {
    pub content: String,
}

impl IntoRender for RecipeInstruction {
    type Output = AnyView;
    fn into_render(self) -> AnyView {
        view! {
            <p>{self.content}</p>
        }
        .into_any()
    }
}

impl RecipeEntry for RecipeInstruction {
    type S = RwSignal<Self>;

    fn get_entry_type() -> RecipeEntryType {
        RecipeEntryType::Instructions
    }

    fn get_css_class_name() -> String {
        "instructions".to_string()
    }

    fn into_editable_view(
        rw_entry: Self::S,
        _menu_info: Option<RecipeEntryMenuInfo<Self>>,
    ) -> AnyView {
        view! {
            <RecipeEntryInput
                class=              "instructions".to_owned()
                placeholder=        "Instruction content".to_owned()
                rw_entry=           rw_entry
            />
        }
        .into_any()
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

impl IntoRender for RecipeNote {
    type Output = AnyView;
    fn into_render(self) -> AnyView {
        view! {
            <div class= "recipe-note-container" >
                <h1 class="recipe-note" >{self.content}</h1>
            </div>
        }
        .into_any()
    }
}

impl RecipeEntry for RecipeNote {
    type S = ArcRwSignal<Self>;

    fn get_entry_type() -> RecipeEntryType {
        RecipeEntryType::Notes
    }

    fn get_css_class_name() -> String {
        "notes".to_string()
    }

    fn into_editable_view(
        rw_entry: Self::S,
        menu_info: Option<RecipeEntryMenuInfo<Self>>,
    ) -> AnyView {
        view! {
            <RecipeEntryMenu
                entry_menu_info=     menu_info.expect("Expected to find menu_signal for ingredient entry.")
            />

            <RecipeEntryInput
                class=              "notes note-content".to_owned()
                placeholder=        "Note content".to_owned()
                rw_entry=           rw_entry
            />
        }
        .into_any()
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

/// TAGs and implementions -----
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeTag {
    pub name: String,
}

impl IntoRender for RecipeTag {
    type Output = AnyView;
    fn into_render(self) -> AnyView {
        view! {
            <p> { self.name } </p>
        }
        .into_any()
    }
}

impl RecipeEntry for RecipeTag {
    type S = ArcRwSignal<Self>;

    fn get_entry_type() -> RecipeEntryType {
        RecipeEntryType::Tag
    }

    fn get_css_class_name() -> String {
        "tags".to_string()
    }

    fn into_editable_view(
        rw_entry: Self::S,
        menu_info: Option<RecipeEntryMenuInfo<Self>>,
    ) -> AnyView {
        view! {
            <div class="editable-recipe tags">
                { rw_entry.get().name }

                { move || {
                    if let Some(entry_menu_info) = menu_info.clone() {
                        view! {
                            <RecipeEntryMenu
                                entry_menu_info=entry_menu_info
                            />
                        }.into_any()
                    } else { ().into_any() }
                }}
            </div>
        }
        .into_any()
    }

    fn update_field_from_string_input(&mut self, _field_id: Option<usize>, input: String) {
        self.name = input;
    }

    fn get_string_from_field(&self, _field_id: Option<usize>) -> String {
        self.name.clone()
    }
}
