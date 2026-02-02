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
                <div>
                    <h1>HTML Ipsum Presents</h1>

    				<p><strong>Pellentesque habitant morbi tristique</strong> senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. <em>Aenean ultricies mi vitae est.</em> Mauris placerat eleifend leo. Quisque sit amet est et sapien ullamcorper pharetra. Vestibulum erat wisi, condimentum sed, <code>commodo vitae</code>, ornare sit amet, wisi. Aenean fermentum, elit eget tincidunt condimentum, eros ipsum rutrum orci, sagittis tempus lacus enim ac dui. <a href=\"#\">Donec non enim</a> in turpis pulvinar facilisis. Ut felis.</p>

    				<h2>Header Level 2</h2>

    				<ol>
    				   <li><a href=\"https://hachha.dev/blog/jdpub\">Lorem ipsum</a> dolor sit amet, consectetuer adipiscing elit.</li>
    				   <li>Aliquam tincidunt mauris eu risus.</li>
    				</ol>

    				<blockquote><p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus magna. Cras in mi at felis aliquet congue. Ut a est eget ligula molestie gravida. Curabitur massa. Donec eleifend, libero at sagittis mollis, tellus est malesuada tellus, at luctus turpis elit sit amet quam. Vivamus pretium ornare est.</p></blockquote>

    				<h3>Header Level 3</h3>

    				<ul>
    				   <li>Lorem ipsum dolor sit amet, consectetuer adipiscing elit.</li>
    				   <li>Aliquam tincidunt mauris eu risus.</li>
    				</ul>

    				<pre><code>
    				#header h1 a {
    				  display: block;
    				  width: 300px;
    				  height: 80px;
    				}
    				</code></pre>
                </div>

                <div>
                    <p>Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo. Quisque sit amet est et sapien ullamcorper pharetra. Vestibulum erat wisi, condimentum sed, commodo vitae, ornare sit amet, wisi. Aenean fermentum, elit eget tincidunt condimentum, eros ipsum rutrum orci, sagittis tempus lacus enim ac dui. Donec non enim in turpis pulvinar facilisis. Ut felis. Praesent dapibus, neque id cursus faucibus, tortor neque egestas augue, eu vulputate magna eros eu erat. Aliquam erat volutpat. <a href=\"https://hachha.dev/blog/pachisim\">Nam dui mi, tincidunt quis, accumsan porttitor, facilisis luctus</a>, metus</p>
                </div>
                <div>
    				<ul>
    				   <li>Morbi in sem quis dui placerat ornare. Pellentesque odio nisi, euismod in, pharetra a, ultricies in, diam. Sed arcu. Cras consequat.</li>
    				   <li>Praesent dapibus, neque id cursus faucibus, tortor neque egestas augue, eu vulputate magna eros eu erat. Aliquam erat volutpat. Nam dui mi, tincidunt quis, accumsan porttitor, facilisis luctus, metus.</li>
    				   <li>Phasellus ultrices nulla quis nibh. Quisque a lectus. Donec consectetuer ligula vulputate sem tristique cursus. Nam nulla quam, gravida non, commodo a, sodales sit amet, nisi.</li>
    				   <li>Pellentesque fermentum dolor. Aliquam quam lectus, facilisis auctor, ultrices ut, elementum vulputate, nunc.</li>
    				</ul>
                </div>
                <div>
                    <p>Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo. Quisque sit amet est et sapien ullamcorper pharetra. Vestibulum erat wisi, condimentum sed, commodo vitae, ornare sit amet, wisi. Aenean fermentum, elit eget tincidunt condimentum, eros ipsum rutrum orci, sagittis tempus lacus enim ac dui. Donec non enim in turpis pulvinar facilisis. Ut felis. Praesent dapibus, neque id cursus faucibus, tortor neque egestas augue, eu vulputate magna eros eu erat. Aliquam erat volutpat. Nam dui mi, tincidunt quis, accumsan porttitor, facilisis luctus, metus</p>
                </div>
                <div>
                    <p>Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo. Quisque sit amet est et sapien ullamcorper pharetra. Vestibulum erat wisi, condimentum sed, commodo vitae, ornare sit amet, wisi. Aenean fermentum, elit eget tincidunt condimentum, eros ipsum rutrum orci, sagittis tempus lacus enim ac dui. Donec non enim in turpis pulvinar facilisis. Ut felis. Praesent dapibus, neque id cursus faucibus, tortor neque egestas augue, eu vulputate magna eros eu erat. Aliquam erat volutpat. Nam dui mi, tincidunt quis, accumsan porttitor, facilisis luctus, metus</p>
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
    let _mc = durf_ratatui::prelude::util::MouseCapture::new()?;
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
                                ast: &ast,
                                state: &mut state,
                                style: &mut style,
                            };
                            widget.handle_click(Position::new(mouse.column, mouse.row));
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
