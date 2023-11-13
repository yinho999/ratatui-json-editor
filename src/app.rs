use std::collections::HashMap;
use color_eyre::eyre::Result;
/// Three possible screens
pub enum CurrentScreen {
    /// The main screen
    Main,
    /// The edit screen
    Edit,
    /// The exit screen
    Exit,
}

/// Keep track of which field the user is editing
pub enum CurrentlyEditing{
    /// Key field
    Key,
    /// Value field
    Value,
}

/// The full application state
pub struct App{
    pub key_input: String, // currently being edited json key
    pub value_input: String, // currently being edited json value
    pub pairs:HashMap<String,String>, // The representation of our key and value pairs with serde Serialize and Deserialize support
    pub current_screen: CurrentScreen, // The current screen the user is looking at, will later determine which UI to render
    pub currently_editing: Option<CurrentlyEditing>, // The optional state containing which key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}


impl App{
    /// Create a new instance of the application state [`App`]
    pub fn new()->App{
        App{
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    /// Save key value pair to the hashmap [`App::pairs`] and reset the input fields [`App::key_input`] and [`App::value_input`], also reset the currently editing state [`App::currently_editing`]
    pub fn save_key_value(&mut self ){
        self.pairs.insert(self.key_input.clone(),self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    /// Toggle editing state [`App::currently_editing`] to [`CurrentlyEditing::Key`] or [`CurrentlyEditing::Value`] depending on the current state. Swap between the two states.
    pub fn toggle_editing(&mut self ){
        if let Some(edit_mode) = &self.currently_editing{
            match edit_mode{
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value);
                },
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key);
                },
            }
        }else{
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    /// Print json string representation of the hashmap [`App::pairs`] to the terminal
    pub fn print_json(&self) -> Result<()>{
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}",output);
        Ok(())
    }
}