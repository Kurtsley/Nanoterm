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
use tui_image::{ColorMode, Image};

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();
    let data = get_data();
    let img = image::open("xno-light.png").unwrap().to_rgba8();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Nano Tracker by Kurtsley",
            Style::default().fg(Color::Red).bg(Color::DarkGray),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let block_info = Block::default()
        .title("Info")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);

    let text = vec![
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(format!("Price : $ {:.4}", data.price.to_string())),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(format!(
            "Daily Change: % {:.5}",
            data.percent_change_24h.to_string()
        )),
    ];

    let text_info = Paragraph::new(text)
        .block(block_info)
        .alignment(Alignment::Center);
    f.render_widget(text_info, top_chunks[1]);

    let block_logo = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let logo = Image::with_img(img).block(block_logo);
    f.render_widget(logo, top_chunks[0]);

    let source = Paragraph::new(Span::styled(
        "www.github.com/something/tui_test",
        Style::default().fg(Color::Cyan),
    ))
    .alignment(Alignment::Center);
    f.render_widget(source, chunks[1]);
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
    percent_change_24h: f32,
}

fn get_data() -> Data {
    reqwest::blocking::get("https://www.kurtsley.net")
        .unwrap()
        .json()
        .unwrap()
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    run_app(&mut terminal)?;
    Ok(())
}
