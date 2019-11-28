use async_std::task;
use vizyr_api::client::*;

fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;

    task::block_on(async {
        let _client = VizyrApiClient::new("https://service");
        Ok(())
    })
}
