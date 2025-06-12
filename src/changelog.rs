use std::io::{self, Write};
use crossterm::{
  execute, queue,
  style, cursor, terminal
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Page {
  version: String,
  body: String,
}

fn load_changelog_data() -> Vec<Page> {
  // Include the JSON file at compile time
  let data = include_str!(env!("CHANGELOGPATH"));
  
  // Parse JSON into our structure
  match serde_json::from_str(data) {
      Ok(pages) => pages,
      Err(e) => {
          eprintln!("Error parsing changelog data: {}", e);
          vec![] // Return empty vector if there's an error
      }
  }
}

pub fn changelog() -> io::Result<()> {
  let mut stdout = io::stdout();
  
  // Load changelog data
  let pages = load_changelog_data();
  
  if pages.is_empty() {
      println!("No changelog data available.");
      return Ok(());
  }

  // Enter alternate screen and enable raw mode
  terminal::enable_raw_mode()?;
  execute!(stdout, terminal::EnterAlternateScreen)?;

  let mut current_page = 0;
  let total_pages = pages.len();

  loop {
      // Clear screen and display current page
      queue!(
          stdout,
          terminal::Clear(terminal::ClearType::All),
          cursor::MoveTo(0, 0),
          style::Print(format!("Changelog ({}/{})\n\n", current_page + 1, total_pages)),
          style::Print(format!("Version: {}\n\n", pages[current_page].version)),
          style::Print(&pages[current_page].body),
          cursor::MoveTo(0, terminal::size()?.1 - 1),
          style::Print("← → or A/D to navigate, Q to quit")
      )?;
      stdout.flush()?;

      // Wait for key input
      match crossterm::event::read()? {
          crossterm::event::Event::Key(key_event) => {
              // Only respond to key press events, not release events
              if key_event.kind == crossterm::event::KeyEventKind::Press {
                  match key_event.code {
                      crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('a') => {
                          if current_page > 0 {
                              current_page -= 1;
                          }
                      },
                      crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('d') => {
                          if current_page < total_pages - 1 {
                              current_page += 1;
                          }
                      },
                      crossterm::event::KeyCode::Char('q') => break,
                      _ => {}
                  }
              }
          },
          _ => {}
      }
  }

  // Restore terminal state
  execute!(stdout, terminal::LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;
  Ok(())
}