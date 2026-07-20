use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(
    name = "docker-search-rs",
    about = "Docker image search with size info",
    version
)]
struct Cli {
    query: String,

    #[arg(short, long, default_value_t = 10)]
    limit: u32,

    /// Max number of tags to show per image
    #[arg(long, default_value_t = 5)]
    tags_limit: u32,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[derive(Deserialize, Debug)]
struct SearchResult {
    repo_name: String,
    short_description: String,
    star_count: u64,
    is_official: bool,
}

#[derive(Deserialize, Debug)]
struct TagsResponse {
    results: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
struct Tag {
    name: String,
    images: Vec<ImageInfo>,
}

#[derive(Deserialize, Debug)]
struct ImageInfo {
    architecture: String,
    size: u64,
}

fn namespace_and_repo(result: &SearchResult) -> (String, String) {
    if let Some((ns, repo)) = result.repo_name.split_once('/') {
        (ns.to_string(), repo.to_string())
    } else {
        ("library".to_string(), result.repo_name.clone())
    }
}

fn format_size(bytes: u64) -> String {
    let mb = bytes as f64 / 1_048_576.0;
    format!("{mb:.1} MB")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let search_url = format!(
        "https://hub.docker.com/v2/search/repositories/?query={}&page_size={}",
        cli.query, cli.limit
    );

    let search_response: SearchResponse = reqwest::get(&search_url).await?.json().await?;

    for result in &search_response.results {
        let official = if result.is_official {
            " [OFFICIAL]"
        } else {
            ""
        };
        println!(
            "\n{}{}  ★ {}",
            result.repo_name, official, result.star_count
        );
        if !result.short_description.is_empty() {
            println!("  {}", result.short_description);
        }

        let (namespace, repo) = namespace_and_repo(result);
        let tags_url = format!(
            "https://hub.docker.com/v2/repositories/{namespace}/{repo}/tags/?page_size={}",
            cli.tags_limit
        );

        match reqwest::get(&tags_url).await {
            Ok(resp) => match resp.json::<TagsResponse>().await {
                Ok(tags_response) => {
                    for tag in &tags_response.results {
                        for image in &tag.images {
                            if image.architecture == "unknown" {
                                continue;
                            }
                            println!(
                                "    {}  ({})  {}",
                                tag.name,
                                image.architecture,
                                format_size(image.size)
                            );
                        }
                    }
                }
                Err(e) => eprintln!("    (couldn't parse tags: {e})"),
            },
            Err(e) => eprintln!("    (couldn't fetch tags: {e})"),
        }
    }

    Ok(())
}
