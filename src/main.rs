#![warn(warnings)]

mod github;
mod gitlab;

use clap::Parser;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! git {
    ( $($args:expr),+ ) => {
        git(&[$($args, )*])
    }
}

#[derive(Parser)]
struct Opt {
    #[clap(long, default_value_t)]
    no_merge: bool,
    url: String,
}

fn main() -> Result {
    let opt = Opt::parse();

    if opt.url.starts_with("https://github.com/") {
        github::merge(&opt.url, opt.no_merge)
    } else {
        gitlab::merge(&opt.url, opt.no_merge)
    }
}

fn merge(remote: &str, remote_name: &str, branch: &str, target: &str) -> Result {
    if !remote_exists(remote, remote_name)? {
        git!("remote", "add", remote_name, remote)?;
    }

    let merge_branch = format!("{remote_name}-{branch})");

    git!("fetch", remote_name)?;
    git!(
        "checkout",
        "-b",
        &merge_branch,
        &format!("{remote_name}/{branch}")
    )?;

    if !is_branch_uptodate(&merge_branch, target)? {
        git!("rebase", target)?;
    }

    git!("checkout", target)?;
    git!("merge", "--no-edit", &merge_branch)?;
    git!("branch", "--delete", "--force", &merge_branch)?;

    Ok(())
}

fn remote_exists(url: &str, name: &str) -> Result<bool> {
    let remote = format!("{name}\t{url}");

    let exists = git!("remote", "--verbose")?.lines().any(|x| x == remote);

    Ok(exists)
}

fn is_branch_uptodate(branch: &str, target: &str) -> Result<bool> {
    let n = git!("log", &format!("{branch}..{target}"))?.lines().count();

    Ok(n > 0)
}

fn git(args: &[&str]) -> std::io::Result<String> {
    let output = std::process::Command::new("git").args(args).output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
