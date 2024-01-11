// use windows::{core::*, Foundation::Uri, Web::Syndication::SyndicationClient};
use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::{SyndicationClient, SyndicationFeed},
};

fn get_retrieve_feed_async(uri: Uri) -> Result<SyndicationFeed> {
    let client = SyndicationClient::new()?;
    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"),
    )?;
    client.RetrieveFeedAsync(&uri)?.get()
}

// #[tokio::main]
fn main() -> Result<()> {
    let uri = Uri::CreateUri(h!(
        "https://feeds.feedburner.com/RecentlyRatedAlbums-AlbumOfTheYear"
    ))?;
    let feed = get_retrieve_feed_async(uri)?;
    let items = feed.Items()?;

    for item in items {
        println!("{:?}", item.Title()?.Text()?.to_string_lossy());
    }

    Ok(())
}
