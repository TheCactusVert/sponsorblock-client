mod action;
mod category;
mod segment;

pub use action::Action;
pub use category::Category;
use segment::Videos;
pub use segment::{Segment, Segments};

use reqwest::{Client, Result, Url};
use sha2::{Digest, Sha256};

static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0";

pub const DEFAULT_SERVER_ADDRESS: &str = "https://sponsor.ajay.app";

pub struct SponsorBlock {
    client: Client,
    server_address: Url,
    private_api: bool,
}

impl SponsorBlock {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::builder().user_agent(USER_AGENT).build()?,
            server_address: Url::parse(DEFAULT_SERVER_ADDRESS).unwrap(),
            private_api: false,
        })
    }

    pub fn set_server_address(&mut self, server_address: Url) {
        self.server_address = server_address;
    }

    pub fn set_private_api(&mut self, private_api: bool) {
        self.private_api = private_api;
    }

    pub async fn fetch<C, A>(
        &self,
        id: String,
        categories: C,
        action_types: A,
    ) -> Result<Segments>
    where
        C: IntoIterator<Item = Category>,
        A: IntoIterator<Item = Action>,
    {
        if self.private_api {
            self.fetch_with_privacy(id, categories, action_types).await
        } else {
            self.fetch_default(id, categories, action_types).await
        }
    }

    async fn fetch_default<C, A>(
        &self,
        id: String,
        categories: C,
        action_types: A,
    ) -> Result<Segments>
    where
        C: IntoIterator<Item = Category>,
        A: IntoIterator<Item = Action>,
    {
        let mut url = self.server_address.join("/api/skipSegments").unwrap();

        url.query_pairs_mut()
            .append_pair("videoID", &id)
            .extend_pairs(categories.into_iter().map(|v| ("category", v)))
            .extend_pairs(action_types.into_iter().map(|v| ("actionType", v)));

        let req = self.client.get(url).send().await?.error_for_status()?;

        Ok(req.json::<Segments>().await?)
    }

    async fn fetch_with_privacy<C, A>(
        &self,
        id: String,
        categories: C,
        action_types: A,
    ) -> Result<Segments>
    where
        C: IntoIterator<Item = Category>,
        A: IntoIterator<Item = Action>,
    {
        let mut hasher = Sha256::new(); // create a Sha256 object
        hasher.update(&id); // write input message
        let hash = hasher.finalize(); // read hash digest and consume hasher

        let mut url = self.server_address
            .join("/api/skipSegments/")
            .unwrap()
            .join(&hex::encode(hash)[0..4])
            .unwrap();

        url.query_pairs_mut()
            .extend_pairs(categories.into_iter().map(|v| ("category", v)))
            .extend_pairs(action_types.into_iter().map(|v| ("actionType", v)));

        let req = self.client.get(url).send().await?.error_for_status()?;

        Ok(req
            .json::<Videos>()
            .await?
            .into_iter()
            .find(|v| v.video_id == id)
            .map_or(Segments::default(), |v| v.segments))
    }
}
