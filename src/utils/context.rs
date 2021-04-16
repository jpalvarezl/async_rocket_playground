use crate::utils::InfoProvider;
use rocket::request::{self, FromRequest, Outcome};
use rocket::{Request, State};

pub struct Context<'a> {
    client: State<'a, reqwest::Client>,
}

impl<'a> Context<'a> {
    pub fn info_provider(self) -> InfoProvider<'a> {
        InfoProvider::from(self.client)
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Context<'a> {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let client = request.guard::<State<reqwest::Client>>().await.unwrap();

        let uri = request.uri().to_string();
        let host = request
            .headers()
            .get_one("Host")
            .map(|host| host.to_string());

        return request::Outcome::Success(Context { client });
    }
}
