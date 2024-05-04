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
    fn create_inputs_node_refs() -> Vec<NodeRef<Input>>;
    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self;
    fn into_editable_view(entry: ReadSignal<Self>, nodes_refs: Vec<NodeRef<Input>>) -> View;
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe);
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

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        vec![create_node_ref::<Input>()]
    }

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {
        if nodes_refs.len() != 1 { panic!("NodeRefs number is not matching !") }
        RecipeTag {
            name: nodes_refs[0].get().expect("<input> to exist").value(),
        }
    }

    fn into_editable_view(entry: ReadSignal<Self>, nodes_refs: Vec<NodeRef<Input>>) -> View {
        view! {
            <RecipeEntryInput
                elem_ref=nodes_refs[0]
                placeholder="Instruction content...".to_owned()
                input=entry.get().name
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<RecipeTag>>, recipe: &mut Recipe) {
        recipe.tags = entries;
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

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        (0..3)
            .map(|_| create_node_ref::<Input>())
            .collect()
    }

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

    fn into_editable_view(entry: ReadSignal<Self>, nodes_refs: Vec<NodeRef<Input>>) -> View {
        let entry = entry.get();
        view! {
            <RecipeEntryInput
                elem_ref=nodes_refs[0]
                input=entry.quantity.to_string()
                placeholder="Quantity...".to_owned()
            />
            <RecipeEntryInput
                input=entry.unit
                placeholder="Unit...".to_owned()
                elem_ref=nodes_refs[1]
            />
            <RecipeEntryInput
                input=entry.content
                placeholder="Ingredient Content...".to_owned()
                elem_ref=nodes_refs[2]
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe) {
        recipe.ingredients = entries;
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

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        vec![create_node_ref::<Input>()]
    }

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {
        if nodes_refs.len() != 1 { panic!("NodeRefs number is not matching !") }
         
        RecipeInstruction {
            content: nodes_refs[0].get().expect("<input> to exist").value()
        }
    }
    
    fn into_editable_view(entry: ReadSignal<Self>, nodes_refs: Vec<NodeRef<Input>>) -> View {
        view! {
            <RecipeEntryInput
                elem_ref=nodes_refs[0]
                input=entry.get().content
                placeholder="Instruction content...".to_owned()
            />
        }
        .into_view()
    }
    
    fn update_recipe(entries: Option<Vec<Self>>, recipe: &mut Recipe) {
        recipe.instructions = entries;
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

    fn create_inputs_node_refs() -> Vec<NodeRef<Input>> {
        (0..2)
            .map(|_| create_node_ref::<Input>())
            .collect()
    }

    fn extract_value(nodes_refs: Vec<NodeRef<Input>>) -> Self {

        if nodes_refs.len() != 2 { panic!("NodeRefs number is not matching !") }
         
        RecipeNote {
            title:      nodes_refs[0].get().expect("<input> to exist").value(),
            content:    nodes_refs[1].get().expect("<input> to exist").value(),
        }
    }
    
    fn into_editable_view(entry: ReadSignal<Self>, nodes_refs: Vec<NodeRef<Input>>) -> View {
        let entry = entry.get();
        view! {
            <RecipeEntryInput
                elem_ref=nodes_refs[0]
                input=entry.title
                placeholder="Note Title...".to_owned()
            />
            <RecipeEntryInput
                elem_ref=nodes_refs[1]
                input=entry.content
                placeholder="Note Content...".to_owned()
            />
        }.into_view()
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
                name=recipe.name
            />

            // Tags
            <EntryList
                entry_list=recipe.tags
                entry_type=RecipeEntryType::Tag
            />

            // Ingredients
            <EntryList
                entry_list=recipe.ingredients
                entry_type=RecipeEntryType::Ingredients
            />

            // Instructions
            <EntryList
                entry_list=recipe.instructions
                entry_type=RecipeEntryType::Instructions
            />

            // Notes
            <EntryList
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
    is_new_recipe: Option<bool>,
    recipe_action: Action<(Recipe, RecipeAction), Result<(), ServerFnError>>,
) -> impl IntoView {

    let editable = true;

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
            },
            Err(e) => {
                log!("{}", e);
            },
        }
    };

    view! {
        <div class="editable-recipe" >

            // Name
            <EditableRecipeName
                name=recipe.name
                recipe_setter=recipe_setter.clone()
                editable=true
            />

            // Tags
            <EditableEntryList
                entry_list=recipe.tags
                entry_type=RecipeEntryType::Tag
                recipe_setter=recipe_setter.clone()
                editable=true
            />

            // Ingredients
            <EditableEntryList
                entry_list=recipe.ingredients
                entry_type=RecipeEntryType::Ingredients
                recipe_setter=recipe_setter.clone()
                editable=true
            />

            // Instructions
            <EditableEntryList
                entry_list=recipe.instructions
                entry_type=RecipeEntryType::Instructions
                recipe_setter=recipe_setter.clone()
                editable=true
            />

            // Notes
            <EditableEntryList
                entry_list=recipe.notes
                entry_type=RecipeEntryType::Notes
                recipe_setter=recipe_setter.clone()
                editable=true
            />

            {
                if editable {

                    Some(view! {
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
                            <p>"wait for save"</p>
                        </Show>
                    })

                } else if is_new_recipe {

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
