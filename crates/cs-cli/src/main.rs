use anyhow::{Result, bail};
use cs_api::ShieldService;
fn main() -> Result<()> {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "example".into());
    let service = ShieldService::new(Default::default())?;
    match cmd.as_str() {
        "example" => println!("{}", serde_json::to_string_pretty(&service.example()?)?),
        "evaluate" => println!("{}", serde_json::to_string_pretty(&service.evaluate()?)?),
        _ => bail!("use example or evaluate"),
    }
    Ok(())
}
