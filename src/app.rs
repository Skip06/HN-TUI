// so app will have 3 screen i: Story, ii: comments for a story, iii: help_popup
//Designing for the render loop — thinking about "what does my UI need to read?"

use crate::api::HnClient;
use crate::api::{Comment, Story};
use crate::ui::render;
use futures::future::join_all;
use open;
use std::sync::Arc;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use std::io::stdout;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    //these vals will be compared and passed along FNs so will impl some derive traits
    Story,
    Comments,
}

//so there will be some feeds like they have on the website
#[derive(Debug, Clone, PartialEq)]
pub enum Feed {
    Ask,
    New,
    Show,
    Top,
}

impl Feed {
    pub fn label(&self) -> &str {
        match self {
            Feed::Top => "Top",
            Feed::New => "New",
            Feed::Ask => "Ask",
            Feed::Show => "Show",
        }
    }
}

//Every field in App answers one question:
//"If I close the app and reopen it mid-session, what do I need to restore exactly where the user was?"
// App is single owner of everything ur application needs
#[derive(Debug, Clone)]
pub struct App {
    pub client: Arc<HnClient>, //shared ownership as this app runs forever say by main loop but fetching happens in background async task
    pub stories: Vec<Story>,
    pub screen: Screen, // which screen i am on
    pub feed: Feed,
    pub comments: Vec<Comment>,
    pub selected_comment: usize,
    pub selected_story: usize,
    pub story_offset: usize, // these are needed for scrolling
    pub comment_offset: usize,
    pub page_size: usize,
}

//now gonna put initialized vals for these so constructor

impl App {
    pub fn new() -> Self {
        Self {
            client: Arc::new(HnClient::new()),
            stories: vec![],
            screen: Screen::Story,
            feed: Feed::Top,
            comments: vec![],
            selected_comment: 0,
            selected_story: 0,
            story_offset: 0,
            comment_offset: 0,
            page_size: 30,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let top_ids = self.client.fetch_top_stories().await?; // its a async funtion

        // current fetch is sequential — fetching story 1, waiting, fetching story 2, 30 sequential HTTP requests is slow.
        //The fix is concurrent fetching — fire all requests at the same time and wait for all of them together. This is what futures::join_all does.

        // for id in &top_ids[..25] {  // slice so borrowing
        //     // we are borrowing then iterating
        //     let story = self.client.fetch_story(*id).await?; // as id is currently &i64
        //     self.stories.push(story);

        // }

        let futures: Vec<_> = top_ids
            .iter()
            .take(30)
            .map(|&id| self.client.fetch_story(id.try_into().unwrap())) // try_into() performs the conversion to req type and returns a result.
            .collect();

        let results: Vec<Result<Story, Box<dyn std::error::Error>>> = join_all(futures).await; //Example result:Ok(story1),Ok(story2),Err(network_error), Ok(story4)]

        self.stories = results.into_iter().filter_map(|r| r.ok()).collect(); // r.ok() => Result to Option => Ok(story1); → Some(story1)Err(error) → None
        // filter_map removes NOne//

        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        //creating terminal with crossterm Backend
        let backend = CrosstermBackend::new(stdout()); // stdout and stderr are the writer o/p stream
        let mut terminal = Terminal::new(backend)?;

        // event loop runs forever until manually broken
        let result = loop {
            // terminal.draw takes a closure (an anonymous function).
            // It passes a 'frame' into that function so we can draw on it.
            // We wrap render in a closure so we can also pass app into it.
            terminal.draw(|frame| render(frame, self))?;

            // event::read() blocks until the user does something (key press, mouse, resize etc.)
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break Ok(());
                    }
                    KeyCode::Down => {
                        let max = self.stories.len().saturating_sub(1);
                        if self.selected_story < max {
                            self.selected_story += 1;
                            if self.selected_story >= self.story_offset + self.page_size {
                                self.story_offset += 1
                            } // the scrolling thing look up the defination of offset
                        }
                    }
                    KeyCode::Up => {
                        if self.selected_story > 0 {
                            self.selected_story -= 1;
                            if self.selected_story < self.story_offset {
                                self.story_offset = self.story_offset.saturating_sub(1);
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let kid_ids = self.stories[self.selected_story].kids.clone().unwrap(); // clone() cause ownership violets

                        //will clear the app.comment
                        self.comments.clear();

                        //   doin the same thing for comments //

                        let futures: Vec<_> = kid_ids
                            .iter()
                            .take(10)
                            .map(|&id| self.client.fetch_comment(id)) //patern match => id is the val and not a ref
                            .collect();

                        let results: Vec<Result<Comment, Box<dyn std::error::Error>>> =
                            join_all(futures).await; //Example result:Ok(story1),Ok(story2),Err(network_error), Ok(story4)]

                        self.comments = results
                            .into_iter()
                            .filter_map(|r: Result<Comment, Box<dyn std::error::Error>>| r.ok())
                            .collect::<Vec<Comment>>(); // r.ok() => Result to Option => Ok(story1); → Some(story1)Err(error) → None

                        self.screen = Screen::Comments;
                    }
                    KeyCode::Esc => {
                        self.screen = Screen::Story;
                    }
                    KeyCode::Char('o') => {
                        if let Some(story) = self.stories.get(self.selected_story) {
                            if let Some(url) = &story.url {
                                let _ = open::that(url); // open the url in default browser
                            }
                        }
                    }
                    _ => {} // handles keycode for the rest
                }
            }
        };

        // always clean up terminal before returning, even on error
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;

        result
    }
}
