#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&hourly=temperature_2m";

    let response = reqwest::get(URL).await?;
    let weather: serde_json::Value = response.json().await?;
    println!("{weather:#?}");

    return Ok(());
}
