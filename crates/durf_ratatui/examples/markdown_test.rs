//! Test the widget with markdown.

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, widgets::Widget};

use durf_ratatui::prelude::*;

const BODY: &'static str = "
    <html>
        <body>
            <h1>Hello</h1>
            <article>
                <h2>World!</h2>
                <p>lorem ipsum...</p>
                <div>
                    <p>This is a <b>really</b> cool section.</p>
                    <p><em>Emphasis</em> mine.</p>
                    <p>Honestly, I may need to do something cool <a href=\"https://hachha.dev\">here</a>, but I'm not sure what <code>I should do.</code></p>
                </div>
            </article>
        </body>
    </html>
";

fn main() -> Result<()> {
    if let Ok(ast) = durf_parser::Ast::from_html(BODY, durf_parser::ParseFlags::default()) {
        println!("AST:\n{}", ast);
    }
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let ast = durf_parser::Ast::from_html(BODY, durf_parser::ParseFlags::default())?;
    let mut state = DurfWidgetState::default();
    let mut style = DurfWidgetStyle::default();
    loop {
        terminal.draw(|frame| {
            let widget = DurfWidget {
                ast: &ast,
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
                    if key.code == KeyCode::Char('k') {
                        state.scroll(-1);
                    }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    event::MouseEventKind::ScrollUp => {
                        state.scroll(-1);
                    }
                    event::MouseEventKind::ScrollDown => {
                        state.scroll(-1);
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
