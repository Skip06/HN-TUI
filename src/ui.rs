use ratatui::{Frame,
            widgets::{Block,Borders}
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App){  // this is what gets callwed every frame
    let area = frame.area();
    let widget = Block::new().borders(Borders::ALL);  // literaally from docs . hatss off to docs
    
    frame.render_widget(widget, area);
}
