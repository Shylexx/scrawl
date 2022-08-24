mod app;
mod input;
mod todo;

use std::error::Error;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {

    // Sets up terminal
    let (mut app, mut terminal) = App::new("Scrawl").unwrap();

    // Render Loop
    app.run(&mut terminal)?;

    // Cleanup App
    app.cleanup(&mut terminal)?;

    Ok(())
}
