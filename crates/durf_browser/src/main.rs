//! Durf browser.

use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, widgets::Widget};

use durf_ratatui::prelude::*;

#[derive(clap::Parser, Debug)]
struct Cli {
    /// URI to open.
    site: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Durf browser.");

    let cli = Cli::parse();
    let terminal = ratatui::init();
    let _mc = durf_ratatui::prelude::util::MouseCapture::new()?;
    let result = run(terminal, &cli.site).await;
    ratatui::restore();

    result
}

async fn run(mut terminal: DefaultTerminal, uri: &str) -> Result<()> {
    let mut current_doc: String = uri.into();
    let mut engine = durf_engine::Engine::new().await?;
    // let ast = durf_parser::Ast::from_html(BODY, durf_parser::ParseFlags::default())?;
    let mut state = DurfWidgetState::default();
    let mut style = DurfWidgetStyle::default();
    loop {
        let doc = engine.load(&current_doc).await?;

        terminal.draw(|frame| {
            let widget = DurfWidget {
                ast: doc.ast(),
                state: &mut state,
                style: &mut style,
            };
            widget.render(frame.area(), frame.buffer_mut());
        })?;
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        break Ok(());
                    }
                    if key.code == KeyCode::Down {
                        state.scroll(1);
                    }
                    if key.code == KeyCode::Up {
                        state.scroll(-1);
                    }
                    if key.code == KeyCode::Char('j') {
                        state.scroll(1);
                    }
                    if key.code == KeyCode::Char('J') {
                        state.scroll(4);
                    }
                    if key.code == KeyCode::Char('k') {
                        state.scroll(-1);
                    }
                    if key.code == KeyCode::Char('K') {
                        state.scroll(-4);
                    }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    event::MouseEventKind::ScrollUp => {
                        state.scroll(-4);
                    }
                    event::MouseEventKind::ScrollDown => {
                        state.scroll(4);
                    }
                    event::MouseEventKind::Down(button) => {
                        if button.is_left() {
                            let mut widget = DurfWidget {
                                ast: doc.ast(),
                                state: &mut state,
                                style: &mut style,
                            };
                            if let Some(event) =
                                widget.handle_click(Position::new(mouse.column, mouse.row))
                            {
                                match event {
                                    DurfEvent::FollowLink(link) => {
                                        current_doc = link;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Event::Resize(_cols, _rows) => {
                    // todo!()
                }
                _ => {}
            }
        }
    }
}
