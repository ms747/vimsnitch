use futures::future::try_join_all;
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
            url: format!("https://api.github.com/repos/{}/{}/issues", owner, repo),
            token: format!("token {}", token),
        }
    }

    // TODO(#33) : Replace with smol since async_std is causing memory leak

    pub fn get(&self, state: IssueState) -> Result<Vec<Issue>, http_types::Error> {
        smol::block_on(async {
            let mut url = String::from(&self.url);

            if state == IssueState::Closed {
                url += "?state=closed";
            }

            let response = surf::get(&url)
                .set_header("User-Agent", "vimsnitch")
                .set_header("Authorization", self.get_token())
                .recv_json::<Vec<Issue>>()
                .await?;
            Ok(response)
        })
    }

    pub fn create_many(&self, title: &[String]) -> Result<Vec<Issue>, http_types::Error> {
        smol::block_on(async {
            let mut issues = try_join_all(title.iter().map(|tit| {
                surf::post(&self.url)
                    .set_header("User-Agent", "vimsnitch")
                    .set_header("Authorization", self.get_token())
                    .body_json(&serde_json::json!({ "title": tit }))
                    .unwrap()
            }))
            .await
            .unwrap();

            let mut new_issues: Vec<Issue> = vec![];

            for (i, issue) in issues.iter_mut().enumerate() {
                if issue.status() == 201 {
                    let body = issue.body_string().await.unwrap();
                    let body = serde_json::from_str::<Issue>(&body).unwrap();
                    new_issues.push(body);
                } else {
                    return Err(http_types::Error::from_str(
                        issue.status(),
                        format!("Failed to create new Issue : {}", &title.get(i).unwrap()),
                    ));
                }
            }
            Ok(new_issues)
        })
    }

    pub fn close_many(&self, title: &[&u32]) -> Result<Vec<Issue>, http_types::Error> {
        smol::block_on(async {
            let mut bodies: Vec<_> = try_join_all(title.iter().map(|tit| {
                surf::patch(&format!("{}/{}", &self.url, tit))
                    .set_header("User-Agent", "vimsnitch")
                    .set_header("Authorization", self.get_token())
                    .body_json(&serde_json::json!({ "state": "closed" }))
                    .unwrap()
            }))
            .await
            .unwrap();

            let mut closed_issue: Vec<Issue> = vec![];

            for (i, body) in bodies.iter_mut().enumerate() {
                if body.status() == 200 {
                    let data = body.body_string().await.unwrap();
                    let data = serde_json::from_str::<Issue>(&data).unwrap();
                    closed_issue.push(data);
                } else {
                    return Err(http_types::Error::from_str(
                        body.status(),
                        format!("Failed to close Issue : {}", &title.get(i).unwrap()),
                    ));
                }
            }
            Ok(closed_issue)
        })
    }

    pub fn create(&self, title: &str) -> Result<Issue, http_types::Error> {
        smol::block_on(async {
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
                    "Error: Failed to create Issue".to_string(),
                ))
            }
        })
    }

    pub fn close(&self, issue: u32) -> Result<Issue, http_types::Error> {
        smol::block_on(async {
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
