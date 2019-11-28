use async_std::task;
use swagger_petstore::client::*;

fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;

    task::block_on(async {
        let _client = SwaggerPetstoreClient::new("https://service");
        Ok(())
    })
}
