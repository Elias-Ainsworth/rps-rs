use rps_rs::Mode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = Mode::prompt_mode()?;
    mode.play_rounds()?;
    Ok(())
}
