use vimsnitch::gitissue::{GitIssue, IssueState};

fn main() -> Result<(), http_types::Error> {
    let issues = GitIssue::new(
        "ms747",
        "vimsnitch",
        "126562439d17dc58ab483485ff006b4af0ef07d3",
    );
    // println!("{:?}", issues.create_many(&vec!["Final Api", "Working"])?);
    // let x = issues.close_many(&vec![7, 39])?;
    // println!("{:#?}", x);
    println!("{:#?}", issues.get(IssueState::Open)?);
    println!("{:#?}", issues.get(IssueState::Closed)?);
    // println!("{:#?}", issues.close(8)?);
    // println!("{:#?}", issues.close(25)?);
    Ok(())
}
