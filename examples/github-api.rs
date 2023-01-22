use std::{fmt::Write, error::Error};
use http::header::USER_AGENT;
use serde::Serialize;

fn write_repo_fragment(f: &mut impl Write, index: usize, owner: &str, name: &str) -> Result<(), Box<dyn Error>> {
  write!(f, "
    repo{index}: repository(owner: \"{owner}\", name: \"{name}\") {{
      owner {{
        id
      }},
      name,
      forkCount,
      stargazerCount,
      pushedAt,
      updatedAt,
      defaultBranchRef {{
        target {{
          ... on Commit {{
            committedDate,
            message,
          }}
        }}
      }}
    }}
  ")?;

  Ok(())
}

fn build_repo_query(repos: &[(&str, &str)]) -> Result<String, Box<dyn Error>> {
  let mut query = String::with_capacity(5 * 1000);
  writeln!(query, "query {{")?;

  for (i, (owner, name)) in repos.iter().enumerate() {
    write_repo_fragment(&mut query, i, owner, name)?;
  }

  writeln!(query, "
    rateLimit {{
      resetAt,
      remaining,
      cost,
      used,
      limit,
    }}
  ")?;
  writeln!(query, "}}")?;

  Ok(query)
}

#[derive(Serialize)]
struct Query {
  query: String,
}

const TOKEN : &str = "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";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  let repos = &[
    ("iced-rs", "iced"),
    ("emilk", "egui"),
    ("linebender", "druid"),
    ("DioxusLabs", "dioxus"),
    ("bevyengine", "bevy"),
    ("slint-ui", "slint"),
    ("audulus", "rui"),
    ("linebender", "xilem"),
    ("StarArawn", "kayak_ui"),
    ("kas-gui", "kas"),
    ("matthunz", "viewbuilder"),
    ("marc2332", "freya"),
    ("vizia", "vizia"),
  ];

  let query = Query {
    query: build_repo_query(repos)?
  };

  println!("{}", &query.query);

  let client = reqwest::Client::new();
  let response : serde_json::Value = client.post("https://api.github.com/graphql")
    .header(USER_AGENT, "BlessedRS")
    .bearer_auth(TOKEN)
    .json(&query)
    .send()
    .await?
    .json()
    .await?;


  println!("{:#?}", response);


  Ok(())

}

