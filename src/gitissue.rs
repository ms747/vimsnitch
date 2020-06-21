use async_std::task;
use http_types::headers::HeaderValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub title: String,
    pub number: i64,
    pub state: String,
}

pub struct GitIssue {
    url: String,
    token: String,
}

#[derive(PartialEq)]
pub enum IssueState {
    Open,
    Closed,
}

impl GitIssue {
    pub fn new(owner: &str, repo: &str, token: &str) -> Self {
        GitIssue {
            url: format!("https://api.github.com/repos/{}/{}/issues", owner, repo).to_string(),
            token: format!("token {}", token),
        }
    }

    pub fn get(&self, state: IssueState) -> Result<Vec<Issue>, http_types::Error> {
        task::block_on(async {
            let mut url = String::from(&self.url);

            if state == IssueState::Closed {
                url = url + "?state=closed";
            }

            let response = surf::get(&url)
                .set_header("User-Agent", "vimsnitch")
                .set_header("Authorization", self.get_token())
                .recv_json::<Vec<Issue>>()
                .await?;
            Ok(response)
        })
    }

    pub fn create(&self, title: &str) -> Result<Issue, http_types::Error> {
        task::block_on(async {
            let mut response = surf::post(&self.url)
                .set_header("User-Agent", "vimsnitch")
                .set_header("Authorization", self.get_token())
                .body_json(&serde_json::json!({ "title": title }))?
                .await?;
            if response.status() == 201 {
                let body = response.body_string().await?;
                let body = serde_json::from_str::<Issue>(&body)?;
                Ok(body)
            } else {
                Err(http_types::Error::from_str(
                    response.status(),
                    format!("Error : Failed to create Issue"),
                ))
            }
        })
    }

    pub fn close(&self, issue: u32) -> Result<Issue, http_types::Error> {
        task::block_on(async {
            let mut response = surf::patch(&format!("{}/{}", &self.url, issue))
                .set_header("User-Agent", "vimsnitch")
                .set_header("Authorization", self.get_token())
                .body_json(&serde_json::json!({ "state" : "closed" }))?
                .await?;
            if response.status() == 200 {
                let body = response.body_string().await?;
                let body = serde_json::from_str::<Issue>(&body)?;
                Ok(body)
            } else {
                Err(http_types::Error::from_str(
                    response.status(),
                    format!("Issue : {} not found", issue),
                ))
            }
        })
    }

    fn get_token(&self) -> HeaderValue {
        let token = String::from(&self.token);
        let token = HeaderValue::from_bytes(token.as_bytes().to_vec()).unwrap();
        token
    }
}
