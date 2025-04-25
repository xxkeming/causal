use serde::Serialize;

use super::{ToolDescription, ToolObject};

use crate::error;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f32,
}

pub struct SearchTaivlyTool {
    tavily: tavily::Tavily,
    description: Vec<ToolDescription>,
}

impl SearchTaivlyTool {
    pub fn try_new(api_key: String) -> Result<Self, error::Error> {
        let tavily = tavily::Tavily::builder(api_key)
            .timeout(std::time::Duration::from_secs(60))
            .max_retries(5)
            .build()?;

        let description = ToolDescription {
            name: "tavily_web_search".to_string(),
            description: "Useful for when you need to answer questions by searching the web."
                .to_string(),
            schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string"
                    }
                }
            }),
        };

        Ok(Self { tavily, description: vec![description] })
    }
}

impl ToolObject for SearchTaivlyTool {
    fn description(&self) -> Vec<ToolDescription> {
        self.description.clone()
    }
    fn call<'a>(
        &'a self, _name: &'a str, param: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, error::Error>> + Send + 'a>,
    > {
        Box::pin(async move {
            let query = param.get("query").and_then(|value| value.as_str()).unwrap_or_default();

            let results = self.tavily.search(query).await?;

            let results = results
                .results
                .into_iter()
                .map(|result| SearchResult {
                    title: result.title,
                    url: result.url,
                    content: result.content,
                    raw_content: result.raw_content,
                    score: result.score,
                })
                .collect::<Vec<_>>();
            Ok(serde_json::to_value(results)?)
        })
    }
}
