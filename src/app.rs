// so app will have 3 screen i: Story, ii: comments for a story, iii: help_popup

use crate::Story;

#[derive(Debug, Clone, PartialEq)]
enum Screen{      //these vals will be compared and passed along FNs so will impl some derive traits
    Story,
    Comments,
    Help
}

//so there will be some feeds like they have on the website
enum Feed{
    Ask,
    New,
    Show,
    Top,
    Best
}

impl Feed{
    pub fn label(&self) -> &str{
        match self {
            Feed::Top => "Top",
            Feed::New => "New",
            Feed::Best => "Best",
            Feed::Ask => "Ask",
            Feed::Show => "Show",
        }
    }
}
struct App{
    Stories: Vec<Story>, 
    Selected_index: usize,  //story idx currently selected
    Loading: bool,       // to show a spinner or smt 
    Status: String,     //for errors or msg loading
    Screen: Screen,    // which screen i am on
    Feed: String,


}
//now gonna put initiazed vals for these so constructor

impl App{
    fn new( Stories: Vec<Story>, Selected_index: usize,Loading: bool,Status: String, Screen: Screen,Feed: String) -> Self{
        Self{
            Stories: vec![],
            Selected_index: 0,
            Loading: false,
            Status: "None".to_string(),
            Screen: Screen::Story,
            Feed: "Top".to_string(),
        }
    }
}