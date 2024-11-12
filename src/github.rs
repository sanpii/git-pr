#[derive(Debug, serde::Deserialize)]
struct PullRequest {
    head: Reference,
    base: Reference,
}

#[derive(Debug, serde::Deserialize)]
struct Reference {
    r#ref: String,
    repo: Repository,
}

#[derive(Debug, serde::Deserialize)]
struct Repository {
    clone_url: String,
    owner: Owner,
}

#[derive(Debug, serde::Deserialize)]
struct Owner {
    login: String,
}

pub fn merge(url: &str, no_merge: bool) -> crate::Result {
    let api_url = url
        .replace("github.com/", "api.github.com/repos/")
        .replace("/pull/", "/pulls/");

    let pr = ureq::get(&api_url)
        .call()?
        .body_mut()
        .read_json::<PullRequest>()?;

    if !no_merge {
        crate::merge(
            &pr.head.repo.clone_url,
            &pr.head.repo.owner.login,
            &pr.head.r#ref,
            &pr.base.r#ref,
        )?;
    }

    Ok(())
}
