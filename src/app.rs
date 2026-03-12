// so app will have 3 screen i: Story, ii: comments for a story, iii: help_popup
//Designing for the render loop — thinking about "what does my UI need to read?"

use crate::api::{Story, Comment};
use crate::api::HnClient;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen{      //these vals will be compared and passed along FNs so will impl some derive traits
    Story,
    Comments,
    Help
}

//so there will be some feeds like they have on the website
#[derive(Debug, Clone, PartialEq)]
pub enum Feed{
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

//Every field in App answers one question:
//"If I close the app and reopen it mid-session, what do I need to restore exactly where the user was?"
// App is single owner of everything ur application needs
#[derive(Debug, Clone)]
pub struct App{
    pub client: Arc<HnClient>, //shared ownership as this app runs forever say by main loop but fetching happens in background async task  
    pub stories: Vec<Story>, 
    pub loading: bool,       // to show a spinner or smt 
    pub status: String,     //for errors or msg loading
    pub screen: Screen,    // which screen i am on
    pub feed: Feed,
    pub comments: Vec<FlatComment>,
    pub selected_comment: usize,
    pub selected_story: usize,
    pub story_offset: usize, // these are needed for srolling . refer pic proj 
    pub comment_offset: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone)]
pub struct FlatComment{
    comment: Comment,
    depth: usize

}



//now gonna put initiazed vals for these so constructor

impl App{
    pub fn new() -> Self{
        Self{
           client: Arc::new(HnClient::new()),
            stories:vec![],
            loading:false,
            status:String::from("Loading"),
            screen:Screen::Story,
            feed: Feed::Top,
            comments: vec![],
            selected_comment: 0,
            selected_story: 0,
            story_offset: 0,
            comment_offset: 0,
            page_size: 30,
        }
    }
}