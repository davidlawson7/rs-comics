use clap::Parser;
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
  io,
  time::{Duration, Instant},
};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::{Constraint, Direction, Layout},
  widgets::{Block, Borders, Widget},
  Frame, Terminal,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// The refresh rate for the ui in ms
  #[clap(short, long, default_value_t = 200)]
  tick_rate: u64,
}

struct Comic {
  name: String,
  src: String,
  main_page_raw: String,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
  let args = Args::parse();

  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let tick_rate = Duration::from_millis(args.tick_rate);
  let res = run_app(&mut terminal, tick_rate);

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  if let Err(err) = res {
    println!("{:?}", err)
  }

  Ok(())
}

fn run_app<B: Backend>(
  terminal: &mut Terminal<B>,
  // mut app: App,
  tick_rate: Duration,
) -> io::Result<()> {
  let mut last_tick = Instant::now();
  loop {
    terminal.draw(|f| ui(f))?;

    let timeout = tick_rate
      .checked_sub(last_tick.elapsed())
      .unwrap_or_else(|| Duration::from_secs(0));
    if crossterm::event::poll(timeout)? {
      if let Event::Key(key) = event::read()? {
        match key.code {
          KeyCode::Char('q') => return Ok(()),
          // KeyCode::Left => app.items.unselect(),
          // KeyCode::Down => app.items.next(),
          // KeyCode::Up => app.items.previous(),
          _ => {}
        }
      }
    }
    if last_tick.elapsed() >= tick_rate {
      // app.on_tick();
      last_tick = Instant::now();
    }
  }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
      [
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
      ]
      .as_ref(),
    )
    .split(f.size());
  let block = Block::default().title("Block").borders(Borders::ALL);
  f.render_widget(block, chunks[0]);
  let block = Block::default().title("Block 2").borders(Borders::ALL);
  f.render_widget(block, chunks[1]);
}
