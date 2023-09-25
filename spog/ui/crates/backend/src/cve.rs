use crate::{ApplyAccessToken, Backend, Endpoint};
use spog_model::prelude::*;
use spog_ui_common::error::*;
use std::rc::Rc;
use yew_oauth2::prelude::*;

pub struct CveService {
    backend: Rc<Backend>,
    access_token: Option<LatestAccessToken>,
    client: reqwest::Client,
}

impl CveService {
    pub fn new(backend: Rc<Backend>, access_token: Option<LatestAccessToken>) -> Self {
        Self {
            backend,
            access_token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, id: impl AsRef<str>) -> Result<CveDetails, ApiError> {
        let url = self.backend.join(
            Endpoint::Api,
            &format!("/api/v1/cve/{id}", id = urlencoding::encode(id.as_ref())),
        )?;

        let response = self
            .client
            .get(url)
            .latest_access_token(&self.access_token)
            .send()
            .await?;

        Ok(response.api_error_for_status().await?.json().await?)
    }
}