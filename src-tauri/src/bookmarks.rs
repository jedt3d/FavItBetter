use serde::Serialize;
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedBookmark {
    pub folder_path: String,
    pub title: String,
    pub original_url: String,
    pub canonical_url: String,
    pub cleaned_url: String,
    pub date_added: Option<String>,
}

pub fn parse_chromium_bookmarks(content: &str) -> Result<Vec<ParsedBookmark>, String> {
    let root: Value =
        serde_json::from_str(content).map_err(|err| format!("Invalid Chromium JSON: {err}"))?;
    let roots = root
        .get("roots")
        .and_then(Value::as_object)
        .ok_or_else(|| "Chromium Bookmarks JSON is missing the roots object".to_string())?;

    let mut bookmarks = Vec::new();

    for (root_key, root_node) in roots {
        if !root_node.is_object() {
            continue;
        }

        let root_name = node_name(root_node).unwrap_or_else(|| display_root_name(root_key));
        let mut folders = vec![root_name];
        walk_node(root_node, &mut folders, &mut bookmarks);
    }

    Ok(bookmarks)
}

fn walk_node(node: &Value, folders: &mut Vec<String>, bookmarks: &mut Vec<ParsedBookmark>) {
    match node.get("type").and_then(Value::as_str) {
        Some("url") => {
            if let Some(url) = node.get("url").and_then(Value::as_str) {
                let title = node_name(node).unwrap_or_else(|| url.to_string());
                let canonical_url = canonicalize_url(url);
                bookmarks.push(ParsedBookmark {
                    folder_path: folders.join(" / "),
                    title,
                    original_url: url.trim().to_string(),
                    cleaned_url: canonical_url.clone(),
                    canonical_url,
                    date_added: node
                        .get("date_added")
                        .and_then(Value::as_str)
                        .map(ToString::to_string),
                });
            }
        }
        Some("folder") | None => {
            let pushed = if let Some(name) = node_name(node) {
                if folders.last().map(|value| value != &name).unwrap_or(true) {
                    folders.push(name);
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if let Some(children) = node.get("children").and_then(Value::as_array) {
                for child in children {
                    walk_node(child, folders, bookmarks);
                }
            }

            if pushed {
                folders.pop();
            }
        }
        _ => {}
    }
}

fn node_name(node: &Value) -> Option<String> {
    node.get("name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn display_root_name(root_key: &str) -> String {
    match root_key {
        "bookmark_bar" => "Bookmarks Bar".to_string(),
        "other" => "Other Bookmarks".to_string(),
        "synced" => "Mobile Bookmarks".to_string(),
        value => value.replace('_', " "),
    }
}

fn canonicalize_url(raw_url: &str) -> String {
    let trimmed = raw_url.trim();
    match Url::parse(trimmed) {
        Ok(mut parsed) => {
            parsed.set_fragment(None);
            parsed.to_string()
        }
        Err(_) => trimmed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_chromium_bookmarks;

    #[test]
    fn parses_nested_chromium_bookmark_urls() {
        let content = r#"{
          "roots": {
            "bookmark_bar": {
              "type": "folder",
              "name": "Bookmarks Bar",
              "children": [
                {
                  "type": "folder",
                  "name": "Work",
                  "children": [
                    {
                      "type": "url",
                      "name": "Example",
                      "url": "https://example.com/page#section",
                      "date_added": "13300000000000000"
                    }
                  ]
                }
              ]
            }
          },
          "version": 1
        }"#;

        let parsed = parse_chromium_bookmarks(content).expect("fixture should parse");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].folder_path, "Bookmarks Bar / Work");
        assert_eq!(parsed[0].title, "Example");
        assert_eq!(parsed[0].canonical_url, "https://example.com/page");
    }
}
