use crossterm::{
    cursor, event, execute, queue,
    style::{Print, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

fn main() -> anyhow::Result<()> {
    let mut out = stdout();

    execute!(out, EnterAlternateScreen)?;
    draw(&mut out)?;

    loop {
        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                event::Event::Key(k) if k.code == event::KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(out, LeaveAlternateScreen)?;
    Ok(())
}

fn draw(out: &mut std::io::Stdout) -> anyhow::Result<()> {
    queue!(out, Clear(ClearType::CurrentLine), Print("Hello, world!"))?;
    out.flush()?;
    Ok(())
}
