// use windows::{core::*, Foundation::Uri, Web::Syndication::SyndicationClient};
use windows::{core::*, Foundation::Uri, Web::Syndication::SyndicationClient};

fn main() -> Result<()> {
    println!("Hello, world!");
    let uri = Uri::CreateUri(h!(
        "https://feeds.feedburner.com/RecentlyRatedAlbums-AlbumOfTheYear"
    ))?;
    let client = SyndicationClient::new()?;
    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"),
    )?;
    let _feed = client.RetrieveFeedAsync(&uri)?.get();
    Ok(())
}
