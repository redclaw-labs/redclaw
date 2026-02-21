use crate::multimodal;
use crate::providers::traits::{
    ChatMessage, ChatResponse, Provider, ProviderCapabilities, ToolCall,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub struct OllamaProvider {
    base_url: String,
    api_key: Option<String>,
    reasoning_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    options: Options,
    #[serde(skip_serializing_if = "Option::is_none")]
    think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OutgoingToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct OutgoingToolCall {
    #[serde(rename = "type")]
    kind: String,
    function: OutgoingFunction,
}

#[derive(Debug, Serialize)]
struct OutgoingFunction {
    name: String,
    arguments: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct Options {
    temperature: f64,
}

#[derive(Debug, Deserialize)]
struct ApiChatResponse {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
    #[serde(default)]
    tool_calls: Vec<IncomingToolCall>,
    #[serde(default)]
    thinking: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IncomingToolCall {
    id: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    function: Option<IncomingFunction>,
}

#[derive(Debug, Deserialize)]
struct IncomingFunction {
    name: Option<String>,
    arguments: Option<serde_json::Value>,
}

impl OllamaProvider {
    pub fn new(base_url: Option<&str>, api_key: Option<&str>) -> Self {
        Self::new_with_reasoning(base_url, api_key, None)
    }

    pub fn new_with_reasoning(
        base_url: Option<&str>,
        api_key: Option<&str>,
        reasoning_enabled: Option<bool>,
    ) -> Self {
        let api_key = api_key.and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        });

        Self {
            base_url: Self::normalize_base_url(base_url.unwrap_or("http://localhost:11434")),
            api_key,
            reasoning_enabled,
        }
    }

    fn normalize_base_url(base_url: &str) -> String {
        let trimmed = base_url.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        let mut normalized = trimmed.trim_end_matches('/');

        // The provider always appends `/api/chat` when sending requests.
        // If the configured URL already ends with `/api`, strip it to avoid
        // accidentally generating `/api/api/chat`.
        if let Some(stripped) = normalized.strip_suffix("/api") {
            normalized = stripped.trim_end_matches('/');
        }

        if normalized.starts_with("http://") || normalized.starts_with("https://") {
            normalized.to_string()
        } else {
            format!("http://{normalized}")
        }
    }

    fn is_local_endpoint(&self) -> bool {
        reqwest::Url::parse(&self.base_url)
            .ok()
            .and_then(|url| url.host_str().map(|host| host.to_string()))
            .is_some_and(|host| matches!(host.as_str(), "localhost" | "127.0.0.1" | "::1"))
    }

    fn http_client(&self) -> Client {
        crate::config::build_runtime_proxy_client_with_timeouts("provider.ollama", 300, 10)
    }

    fn resolve_request_details(&self, model: &str) -> anyhow::Result<(String, bool)> {
        let requests_cloud = model.ends_with(":cloud");
        let normalized_model = model.strip_suffix(":cloud").unwrap_or(model).to_string();

        if requests_cloud && self.is_local_endpoint() {
            anyhow::bail!(
                "Model '{}' requested cloud routing, but Ollama endpoint is local. Configure api_url with a remote Ollama endpoint.",
                model
            );
        }

        if requests_cloud && self.api_key.is_none() {
            anyhow::bail!(
                "Model '{}' requested cloud routing, but no API key is configured. Set OLLAMA_API_KEY or config api_key.",
                model
            );
        }

        let should_auth = self.api_key.is_some() && !self.is_local_endpoint();
        Ok((normalized_model, should_auth))
    }

    fn parse_tool_arguments(arguments: &str) -> serde_json::Value {
        serde_json::from_str(arguments).unwrap_or_else(|_| serde_json::json!({}))
    }

    fn extract_tool_name_and_args(&self, call: &IncomingToolCall) -> (String, serde_json::Value) {
        let name = call
            .function
            .as_ref()
            .and_then(|f| f.name.clone())
            .unwrap_or_else(|| "unknown".to_string());
        let args = call
            .function
            .as_ref()
            .and_then(|f| f.arguments.clone())
            .unwrap_or_else(|| serde_json::json!({}));
        (name, args)
    }

    fn build_chat_request(
        &self,
        messages: Vec<Message>,
        model: &str,
        temperature: f64,
        tools: Option<&[serde_json::Value]>,
    ) -> ChatRequest {
        ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
            options: Options { temperature },
            think: self.reasoning_enabled,
            tools: tools.map(|t| t.to_vec()),
        }
    }

    fn convert_user_message_content(&self, content: &str) -> (Option<String>, Option<Vec<String>>) {
        let (cleaned, image_refs) = multimodal::parse_image_markers(content);
        if image_refs.is_empty() {
            return (Some(content.to_string()), None);
        }

        let images: Vec<String> = image_refs
            .iter()
            .filter_map(|reference| multimodal::extract_ollama_image_payload(reference))
            .collect();

        if images.is_empty() {
            return (Some(content.to_string()), None);
        }

        let cleaned = cleaned.trim();
        let content = if cleaned.is_empty() {
            None
        } else {
            Some(cleaned.to_string())
        };

        (content, Some(images))
    }

    /// Convert internal chat history format to Ollama's native tool-call message schema.
    fn convert_messages(&self, messages: &[ChatMessage]) -> Vec<Message> {
        let mut tool_name_by_id: HashMap<String, String> = HashMap::new();

        messages
            .iter()
            .map(|message| {
                if message.role == "assistant" {
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&message.content) {
                        if let Some(tool_calls_value) = value.get("tool_calls") {
                            if let Ok(parsed_calls) =
                                serde_json::from_value::<Vec<ToolCall>>(tool_calls_value.clone())
                            {
                                let outgoing_calls: Vec<OutgoingToolCall> = parsed_calls
                                    .into_iter()
                                    .map(|call| {
                                        tool_name_by_id.insert(call.id.clone(), call.name.clone());
                                        OutgoingToolCall {
                                            kind: "function".to_string(),
                                            function: OutgoingFunction {
                                                name: call.name,
                                                arguments: Self::parse_tool_arguments(
                                                    &call.arguments,
                                                ),
                                            },
                                        }
                                    })
                                    .collect();

                                let content = value
                                    .get("content")
                                    .and_then(serde_json::Value::as_str)
                                    .map(ToString::to_string);

                                return Message {
                                    role: "assistant".to_string(),
                                    content,
                                    images: None,
                                    tool_calls: Some(outgoing_calls),
                                    tool_name: None,
                                };
                            }
                        }
                    }
                }

                if message.role == "tool" {
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&message.content) {
                        let tool_name = value
                            .get("tool_name")
                            .and_then(serde_json::Value::as_str)
                            .map(ToString::to_string)
                            .or_else(|| {
                                value
                                    .get("tool_call_id")
                                    .and_then(serde_json::Value::as_str)
                                    .and_then(|id| tool_name_by_id.get(id))
                                    .cloned()
                            });

                        let content = value
                            .get("content")
                            .and_then(serde_json::Value::as_str)
                            .map(ToString::to_string)
                            .or_else(|| {
                                (!message.content.trim().is_empty())
                                    .then_some(message.content.clone())
                            });

                        return Message {
                            role: "tool".to_string(),
                            content,
                            images: None,
                            tool_calls: None,
                            tool_name,
                        };
                    }
                }

                if message.role == "user" {
                    let (content, images) = self.convert_user_message_content(&message.content);
                    return Message {
                        role: "user".to_string(),
                        content,
                        images,
                        tool_calls: None,
                        tool_name: None,
                    };
                }

                Message {
                    role: message.role.clone(),
                    content: Some(message.content.clone()),
                    images: None,
                    tool_calls: None,
                    tool_name: None,
                }
            })
            .collect()
    }

    async fn send_request(
        &self,
        messages: Vec<Message>,
        model: &str,
        temperature: f64,
        should_auth: bool,
        tools: Option<&[serde_json::Value]>,
    ) -> anyhow::Result<ApiChatResponse> {
        let request = self.build_chat_request(messages, model, temperature, tools);

        let url = format!("{}/api/chat", self.base_url);

        tracing::debug!(
            "Ollama request: url={} model={} message_count={} temperature={} think={:?} tool_count={}",
            url,
            model,
            request.messages.len(),
            temperature,
            request.think,
            request.tools.as_ref().map_or(0, |t| t.len()),
        );

        let mut request_builder = self.http_client().post(&url).json(&request);
        if should_auth {
            if let Some(key) = self.api_key.as_ref() {
                request_builder = request_builder.bearer_auth(key);
            }
        }

        let response = request_builder.send().await?;

        if !response.status().is_success() {
            let err = super::api_error("Ollama", response).await;
            anyhow::bail!("{err}. Is Ollama running? (brew install ollama && ollama serve)");
        }

        Ok(response.json().await?)
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            native_tool_calling: true,
            vision: true,
        }
    }

    async fn chat_with_system(
        &self,
        system_prompt: Option<&str>,
        message: &str,
        model: &str,
        temperature: f64,
    ) -> anyhow::Result<String> {
        let (normalized_model, should_auth) = self.resolve_request_details(model)?;

        let mut messages = Vec::new();

        if let Some(sys) = system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: Some(sys.to_string()),
                images: None,
                tool_calls: None,
                tool_name: None,
            });
        }

        let (user_content, user_images) = self.convert_user_message_content(message);
        messages.push(Message {
            role: "user".to_string(),
            content: user_content,
            images: user_images,
            tool_calls: None,
            tool_name: None,
        });

        let chat_response = self
            .send_request(messages, &normalized_model, temperature, should_auth, None)
            .await?;

        if !chat_response.message.tool_calls.is_empty() {
            let tool_calls: Vec<ToolCall> = chat_response
                .message
                .tool_calls
                .iter()
                .map(|tc| {
                    let (name, args) = self.extract_tool_name_and_args(tc);
                    ToolCall {
                        id: tc.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string()),
                        name,
                        arguments: serde_json::to_string(&args)
                            .unwrap_or_else(|_| "{}".to_string()),
                    }
                })
                .collect();
            let text = if chat_response.message.content.is_empty() {
                None
            } else {
                Some(chat_response.message.content)
            };
            let payload = serde_json::json!({
                "content": text,
                "tool_calls": tool_calls,
            });
            return Ok(payload.to_string());
        }

        Ok(chat_response.message.content)
    }

    async fn chat_with_history(
        &self,
        messages: &[ChatMessage],
        model: &str,
        temperature: f64,
    ) -> anyhow::Result<String> {
        let (normalized_model, should_auth) = self.resolve_request_details(model)?;

        let api_messages = self.convert_messages(messages);
        let response = self
            .send_request(
                api_messages,
                &normalized_model,
                temperature,
                should_auth,
                None,
            )
            .await?;

        if !response.message.tool_calls.is_empty() {
            let tool_calls: Vec<ToolCall> = response
                .message
                .tool_calls
                .iter()
                .map(|tc| {
                    let (name, args) = self.extract_tool_name_and_args(tc);
                    ToolCall {
                        id: tc.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string()),
                        name,
                        arguments: serde_json::to_string(&args)
                            .unwrap_or_else(|_| "{}".to_string()),
                    }
                })
                .collect();
            let text = if response.message.content.is_empty() {
                None
            } else {
                Some(response.message.content)
            };
            let payload = serde_json::json!({
                "content": text,
                "tool_calls": tool_calls,
            });
            return Ok(payload.to_string());
        }

        Ok(response.message.content)
    }

    async fn chat(
        &self,
        request: crate::providers::traits::ChatRequest<'_>,
        model: &str,
        temperature: f64,
    ) -> anyhow::Result<ChatResponse> {
        let (normalized_model, should_auth) = self.resolve_request_details(model)?;

        let api_messages = self.convert_messages(request.messages);

        let tools_opt = request.tools.map(|tools| {
            tools
                .iter()
                .map(|tool| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": tool.name,
                            "description": tool.description,
                            "parameters": tool.parameters,
                        }
                    })
                })
                .collect::<Vec<_>>()
        });

        let tools_opt = tools_opt.as_deref().filter(|tools| !tools.is_empty());

        let response = self
            .send_request(
                api_messages,
                &normalized_model,
                temperature,
                should_auth,
                tools_opt,
            )
            .await?;

        if !response.message.tool_calls.is_empty() {
            let tool_calls: Vec<ToolCall> = response
                .message
                .tool_calls
                .iter()
                .map(|tc| {
                    let (name, args) = self.extract_tool_name_and_args(tc);
                    ToolCall {
                        id: tc.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string()),
                        name,
                        arguments: serde_json::to_string(&args)
                            .unwrap_or_else(|_| "{}".to_string()),
                    }
                })
                .collect();

            let text = if response.message.content.is_empty() {
                None
            } else {
                Some(response.message.content)
            };
            return Ok(ChatResponse { text, tool_calls });
        }

        // Plain text response.
        let content = response.message.content;
        if content.is_empty() {
            if let Some(thinking) = &response.message.thinking {
                tracing::warn!(
                    "Ollama returned empty content with only thinking: '{}'. Model may have stopped prematurely.",
                    if thinking.len() > 100 { &thinking[..100] } else { thinking }
                );
                return Ok(ChatResponse {
                    text: Some(format!(
                        "I was thinking about this: {}... but I didn't complete my response. Could you try asking again?",
                        if thinking.len() > 200 { &thinking[..200] } else { thinking }
                    )),
                    tool_calls: vec![],
                });
            }
            tracing::warn!("Ollama returned empty content with no tool calls");
        }

        Ok(ChatResponse {
            text: Some(content),
            tool_calls: vec![],
        })
    }

    fn supports_native_tools(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_url() {
        let p = OllamaProvider::new(None, None);
        assert_eq!(p.base_url, "http://localhost:11434");
    }

    #[test]
    fn custom_url_trailing_slash() {
        let p = OllamaProvider::new(Some("http://192.168.1.100:11434/"), None);
        assert_eq!(p.base_url, "http://192.168.1.100:11434");
    }

    #[test]
    fn custom_url_no_trailing_slash() {
        let p = OllamaProvider::new(Some("http://myserver:11434"), None);
        assert_eq!(p.base_url, "http://myserver:11434");
    }

    #[test]
    fn custom_url_strips_api_suffix() {
        let p = OllamaProvider::new(Some("https://ollama.com/api/"), None);
        assert_eq!(p.base_url, "https://ollama.com");
    }

    #[test]
    fn empty_url_uses_empty() {
        let p = OllamaProvider::new(Some(""), None);
        assert_eq!(p.base_url, "");
    }

    #[test]
    fn cloud_suffix_strips_model_name() {
        let p = OllamaProvider::new(Some("https://ollama.com"), Some("ollama-key"));
        let (model, should_auth) = p.resolve_request_details("qwen3:cloud").unwrap();
        assert_eq!(model, "qwen3");
        assert!(should_auth);
    }

    #[test]
    fn cloud_suffix_with_local_endpoint_errors() {
        let p = OllamaProvider::new(None, Some("ollama-key"));
        let error = p
            .resolve_request_details("qwen3:cloud")
            .expect_err("cloud suffix should fail on local endpoint");
        assert!(error
            .to_string()
            .contains("requested cloud routing, but Ollama endpoint is local"));
    }

    #[test]
    fn cloud_suffix_without_api_key_errors() {
        let p = OllamaProvider::new(Some("https://ollama.com"), None);
        let error = p
            .resolve_request_details("qwen3:cloud")
            .expect_err("cloud suffix should require API key");
        assert!(error
            .to_string()
            .contains("requested cloud routing, but no API key is configured"));
    }

    #[test]
    fn remote_endpoint_auth_enabled_when_key_present() {
        let p = OllamaProvider::new(Some("https://ollama.com"), Some("ollama-key"));
        let (_model, should_auth) = p.resolve_request_details("qwen3").unwrap();
        assert!(should_auth);
    }

    #[test]
    fn remote_endpoint_with_api_suffix_still_allows_cloud_models() {
        let p = OllamaProvider::new(Some("https://ollama.com/api"), Some("ollama-key"));
        let (model, should_auth) = p.resolve_request_details("qwen3:cloud").unwrap();
        assert_eq!(model, "qwen3");
        assert!(should_auth);
    }

    #[test]
    fn local_endpoint_auth_disabled_even_with_key() {
        let p = OllamaProvider::new(None, Some("ollama-key"));
        let (_model, should_auth) = p.resolve_request_details("llama3").unwrap();
        assert!(!should_auth);
    }

    #[test]
    fn request_serializes_with_system() {
        let req = ChatRequest {
            model: "llama3".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: Some("You are RedClaw".to_string()),
                    images: None,
                    tool_calls: None,
                    tool_name: None,
                },
                Message {
                    role: "user".to_string(),
                    content: Some("hello".to_string()),
                    images: None,
                    tool_calls: None,
                    tool_name: None,
                },
            ],
            stream: false,
            options: Options { temperature: 0.7 },
            think: None,
            tools: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"stream\":false"));
        assert!(json.contains("llama3"));
        assert!(json.contains("system"));
        assert!(json.contains("\"temperature\":0.7"));
    }

    #[test]
    fn request_serializes_without_system() {
        let req = ChatRequest {
            model: "mistral".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: Some("test".to_string()),
                images: None,
                tool_calls: None,
                tool_name: None,
            }],
            stream: false,
            options: Options { temperature: 0.0 },
            think: None,
            tools: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("\"role\":\"system\""));
        assert!(json.contains("mistral"));
    }

    #[test]
    fn request_omits_think_when_reasoning_not_configured() {
        let provider = OllamaProvider::new(None, None);
        let request = provider.build_chat_request(
            vec![Message {
                role: "user".to_string(),
                content: Some("hello".to_string()),
                images: None,
                tool_calls: None,
                tool_name: None,
            }],
            "llama3",
            0.7,
            None,
        );

        let json = serde_json::to_value(request).unwrap();
        assert!(json.get("think").is_none());
    }

    #[test]
    fn request_includes_think_when_reasoning_configured() {
        let provider = OllamaProvider::new_with_reasoning(None, None, Some(false));
        let request = provider.build_chat_request(
            vec![Message {
                role: "user".to_string(),
                content: Some("hello".to_string()),
                images: None,
                tool_calls: None,
                tool_name: None,
            }],
            "llama3",
            0.7,
            None,
        );

        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json.get("think"), Some(&serde_json::json!(false)));
    }

    #[test]
    fn convert_messages_extracts_images_from_user_marker() {
        let provider = OllamaProvider::new(None, None);
        let messages = vec![ChatMessage {
            role: "user".into(),
            content: "Inspect this screenshot [IMAGE:data:image/png;base64,abcd==]".into(),
        }];

        let converted = provider.convert_messages(&messages);
        assert_eq!(converted.len(), 1);
        assert_eq!(converted[0].role, "user");
        assert_eq!(
            converted[0].content.as_deref(),
            Some("Inspect this screenshot")
        );
        let images = converted[0]
            .images
            .as_ref()
            .expect("images should be present");
        assert_eq!(images, &vec!["abcd==".to_string()]);
    }

    #[test]
    fn capabilities_include_native_tools_and_vision() {
        let provider = OllamaProvider::new(None, None);
        let caps = <OllamaProvider as Provider>::capabilities(&provider);
        assert!(caps.native_tool_calling);
        assert!(caps.vision);
    }

    #[test]
    fn response_deserializes() {
        let json = r#"{"message":{"role":"assistant","content":"Hello from Ollama!"}}"#;
        let resp: ApiChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.message.content, "Hello from Ollama!");
    }

    #[test]
    fn response_with_empty_content() {
        let json = r#"{"message":{"role":"assistant","content":""}}"#;
        let resp: ApiChatResponse = serde_json::from_str(json).unwrap();
        assert!(resp.message.content.is_empty());
    }

    #[test]
    fn response_with_multiline() {
        let json = r#"{"message":{"role":"assistant","content":"line1\nline2\nline3"}}"#;
        let resp: ApiChatResponse = serde_json::from_str(json).unwrap();
        assert!(resp.message.content.contains("line1"));
    }
}
