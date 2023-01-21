// Brian Beard

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use tui_image::Image;

#[derive(Deserialize)]
struct Data {
    price: f32,
    percent_change_1h: f32,
    percent_change_24h: f32,
}

// Query kurtsley.net and return data or error.
fn get_data() -> Result<Data, reqwest::Error> {
    let url = "https://www.kurtsley.net";
    let res = reqwest::blocking::get(url)?.json::<Data>()?;
    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let tick_rate = Duration::from_secs(5);
    let res = run_app(&mut terminal, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

// Center the paragraph vertically in a block
fn center_vert(height: u16, lines: u16) {}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Save image
    const IMAGE: &[u8] = include_bytes!("../res/xno-light.png");

    let size = f.size();

    let term_height = size.bottom();

    let data = get_data();

    let data = match data {
        Ok(data) => data,
        Err(e) => panic!("Couldn't access kurtsley.net: {:?}", e),
    };

    let img = image::load_from_memory(IMAGE)
        .expect("Couldn't load image")
        .into_rgba8();

    // Main block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Nanoterm by Kurtsley",
            Style::default().fg(Color::Blue),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    // Vertical chunk split
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(88), Constraint::Percentage(12)].as_ref())
        .split(f.size());

    // Top chunks
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Info chunk vertical split
    let chunk_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(top_chunks[1]);

    // Info block
    let block_info_middle = Block::default();

    // Info block main
    let block_info_main = Block::default()
        .title("Info")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);

    let info_size = block_info_main.inner(top_chunks[1]);

    f.render_widget(block_info_main, top_chunks[1]);

    // If-else statement that determines the size of the block_info_main area
    // and adds extra Spans::from("") to add space to the beginning of the vec.
    // TODO Simplify this in the future!!!
    let text = if term_height > 54 {
        vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else if term_height > 44 && term_height <= 54 {
        vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else if term_height > 33 && term_height <= 44 {
        vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else if term_height > 26 && term_height <= 33 {
        vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else if term_height >= 26 && term_height <= 30 {
        // Info text
        vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else if term_height > 23 && term_height <= 26 {
        // Info text
        vec![
            Spans::from(""),
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else if term_height >= 20 && term_height <= 23 {
        // Info text
        vec![
            Spans::from(Span::styled(
                format!("Price: ${:.5}", data.price.to_string()),
                Style::default().fg(Color::Yellow),
            )),
            Spans::from(""),
            // Check if positive or negative
            if data.percent_change_1h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % +{:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_1h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.6}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 1h: % {:.5}", data.percent_change_1h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
            Spans::from(""),
            if data.percent_change_24h > 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % +{:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Rgb(0, 255, 0)),
                ))
            } else if data.percent_change_24h < 0.0 {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.6}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Spans::from(Span::styled(
                    format!("Change 24h: % {:.5}", data.percent_change_24h.to_string()),
                    Style::default().fg(Color::White),
                ))
            },
        ]
    } else {
        vec![Spans::from("Terminal too small!")]
    };

    let text_info = Paragraph::new(text)
        .block(block_info_middle)
        .alignment(Alignment::Center);
    f.render_widget(text_info, chunk_left[1]);

    // Logo
    let block_logo = Block::default()
        .title(format!("height: {:?}", info_size.height))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let logo = Image::with_img(img).block(block_logo);

    f.render_widget(logo, top_chunks[0]);

    let source_text = vec![
        Spans::from("press 'q' to exit"),
        Spans::from(""),
        Spans::from(Span::styled(
            "made with tui-rs - https://github.com/fdehau/tui-rs",
            Style::default().fg(Color::LightCyan),
        )),
    ];

    let source = Paragraph::new(source_text).alignment(Alignment::Center);
    f.render_widget(source, chunks[1]);
}
