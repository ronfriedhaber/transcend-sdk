use mpera_frontend::program::Program;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::new();
    let program = program.dataframe(None)?.alias("row_count", None)?;

    let client = common::client_from_env_default()?;
    let response = client
        .upload_program(&program, Some("transcend-sdk-example-program".to_string()))
        .await?;

    println!("{response:#?}");

    Ok(())
}
