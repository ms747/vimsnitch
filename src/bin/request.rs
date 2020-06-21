// fn main() {
//     let resp = reqwest::locking::get("https://api.github.com/repos/ms747/dotfiles/issues")
//         .unwrap()
//         .json::<HashMap<String, String>>()
//         .unwrap();

//     dbg!(resp);
// }
//

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Issues {
    pub title: String,
    pub number: i64,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    pub title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/ms747/vimsnitch/issues";
    let client = reqwest::Client::new();

    // Getting Issues
    let resp = client
        .get(url)
        .header("User-Agent", "vimsnitch")
        .header(
            "Authorization",
            "token 126562439d17dc58ab483485ff006b4af0ef07d3",
        )
        .send()
        .await?;
    println!("Fetching Issues");
    let body = resp.json::<Vec<Issues>>().await?;
    println!("{:#?}", body);

    // Create Issue
    // let issue = Issue {
    //     title: String::from("Issue from vimsnitch"),
    // };
    // let resp = client
    //     .post(url)
    //     .json(&issue)
    //     .header("User-Agent", "vimsnitch")
    //     .header(
    //         "Authorization",
    //         "token 126562439d17dc58ab483485ff006b4af0ef07d3",
    //     )
    //     .send()
    //     .await?;
    // println!("Creating Issue");
    // let body = resp.json::<Issues>().await?;
    // println!("{:#?}", body);

    // Close Issue
    // let resp = client
    //     .patch(&format!("{}/5", url))
    //     .json(&issue)
    //     .header("User-Agent", "vimsnitch")
    //     .header(
    //         "Authorization",
    //         "token 126562439d17dc58ab483485ff006b4af0ef07d3",
    //     )
    //     .send()
    //     .await?;
    // println!("Creating Issue");
    // let body = resp.json::<Issues>().await?;
    // println!("{:#?}", body);

    Ok(())
}
