use regex::Regex;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use vimsnitch::gitignore::Gitignore;
use vimsnitch::gitissue::GitIssue;
use vimsnitch::matched::{File, MatchedLine};

use ansi_term::Colour;
use git2::Repository;

fn get_git_token() -> String {
    let git_token = match std::env::var("GIT") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Variable : $GIT not set");
            std::process::exit(1);
        }
    };
    git_token
}

fn get_repo_path() -> git2::Repository {
    let repo = match Repository::discover(".") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Not a git repository : {}", e);
            std::process::exit(1);
        }
    };
    repo
}

fn get_remote(repo: &git2::Repository) -> git2::Remote {
    let remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("No remotes set : {}", e);
            std::process::exit(1);
        }
    };
    remote
}

fn get_repo_local_path(repo: &git2::Repository) -> std::path::PathBuf {
    let mut path = repo.path().parent().unwrap().to_path_buf();
    path.push(".gitignore");
    path
}

fn get_repo_url<'a>(remote: &git2::Remote) -> String {
    let url: String = remote.url().unwrap().split(':').skip(1).collect();
    url
}

fn get_owner_and_repo(url: Vec<&str>) -> (&str, String) {
    let owner = url[0];
    let repo: String = url[1].split('.').take(1).collect();
    (owner, repo)
}

fn main() -> Result<(), http_types::Error> {
    let git_token = get_git_token();
    let repo = get_repo_path();
    let remote = get_remote(&repo);
    let path = get_repo_local_path(&repo);
    let base_path = std::env::current_dir().unwrap();
    let url = get_repo_url(&remote);
    let url: Vec<&str> = url.split('/').collect();
    let (owner, repo) = get_owner_and_repo(url);

    let issues = GitIssue::new(&owner, &repo, &git_token);

    let current_path = Path::new(path.to_str().unwrap());

    let mut ignore = Gitignore::new(current_path);
    ignore.included_files();

    let todo_regex = Regex::new(r"//\s*TODO\s*:\s*(.*)").unwrap();

    let mut threads = vec![];
    let (tx, rx) = mpsc::channel::<(File, Vec<MatchedLine>)>();

    for file in ignore.get_files().into_iter() {
        let regex = todo_regex.clone();
        let tx = tx.clone();

        threads.push(thread::spawn(move || {
            let file = file.to_str().unwrap();
            let contents = read_to_string(file).expect("Unable to open file");

            let mut line_matches: Vec<MatchedLine> = vec![];
            let mut found = false;

            for (i, line) in contents.lines().enumerate() {
                let line = line.trim();
                let matches = regex.captures(line);
                if matches.iter().len() > 0 {
                    let matched = match &matches {
                        Some(data) => data.get(1).unwrap().as_str().trim(),
                        _ => panic!("Regular Expression Failed"),
                    };
                    let line_count: usize = i + 1;
                    line_matches.push(MatchedLine::new(line_count, matched));
                    found = true;
                }
            }

            if found {
                tx.send((file.to_string(), line_matches)).unwrap();
            }
        }));
    }

    let mut storage = vec![];

    for thread in threads {
        thread.join().unwrap();
        if let Ok(data) = rx.try_recv() {
            storage.push(data);
        }
    }

    let mut todos = vec![];

    if storage.iter().len() == 0 {
        println!("{}", Colour::Green.paint("No Todos found :)"));
    } else {
        for (file, matches) in storage.iter() {
            let file = std::path::Path::new(file);
            println!(
                "{}",
                Colour::Purple.paint(file.strip_prefix(&base_path).unwrap().display().to_string())
            );
            for capture in matches.iter() {
                todos.push(capture.get_line());
                println!("{}", capture);
            }
        }
    }

    let new_issues = issues
        .create_many(&todos[..])?
        .iter()
        .map(|val| val.number as usize)
        .collect::<Vec<usize>>();

    let mut write_threads = vec![];

    let new_issues_index = Arc::new(Mutex::new(0));

    for (file, patterns) in storage.iter() {
        let todo_regex = todo_regex.clone();
        let patterns = patterns.clone();
        let file = file.clone();
        let patterns = patterns.clone();
        let new_issues_index = new_issues_index.clone();
        let new_issues = new_issues.clone();
        write_threads.push(thread::spawn(move || {
            let mut new_issues_index = *new_issues_index.lock().unwrap() as usize;
            let contents = read_to_string(&file).expect("Unable to Read File");
            let mut new_contents = String::new();
            let mut pattern_index: usize = 0;
            for (i, line) in contents.lines().enumerate() {
                if pattern_index > patterns.len() - 1 {
                    pattern_index -= 1;
                }
                if patterns[pattern_index].get_line_num() == i + 1 {
                    let editied = todo_regex.replace(line, |capture: &regex::Captures| {
                        format!(
                            "// TODO(#{}) : {}",
                            new_issues[new_issues_index], &capture[1]
                        )
                    });
                    new_contents.push_str(&format!("{}\n", editied));
                    pattern_index += 1;
                    new_issues_index += 1;
                    continue;
                } else {
                    new_contents.push_str(&format!("{}\n", line));
                }
            }
            write(file, new_contents.as_str()).expect("Unable to Write File");
        }));
    }

    for thread in write_threads {
        thread.join().unwrap();
    }

    Ok(())
}
