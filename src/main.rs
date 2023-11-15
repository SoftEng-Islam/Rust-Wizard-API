#[async_std::main]

async fn main() -> tide::Result<()> {
    let app = tide::new();
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
