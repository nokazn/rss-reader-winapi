use encoding_rs::*;
use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::{SyndicationClient, SyndicationFeed},
    Win32::UI::WindowsAndMessaging::*,
};

fn get_retrieve_feed_async(uri: Uri) -> Result<SyndicationFeed> {
    let client = SyndicationClient::new()?;
    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"),
    )?;
    client.RetrieveFeedAsync(&uri)?.get()
}

fn main() -> Result<()> {
    let uri = Uri::CreateUri(h!(
        "https://feeds.feedburner.com/RecentlyRatedAlbums-AlbumOfTheYear"
    ))?;
    let feed = get_retrieve_feed_async(uri)?;
    let items = feed.Items()?;
    let items = items
        .into_iter()
        .filter_map(|item| {
            item.Title()
                .and_then(|t| t.Text())
                .map(|t| format!(". {}", t.to_string_lossy()))
                .ok()
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut text = SHIFT_JIS.encode(&items).0.to_vec();
    // null終端にする
    text.push(0);

    unsafe {
        MessageBoxA(None, PCSTR(text.as_ptr()), s!("Feed of AOTY"), MB_OK);
    }

    Ok(())
}
