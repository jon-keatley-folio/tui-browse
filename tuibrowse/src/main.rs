use std::env;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    layout::Rect,
    text::Text,
    DefaultTerminal,
    Frame,
    widgets::Clear,
};

pub struct TUIBrowseApp{
    exit:bool,
    file:String,
    
}

impl TUIBrowseApp
{
    fn new(path:&str) -> Result<TUIBrowseApp, String>
    {
        Ok(
            TUIBrowseApp {
                exit:false,
                file: path.to_string()
            }
        )
    }
    
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(200)).is_ok()
        {
            match event::read()?
            {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            };
        }
        
        Ok(())
    }
    
    fn handle_key_event(&mut self, key_event: KeyEvent)
    {
        match key_event.code {
            KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }
    
    fn draw(&self, frame: &mut Frame) {
        let text = Text::raw("Hello World!");
        frame.render_widget(text, frame.area());
    }
}


fn main(){
    
    let args:Vec<String> =  env::args().skip(1).collect();
    if args.len() < 1 || args.len() > 2
    {
         let unexpected_usage:&str = "Unexpected usage! Requires Path[fasta path]";
        println!("{}",unexpected_usage);
        return;
    }
    
    let app_result  = TUIBrowseApp::new(&args[0]);
    
    match app_result
    {
        Ok(mut app) =>
        {
            let mut terminal = ratatui::init();
            let _ = app.run(&mut terminal);
            ratatui::restore();
            
        },
        Err(err) =>
        {
            println!("ERROR! {}",err );
        }
    }
}