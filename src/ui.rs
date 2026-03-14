//im using this video https://youtu.be/M-BTpC_BEN0?si=May0iUd2bJT3xn1a
// and docs and gemini to write ratatui (mostly generated)

use ratatui::{Frame,
            widgets::{List, Block, Borders, ListItem},
            layout::{Layout, Constraint, Direction},
            style::{Stylize, Color, Modifier, Style},
            text::{Line, Span}
};
use crate::api::{time_ago, Comment};
use crate::api::HnClient;
use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &mut App){  // this is what gets callwed every frame
    let area = frame.area();
    // let widget = Block::new().borders(Borders::ALL);  // literaally from docs . hatss off to docs
    // frame.render_widget(widget, area);
   
    match app.screen {
        Screen::Story => draw_stories(frame, app),
        Screen::Comments => draw_comments(frame, app),
        
    }
    
  
    
    
}

fn draw_stories(frame: &mut Frame, app: &mut App){
    let chunks = Layout::default() // now chunks is a vector of areas
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1)
        ])
        .split(frame.area());
    
   // List is widget that holds all ListItems . [span->line->ListItem->List]  
   let items: Vec<ListItem> = app.stories
       .iter()
       .skip(app.story_offset) //skips n elems from start
       .take(chunks[1].height as usize) // shows n elems 
       .enumerate()  // gives u (index, story)
       .map(|(i, story)| {
           let is_selected = i == app.selected_story; // is_selected contains bool val
           let title_line = Line::from(vec![
               Span::raw(format!("  {}. ", i + 1)),
               Span::style(story.title.clone().into(), //.into() => converts to req type
                   if is_selected{
                       Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                   }
                   else{
                       Style::default().fg(Color::White)
                   }
               ),
           ]);
           
           let meta_line = Line::from(vec![
               Span::raw(format!(
                   "     {} pts · {} · {}",story.score.unwrap_or(0), story.by.as_deref().unwrap(), time_ago(story.time.unwrap())  
               )),
           ]);
           
           ListItem::new(vec![title_line, meta_line])
       })
       .collect(); //collect() can take anything iterable, and turn it into a relevant collection.
   let list = List::new(items).block(Block::default().fg(Color::Blue).borders(Borders::ALL)); //list is a widget
   frame.render_widget(list, chunks[1]);
    frame.render_widget(Block::new().fg(Color::Red).borders(Borders::ALL), chunks[0]);
    //frame.render_widget(Block::new().fg(Color::Blue).borders(Borders::ALL), chunks[1]);
    frame.render_widget(Block::new().fg(Color::Green).borders(Borders::ALL), chunks[2]);
    
    
    
}


fn draw_comments(frame: &mut Frame, app: &mut App){
    
    //HnClient::fetch_comment(app, kids_id);  {NEVER FETCCH INSIDE A RENDER THING AS RENDER GETS CALLED SAY 60 TIMES PER SEC SO WILL THE FETCH HTTP CALL}
    let items: Vec<ListItem> = app.comments
           .iter()
           .map(|comment| {
               let author = comment.by.as_deref().unwrap();
               let time = time_ago(comment.time.unwrap());
               let text = comment.text.as_deref().unwrap();
               
               let line1 = Line::from(format!("  {} · {}", author, time));
               let line2 = Line::from(format!("  {}", text));
               
               ListItem::new(vec![line1, line2])
           })
           .collect();
       
       let list = List::new(items);
       frame.render_widget(list, frame.area());
}

