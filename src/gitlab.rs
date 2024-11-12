#[derive(Debug, serde::Deserialize)]
struct PullRequest {
    source_project_id: u32,
    author: Author,
    source_branch: String,
    target_branch: String,
    merge_status: String,
}

#[derive(Debug, serde::Deserialize)]
struct Author {
    username: String,
}

pub fn merge(url: &str, no_merge: bool) -> crate::Result {
    static REGEX: std::sync::LazyLock<regex_lite::Regex> = std::sync::LazyLock::new(|| {
        regex_lite::Regex::new(r"(https://[^/]+)/([^/]+)/([^/]+)/-/(.*)").unwrap()
    });

    let api_url = REGEX.replace(url, r"$1/api/v4/projects/$2%2F$3/$4");

    let pr = ureq::get(api_url.into_owned())
        .call()?
        .body_mut()
        .read_json::<PullRequest>()?;

    if pr.merge_status == "can_be_merged" {
        if !no_merge {
            crate::merge(
                &remote(url, pr.source_project_id)?,
                &pr.author.username,
                &pr.source_branch,
                &pr.target_branch,
            )?;
        }
    } else {
        eprintln!("This MR couldn’t be merged!");
    }

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Repository {
    http_url_to_repo: String,
}

fn remote(url: &str, id: u32) -> crate::Result<String> {
    static REGEX: std::sync::LazyLock<regex_lite::Regex> =
        std::sync::LazyLock::new(|| regex_lite::Regex::new(r"(https://[^/]+)").unwrap());
    let base_url = REGEX.captures(url).unwrap().get(0).unwrap().as_str();
    let api_url = format!("{base_url}/api/v4/projects/{id}");

    let repo = ureq::get(&api_url)
        .call()?
        .body_mut()
        .read_json::<Repository>()?;

    Ok(repo.http_url_to_repo)
}
