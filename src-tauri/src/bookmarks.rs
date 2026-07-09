use serde::Serialize;
use serde_json::Value;
use url::{form_urlencoded, Url};

const TRACKING_PARAM_EXACT: &[&str] = &[
    "fbclid", "gclid", "msclkid", "mc_cid", "mc_eid", "igshid", "ref", "spm",
];

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlCleanResult {
    pub cleaned_url: String,
    pub removed_tracking_params: Vec<String>,
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
                let canonical_url = clean_url(url).cleaned_url;
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

pub fn clean_url(raw_url: &str) -> UrlCleanResult {
    let trimmed = raw_url.trim();
    match Url::parse(trimmed) {
        Ok(mut parsed) => {
            parsed.set_fragment(None);

            let mut removed_tracking_params = Vec::new();
            let retained_pairs = parsed
                .query_pairs()
                .filter_map(|(name, value)| {
                    let name_string = name.to_string();
                    if is_tracking_param(&name_string) {
                        removed_tracking_params.push(name_string);
                        None
                    } else {
                        Some((name_string, value.to_string()))
                    }
                })
                .collect::<Vec<_>>();

            parsed.set_query(None);
            if !retained_pairs.is_empty() {
                let query = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(
                        retained_pairs
                            .iter()
                            .map(|(name, value)| (&**name, &**value)),
                    )
                    .finish();
                parsed.set_query(Some(&query));
            }

            UrlCleanResult {
                cleaned_url: parsed.to_string(),
                removed_tracking_params,
            }
        }
        Err(_) => UrlCleanResult {
            cleaned_url: trimmed.to_string(),
            removed_tracking_params: Vec::new(),
        },
    }
}

fn is_tracking_param(name: &str) -> bool {
    let lower_name = name.to_ascii_lowercase();
    lower_name.starts_with("utm_")
        || TRACKING_PARAM_EXACT
            .iter()
            .any(|candidate| lower_name == *candidate)
}

#[cfg(test)]
mod tests {
    use super::{clean_url, parse_chromium_bookmarks};

    #[test]
    fn removes_tracking_params_and_preserves_content_params() {
        let result = clean_url(
            "https://example.com/article?id=42&utm_source=newsletter&fbclid=abc&view=full#hero",
        );

        assert_eq!(
            result.cleaned_url,
            "https://example.com/article?id=42&view=full"
        );
        assert_eq!(
            result.removed_tracking_params,
            vec!["utm_source".to_string(), "fbclid".to_string()]
        );
    }

    #[test]
    fn preserves_non_tracking_query_strings() {
        let result = clean_url("https://example.com/search?q=tauri&page=2");
        assert_eq!(
            result.cleaned_url,
            "https://example.com/search?q=tauri&page=2"
        );
        assert!(result.removed_tracking_params.is_empty());
    }

    #[test]
    fn produces_same_canonical_url_for_tracking_variants() {
        let first = clean_url("https://example.com/page?utm_medium=email&id=7");
        let second = clean_url("https://example.com/page?id=7");
        assert_eq!(first.cleaned_url, second.cleaned_url);
    }

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
