// so app will have 3 screen i: Story, ii: comments for a story, iii: help_popup

use crate::api::{Story, Comment};

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
    stories: Vec<Story>, 
    selected_index: usize,  //story idx currently selected
    loading: bool,       // to show a spinner or smt 
    status: Option<String>,     //for errors or msg loading
    screen: Screen,    // which screen i am on
    feed: Option<Feed>,
    comments: Vec<FlatComment>
}

struct FlatComment{
    comment: Comment,
    depth: usize

}



//now gonna put initiazed vals for these so constructor

impl App{
    fn new(stories: Vec<Story>, selected_index: usize, loading: bool, status: Option<String>, screen: Screen, feed: Option<Feed>) -> Self{
        Self{
            stories,
            selected_index,
            loading,
            status,
            screen,
            feed,
            comments: vec![],
        }
    }
}