use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{self, Print},
    terminal,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rspipes::{Args, Pipe};

pub struct App {
    _pipes_count: usize,
    pipes: Vec<Pipe>,
    _fps: usize,
    frame_time: Duration,
    turn_chance: f64,
    char_map: [char; 6],
    terminal_width: u16,
    terminal_height: u16,
    // bg_color: Option<style::Color>
}
impl App {
    fn new(args: Args, rng: &mut ThreadRng) -> Self {
        let size = terminal::size().unwrap();
        App {
            _pipes_count: args.pipes,
            pipes: (0..args.pipes).map(|_| rng.gen()).collect(),
            _fps: args.fps,
            frame_time: Duration::from_secs_f64(1.0 / args.fps as f64),
            turn_chance: args.turn_chance,
            char_map: args.char_map,
            terminal_width: size.0,
            terminal_height: size.1,
            // bg_color: args.bg_color
        }
    }
    fn tick(&mut self, stdout: &mut Stdout, rng: &mut ThreadRng) -> io::Result<()> {
        for pipe in self.pipes.iter_mut() {
            let c = pipe.tick(
                rng,
                self.turn_chance,
                self.terminal_width,
                self.terminal_height,
                self.char_map,
            );
            execute!(
                stdout,
                cursor::SavePosition,
                cursor::MoveTo(pipe.x, pipe.y),
                style::SetForegroundColor(pipe.color),
                Print(c),
                style::ResetColor,
                cursor::RestorePosition
            )?;
        }
        Ok(())
    }
}

fn read_event(timeout: Duration) -> Option<Event> {
    if event::poll(timeout).ok()? {
        event::read().ok()
    } else {
        None
    }
}


fn main() -> io::Result<()> {
    
    let mut rng = thread_rng();
    let args = Args::parse();
    let mut app: App = App::new(args, &mut rng);

    let mut stdout = io::stdout();

    
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide, )?;
    if let Some(color) = /*app.bg_color*/ None {
        execute!(
            stdout,
            style::SetBackgroundColor(color),
            terminal::Clear(terminal::ClearType::All)
        )?;
    }


    loop {
        if let Some(e) = read_event(app.frame_time) {
            match e {
                // C-c
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                })
                // q
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('q') | KeyCode::Esc,
                    ..
                }) 
                => break,
                Event::Resize(x, y) => {
                    app.terminal_width = x;
                    app.terminal_height = y;
                    // execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
                }
                _ => {},
            }
        }
        app.tick(&mut stdout, &mut rng)?;
        stdout.flush()?;
    }

  
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
