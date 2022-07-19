// Brian Beard

use std::io;

use serde::Deserialize;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use tui_image::Image;

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Save image
    const IMAGE: &[u8] = include_bytes!("../res/xno-light.png");

    let size = f.size();

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
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
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

    // Info blocks
    let block_info_middle = Block::default();
    let block_blank_top = Block::default();
    let block_blank_bottom = Block::default();

    let block_info_main = Block::default()
        .title("Info")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);
    f.render_widget(block_info_main, top_chunks[1]);

    // Info text
    let text = vec![
        Spans::from(""),
        Spans::from(Span::styled(
            format!("Price: ${:.5}", data.price.to_string()),
            Style::default().fg(Color::Yellow),
        )),
        Spans::from(""),
        // Check if positive or negative
        if data.percent_change_1h > 0.0 {
            Spans::from(Span::styled(
                format!("Change 1h: %{:.5}", data.percent_change_1h.to_string()),
                Style::default().fg(Color::Rgb(0, 255, 0)),
            ))
        } else if data.percent_change_1h < 0.0 {
            Spans::from(Span::styled(
                format!("Change 1h: %{:.6}", data.percent_change_1h.to_string()),
                Style::default().fg(Color::Red),
            ))
        } else {
            Spans::from(Span::styled(
                format!("Change 1h: %{:.5}", data.percent_change_1h.to_string()),
                Style::default().fg(Color::White),
            ))
        },
        Spans::from(""),
        if data.percent_change_24h > 0.0 {
            Spans::from(Span::styled(
                format!("Change 24h: %{:.5}", data.percent_change_24h.to_string()),
                Style::default().fg(Color::Rgb(0, 255, 0)),
            ))
        } else if data.percent_change_24h < 0.0 {
            Spans::from(Span::styled(
                format!("Change 24h: %{:.6}", data.percent_change_24h.to_string()),
                Style::default().fg(Color::Red),
            ))
        } else {
            Spans::from(Span::styled(
                format!("Change 24h: %{:.5}", data.percent_change_24h.to_string()),
                Style::default().fg(Color::White),
            ))
        },
    ];

    let text_info = Paragraph::new(text)
        .block(block_info_middle)
        .alignment(Alignment::Center);
    f.render_widget(block_blank_top, chunk_left[0]);
    f.render_widget(text_info, chunk_left[1]);
    f.render_widget(block_blank_bottom, chunk_left[2]);

    // Logo
    let block_logo = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let logo = Image::with_img(img).block(block_logo);
    f.render_widget(logo, top_chunks[0]);

    let credit = Paragraph::new(Span::styled(
        "made with tui-rs - https://github.com/fdehau/tui-rs",
        Style::default().fg(Color::LightCyan),
    ))
    .alignment(Alignment::Center);
    f.render_widget(credit, chunks[1]);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;
        std::thread::sleep(std::time::Duration::from_secs(300));
    }
}

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

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    run_app(&mut terminal)?;
    Ok(())
}
