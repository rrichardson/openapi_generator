use async_std::task;
use {{snakecase info.title}}::client::*;

fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;

    task::block_on(async {
        let client = {{camelcase info.title "Client"}}::new("https://service");
        Ok(())
    })
}
