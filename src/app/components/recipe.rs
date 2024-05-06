use leptos::{ ev::MouseEvent, html::Input, logging::log, *
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
pub trait RecipeEntry: IntoView + Clone + Default + 'static {

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
    
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String) {
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
    
    fn update_field_from_string_input(&mut self, field_id: Option<usize>, input: String) {
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
pub enum RecipeAction {
    Add,
    Save,
    Delete,
}


#[component]
pub fn NewRecipe(
    recipe_action: Action<(Recipe, RecipeAction), Result<(), ServerFnError>>
) -> impl IntoView {

    view! {
        <div>
            <EditableRecipeSheet
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
            <EditableRecipeName
                editable=   false
                name=       recipe.name
            />

            // Tags
            <EditableEntryList
                editable=   false
                entry_list= recipe.tags.unwrap_or_default()
                entry_type= RecipeEntryType::Tag
            />

            // Ingredients
            <EditableEntryList
                editable=   false
                entry_list= recipe.ingredients.unwrap_or_default()
                entry_type= RecipeEntryType::Ingredients
            />

            // Instructions
            <EditableEntryList
                editable=   false
                entry_list= recipe.instructions.unwrap_or_default()
                entry_type= RecipeEntryType::Instructions
            />

            // Notes
            <EditableEntryList
                editable=   false
                entry_list= recipe.notes.unwrap_or_default()
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
    let name_signal = create_rw_signal(recipe.name);

    // Needed for move into closure view
    // 0.tags, 1.ingredients, 2.instructions, 3.notes
    let recipe_signals = create_rw_signal((
        entries_into_signals(recipe.tags),
        entries_into_signals(recipe.ingredients),
        entries_into_signals(recipe.instructions),
        entries_into_signals(recipe.notes),
    ));


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

        let signals = recipe_signals.get_untracked();

        // Gather recipe
        let updated_recipe = Recipe {
            id:             recipe.id.clone(),
            name:           name_signal.clone().get_untracked(),
            tags:           signals_into_entries(signals.0),
            ingredients:    signals_into_entries(signals.1),
            instructions:   signals_into_entries(signals.2),
            notes:          signals_into_entries(signals.3),
        };

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

                let (
                    tags_signals,
                    ingredients_signals,
                    instructions_signals,
                    notes_signals
                ) = recipe_signals.get();
                
                view! {
                    // Name
                    <EditableRecipeName
                        name_signal=    name_signal
                        editable=       true
                    />

                    // Tags
                    <EditableEntryList
                        editable=           true
                        entry_list_signals= tags_signals.clone().unwrap_or_default()
                        entry_type=         RecipeEntryType::Tag
                    />

                    // Ingredients
                    <EditableEntryList
                        editable=           true
                        entry_list_signals= ingredients_signals.clone().unwrap_or_default()
                        entry_type=         RecipeEntryType::Ingredients
                    />

                    // Instructions
                    <EditableEntryList
                        editable=           true
                        entry_list_signals= instructions_signals.clone().unwrap_or_default()
                        entry_type=         RecipeEntryType::Instructions
                    />

                    // Notes
                    <EditableEntryList
                        editable=           true
                        entry_list_signals= notes_signals.clone().unwrap_or_default()
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
fn entries_into_signals<T: RecipeEntry>(entries: Option<Vec<T>>) -> Option<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>> {
    if let Some(entries) = entries {
        let length = entries.len() as u16;
        Some(
            entries
                .into_iter()
                .zip(0..length)
                .map(|(entry, id)| { (id, (create_signal(entry))) })
                .collect()
        )
    } else {
        None
    }
}

fn signals_into_entries<T: RecipeEntry>(signals: Option<Vec<(u16, (ReadSignal<T>, WriteSignal<T>))>>) -> Option<Vec<T>> {
    if let Some(signals) = signals {

        if signals.len() > 0 {
            let entries = signals
                .iter()
                .map(|(_, (get_signal, _))| get_signal.get_untracked())
                .collect();
            Some(entries)

        } else {  None }
    } else { None }
}
