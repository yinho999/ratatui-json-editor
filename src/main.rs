use color_eyre::eyre::Result;
use crossterm::execute;

fn main() ->Result<()> {
    // setup color_eyre
    color_eyre::install()?;
    // setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(
        stderr,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide,
        crossterm::event::EnableMouseCapture,
    )?;
    println!("Hello, world!");
    Ok(())
}
