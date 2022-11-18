use nml::{cli::Cli, fetch::Result, run};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Cli::build();
    let save_path = client.save_path;
    let search_names = client.search_names;
    for search_name in search_names {
        run(search_name, save_path.clone()).await?;
    }
    Ok(())
}
