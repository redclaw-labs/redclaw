#![allow(clippy::uninlined_format_args)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::trim_split_whitespace)]
#![allow(clippy::doc_link_with_quotes)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_map_or)]

use anyhow::{anyhow, Result};
use async_imap::extensions::idle::IdleResponse;
use async_imap::types::Fetch;
use async_imap::Session;
use async_trait::async_trait;
use futures_util::TryStreamExt;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use mail_parser::{MessageParser, MimeHeaders};
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::DnsName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Maximum number of seen message IDs to retain before evicting the oldest.
const SEEN_MESSAGES_CAPACITY: usize = 100_000;

type ImapSession = Session<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>;

use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, timeout};
use tokio_rustls::client::TlsStream;
use tokio_rustls::TlsConnector;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::traits::{Channel, ChannelMessage, SendMessage};

/// Email channel configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmailConfig {
    /// IMAP server hostname
    pub imap_host: String,
    /// IMAP server port (default: 993 for TLS)
    #[serde(default = "default_imap_port")]
    pub imap_port: u16,
    /// IMAP folder to poll (default: INBOX)
    #[serde(default = "default_imap_folder")]
    pub imap_folder: String,
    /// SMTP server hostname
    pub smtp_host: String,
    /// SMTP server port (default: 587 for STARTTLS)
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    /// Use TLS for SMTP (default: true)
    #[serde(default = "default_true")]
    pub smtp_tls: bool,
    /// Email username for authentication
    pub username: String,
    /// Email password for authentication
    pub password: String,
    /// From address for outgoing emails
    pub from_address: String,
    /// IDLE timeout in seconds before re-establishing connection (default: 1740 = 29 minutes)
    /// RFC 2177 recommends clients restart IDLE every 29 minutes
    #[serde(default = "default_idle_timeout", alias = "poll_interval_secs")]
    pub idle_timeout_secs: u64,
    /// Allowed sender addresses/domains (empty = deny all, ["*"] = allow all)
    #[serde(default)]
    pub allowed_senders: Vec<String>,
}

fn default_imap_port() -> u16 {
    993
}
fn default_smtp_port() -> u16 {
    587
}
fn default_imap_folder() -> String {
    "INBOX".into()
}
fn default_idle_timeout() -> u64 {
    1740 // 29 minutes per RFC 2177
}
fn default_true() -> bool {
    true
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            imap_host: String::new(),
            imap_port: default_imap_port(),
            imap_folder: default_imap_folder(),
            smtp_host: String::new(),
            smtp_port: default_smtp_port(),
            smtp_tls: true,
            username: String::new(),
            password: String::new(),
            from_address: String::new(),
            idle_timeout_secs: default_idle_timeout(),
            allowed_senders: Vec::new(),
        }
    }
}

/// Bounded dedup set that evicts oldest entries when capacity is reached.
struct BoundedSeenSet {
    set: HashSet<String>,
    order: VecDeque<String>,
    capacity: usize,
}

impl BoundedSeenSet {
    fn new(capacity: usize) -> Self {
        Self {
            set: HashSet::with_capacity(capacity.min(1024)),
            order: VecDeque::with_capacity(capacity.min(1024)),
            capacity,
        }
    }

    fn contains(&self, id: &str) -> bool {
        self.set.contains(id)
    }

    fn insert(&mut self, id: String) -> bool {
        if self.set.contains(&id) {
            return false;
        }
        if self.order.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.set.remove(&oldest);
            }
        }
        self.order.push_back(id.clone());
        self.set.insert(id);
        true
    }

    fn len(&self) -> usize {
        self.set.len()
    }
}

/// Email channel — IMAP IDLE for instant push notifications, SMTP for outbound
pub struct EmailChannel {
    pub config: EmailConfig,
    seen_messages: Arc<Mutex<BoundedSeenSet>>,
}

impl EmailChannel {
    pub fn new(config: EmailConfig) -> Self {
        Self {
            config,
            seen_messages: Arc::new(Mutex::new(BoundedSeenSet::new(SEEN_MESSAGES_CAPACITY))),
        }
    }

    /// Check if a sender email is in the allowlist
    pub fn is_sender_allowed(&self, email: &str) -> bool {
        if self.config.allowed_senders.is_empty() {
            return false; // Empty = deny all
        }
        if self.config.allowed_senders.iter().any(|a| a == "*") {
            return true; // Wildcard = allow all
        }
        let email_lower = email.to_lowercase();
        self.config.allowed_senders.iter().any(|allowed| {
            if allowed.starts_with('@') {
                // Domain match with @ prefix: "@example.com"
                email_lower.ends_with(&allowed.to_lowercase())
            } else if allowed.contains('@') {
                // Full email address match
                allowed.eq_ignore_ascii_case(email)
            } else {
                // Domain match without @ prefix: "example.com"
                email_lower.ends_with(&format!("@{}", allowed.to_lowercase()))
            }
        })
    }

    /// Strip HTML tags from content (basic)
    pub fn strip_html(html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;
        for ch in html.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(ch),
                _ => {}
            }
        }
        let mut normalized = String::with_capacity(result.len());
        for word in result.split_whitespace() {
            if !normalized.is_empty() {
                normalized.push(' ');
            }
            normalized.push_str(word);
        }
        normalized
    }

    /// Extract the sender address from a parsed email
    fn extract_sender(parsed: &mail_parser::Message) -> String {
        parsed
            .from()
            .and_then(|addr| addr.first())
            .and_then(|a| a.address())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".into())
    }

    /// Extract readable text from a parsed email
    fn extract_text(parsed: &mail_parser::Message) -> String {
        if let Some(text) = parsed.body_text(0) {
            return text.to_string();
        }
        if let Some(html) = parsed.body_html(0) {
            return Self::strip_html(html.as_ref());
        }
        for part in parsed.attachments() {
            let part: &mail_parser::MessagePart = part;
            if let Some(ct) = MimeHeaders::content_type(part) {
                if ct.ctype() == "text" {
                    if let Ok(text) = std::str::from_utf8(part.contents()) {
                        let name = MimeHeaders::attachment_name(part).unwrap_or("file");
                        return format!("[Attachment: {}]\n{}", name, text);
                    }
                }
            }
        }
        "(no readable content)".to_string()
    }

    /// Connect to IMAP server with TLS and authenticate
    async fn connect_imap(&self) -> Result<ImapSession> {
        let addr = format!("{}:{}", self.config.imap_host, self.config.imap_port);
        debug!("Connecting to IMAP server at {}", addr);

        // Connect TCP
        let tcp = TcpStream::connect(&addr).await?;

        // Establish TLS using rustls
        let certs = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };
        let config = ClientConfig::builder()
            .with_root_certificates(certs)
            .with_no_client_auth();
        let tls_stream: TlsConnector = Arc::new(config).into();
        let sni: DnsName = self.config.imap_host.clone().try_into()?;
        let stream: TlsStream<TcpStream> = tls_stream.connect(sni.into(), tcp).await?;

        // Create IMAP client
        let client = async_imap::Client::new(stream);

        // Login
        let session = client
            .login(&self.config.username, &self.config.password)
            .await
            .map_err(|(e, _)| anyhow!("IMAP login failed: {}", e))?;

        debug!("IMAP login successful");
        Ok(session)
    }

    /// Fetch and process unseen messages from the selected mailbox
    async fn fetch_unseen(&self, session: &mut ImapSession) -> Result<Vec<ParsedEmail>> {
        // Search for unseen messages
        let uids = session.uid_search("UNSEEN").await?;
        if uids.is_empty() {
            return Ok(Vec::new());
        }

        debug!("Found {} unseen messages", uids.len());

        let mut results = Vec::new();
        let uid_set: String = uids
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join(",");

        // Fetch message bodies
        let messages = session.uid_fetch(&uid_set, "RFC822").await?;
        let messages: Vec<Fetch> = messages.try_collect().await?;

        for msg in messages {
            let uid = msg.uid.unwrap_or(0);
            if let Some(body) = msg.body() {
                if let Some(parsed) = MessageParser::default().parse(body) {
                    let sender = Self::extract_sender(&parsed);
                    let subject = parsed.subject().unwrap_or("(no subject)").to_string();
                    let body_text = Self::extract_text(&parsed);
                    let content = format!("Subject: {}\n\n{}", subject, body_text);
                    let msg_id = parsed
                        .message_id()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("gen-{}", Uuid::new_v4()));

                    #[allow(clippy::cast_sign_loss)]
                    let ts = parsed
                        .date()
                        .map(|d| {
                            let naive = chrono::NaiveDate::from_ymd_opt(
                                d.year as i32,
                                u32::from(d.month),
                                u32::from(d.day),
                            )
                            .and_then(|date| {
                                date.and_hms_opt(
                                    u32::from(d.hour),
                                    u32::from(d.minute),
                                    u32::from(d.second),
                                )
                            });
                            naive.map_or(0, |n| n.and_utc().timestamp() as u64)
                        })
                        .unwrap_or_else(|| {
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0)
                        });

                    results.push(ParsedEmail {
                        _uid: uid,
                        msg_id,
                        sender,
                        content,
                        timestamp: ts,
                    });
                }
            }
        }

        // Mark fetched messages as seen
        if !results.is_empty() {
            let _ = session
                .uid_store(&uid_set, "+FLAGS (\\Seen)")
                .await?
                .try_collect::<Vec<_>>()
                .await;
        }

        Ok(results)
    }

    /// Run the IDLE loop, returning when a new message arrives or timeout
    /// Note: IDLE consumes the session and returns it via done()
    async fn wait_for_changes(
        &self,
        session: ImapSession,
    ) -> Result<(IdleWaitResult, ImapSession)> {
        let idle_timeout = Duration::from_secs(self.config.idle_timeout_secs);

        // Start IDLE mode - this consumes the session
        let mut idle = session.idle();
        idle.init().await?;

        debug!("Entering IMAP IDLE mode");

        // wait() returns (future, stop_source) - we only need the future
        let (wait_future, _stop_source) = idle.wait();

        // Wait for server notification or timeout
        let result = timeout(idle_timeout, wait_future).await;

        match result {
            Ok(Ok(response)) => {
                debug!("IDLE response: {:?}", response);
                // Done with IDLE, return session to normal mode
                let session = idle.done().await?;
                let wait_result = match response {
                    IdleResponse::NewData(_) => IdleWaitResult::NewMail,
                    IdleResponse::Timeout => IdleWaitResult::Timeout,
                    IdleResponse::ManualInterrupt => IdleWaitResult::Interrupted,
                };
                Ok((wait_result, session))
            }
            Ok(Err(e)) => {
                // Try to clean up IDLE state
                let _ = idle.done().await;
                Err(anyhow!("IDLE error: {}", e))
            }
            Err(_) => {
                // Timeout - RFC 2177 recommends restarting IDLE every 29 minutes
                debug!("IDLE timeout reached, will re-establish");
                let session = idle.done().await?;
                Ok((IdleWaitResult::Timeout, session))
            }
        }
    }

    /// Main IDLE-based listen loop with automatic reconnection
    async fn listen_with_idle(&self, tx: mpsc::Sender<ChannelMessage>) -> Result<()> {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            match self.run_idle_session(&tx).await {
                Ok(()) => {
                    // Clean exit (channel closed)
                    return Ok(());
                }
                Err(e) => {
                    error!(
                        "IMAP session error: {}. Reconnecting in {:?}...",
                        e, backoff
                    );
                    sleep(backoff).await;
                    // Exponential backoff with cap
                    backoff = std::cmp::min(backoff * 2, max_backoff);
                }
            }
        }
    }

    /// Run a single IDLE session until error or clean shutdown
    async fn run_idle_session(&self, tx: &mpsc::Sender<ChannelMessage>) -> Result<()> {
        // Connect and authenticate
        let mut session = self.connect_imap().await?;

        // Select the mailbox
        session.select(&self.config.imap_folder).await?;
        info!(
            "Email IDLE listening on {} (instant push enabled)",
            self.config.imap_folder
        );

        // Check for existing unseen messages first
        self.process_unseen(&mut session, tx).await?;

        loop {
            // Enter IDLE and wait for changes (consumes session, returns it via result)
            match self.wait_for_changes(session).await {
                Ok((IdleWaitResult::NewMail, returned_session)) => {
                    debug!("New mail notification received");
                    session = returned_session;
                    self.process_unseen(&mut session, tx).await?;
                }
                Ok((IdleWaitResult::Timeout, returned_session)) => {
                    // Re-check for mail after IDLE timeout (defensive)
                    session = returned_session;
                    self.process_unseen(&mut session, tx).await?;
                }
                Ok((IdleWaitResult::Interrupted, _)) => {
                    info!("IDLE interrupted, exiting");
                    return Ok(());
                }
                Err(e) => {
                    // Connection likely broken, need to reconnect
                    return Err(e);
                }
            }
        }
    }

    /// Fetch unseen messages and send to channel
    async fn process_unseen(
        &self,
        session: &mut ImapSession,
        tx: &mpsc::Sender<ChannelMessage>,
    ) -> Result<()> {
        let messages = self.fetch_unseen(session).await?;

        for email in messages {
            // Check allowlist
            if !self.is_sender_allowed(&email.sender) {
                warn!("Blocked email from {}", email.sender);
                continue;
            }

            let is_new = {
                let mut seen = self.seen_messages.lock().await;
                seen.insert(email.msg_id.clone())
            };
            if !is_new {
                continue;
            }

            let msg = ChannelMessage {
                id: email.msg_id,
                reply_target: email.sender.clone(),
                sender: email.sender,
                content: email.content,
                channel: "email".to_string(),
                timestamp: email.timestamp,
                thread_ts: None,
            };

            if tx.send(msg).await.is_err() {
                // Channel closed, exit cleanly
                return Ok(());
            }
        }

        Ok(())
    }

    fn create_smtp_transport(&self) -> Result<SmtpTransport> {
        let creds = Credentials::new(self.config.username.clone(), self.config.password.clone());
        let transport = if self.config.smtp_tls {
            SmtpTransport::relay(&self.config.smtp_host)?
                .port(self.config.smtp_port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::builder_dangerous(&self.config.smtp_host)
                .port(self.config.smtp_port)
                .credentials(creds)
                .build()
        };
        Ok(transport)
    }
}

/// Internal struct for parsed email data
struct ParsedEmail {
    _uid: u32,
    msg_id: String,
    sender: String,
    content: String,
    timestamp: u64,
}

/// Result from waiting on IDLE
enum IdleWaitResult {
    NewMail,
    Timeout,
    Interrupted,
}

#[async_trait]
impl Channel for EmailChannel {
    fn name(&self) -> &str {
        "email"
    }

    async fn send(&self, message: &SendMessage) -> Result<()> {
        // Use explicit subject if provided, otherwise fall back to legacy parsing or default.
        let (subject, body) = if let Some(ref subj) = message.subject {
            (subj.as_str(), message.content.as_str())
        } else if message.content.starts_with("Subject: ") {
            if let Some(pos) = message.content.find('\n') {
                (&message.content[9..pos], message.content[pos + 1..].trim())
            } else {
                ("RedClaw Message", message.content.as_str())
            }
        } else {
            ("RedClaw Message", message.content.as_str())
        };

        let email = Message::builder()
            .from(self.config.from_address.parse()?)
            .to(message.recipient.parse()?)
            .subject(subject)
            .body(body.to_string())?;

        let transport = self.create_smtp_transport()?;
        transport.send(&email)?;
        info!("Email sent to {}", message.recipient);
        Ok(())
    }

    async fn listen(&self, tx: mpsc::Sender<ChannelMessage>) -> Result<()> {
        info!(
            "Starting email channel with IDLE support on {}",
            self.config.imap_folder
        );
        self.listen_with_idle(tx).await
    }

    async fn health_check(&self) -> bool {
        // Fully async health check - attempt IMAP connection
        match timeout(Duration::from_secs(10), self.connect_imap()).await {
            Ok(Ok(mut session)) => {
                let _ = session.logout().await;
                true
            }
            Ok(Err(e)) => {
                debug!("Health check failed: {}", e);
                false
            }
            Err(_) => {
                debug!("Health check timed out");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundedSeenSet, EmailChannel, EmailConfig};

    #[test]
    fn default_idle_timeout_is_29_minutes() {
        assert_eq!(super::default_idle_timeout(), 1740);
    }

    #[tokio::test]
    async fn seen_messages_starts_empty() {
        let channel = EmailChannel::new(EmailConfig::default());
        let seen = channel.seen_messages.lock().await;
        assert_eq!(seen.len(), 0);
    }

    #[tokio::test]
    async fn seen_messages_tracks_unique_ids() {
        let channel = EmailChannel::new(EmailConfig::default());
        let mut seen = channel.seen_messages.lock().await;

        assert!(seen.insert("first-id".to_string()));
        assert!(!seen.insert("first-id".to_string()));
        assert_eq!(seen.len(), 1);
    }

    #[test]
    fn idle_timeout_deserializes_explicit_value() {
        let json = r#"{
            "imap_host": "imap.test.com",
            "smtp_host": "smtp.test.com",
            "username": "user",
            "password": "pass",
            "from_address": "bot@test.com",
            "idle_timeout_secs": 900
        }"#;
        let config: EmailConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.idle_timeout_secs, 900);
    }

    #[test]
    fn idle_timeout_deserializes_legacy_poll_interval_alias() {
        let json = r#"{
            "imap_host": "imap.test.com",
            "smtp_host": "smtp.test.com",
            "username": "user",
            "password": "pass",
            "from_address": "bot@test.com",
            "poll_interval_secs": 120
        }"#;
        let config: EmailConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.idle_timeout_secs, 120);
    }

    #[test]
    fn idle_timeout_propagates_to_channel() {
        let config = EmailConfig {
            idle_timeout_secs: 600,
            ..Default::default()
        };
        let channel = EmailChannel::new(config);
        assert_eq!(channel.config.idle_timeout_secs, 600);
    }

    #[test]
    fn bounded_seen_set_insert_and_contains() {
        let mut set = BoundedSeenSet::new(10);
        assert!(set.insert("a".into()));
        assert!(set.contains("a"));
        assert!(!set.contains("b"));
    }

    #[test]
    fn bounded_seen_set_rejects_duplicates() {
        let mut set = BoundedSeenSet::new(10);
        assert!(set.insert("a".into()));
        assert!(!set.insert("a".into()));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn bounded_seen_set_evicts_oldest_at_capacity() {
        let mut set = BoundedSeenSet::new(3);
        set.insert("a".into());
        set.insert("b".into());
        set.insert("c".into());
        assert_eq!(set.len(), 3);

        // Inserting a 4th should evict "a"
        set.insert("d".into());
        assert_eq!(set.len(), 3);
        assert!(!set.contains("a"), "oldest entry should be evicted");
        assert!(set.contains("b"));
        assert!(set.contains("c"));
        assert!(set.contains("d"));
    }

    #[test]
    fn bounded_seen_set_evicts_in_fifo_order() {
        let mut set = BoundedSeenSet::new(2);
        set.insert("first".into());
        set.insert("second".into());
        set.insert("third".into());
        assert!(!set.contains("first"));
        assert!(set.contains("second"));
        assert!(set.contains("third"));

        set.insert("fourth".into());
        assert!(!set.contains("second"));
        assert!(set.contains("third"));
        assert!(set.contains("fourth"));
    }

    #[test]
    fn bounded_seen_set_capacity_one() {
        let mut set = BoundedSeenSet::new(1);
        set.insert("a".into());
        assert!(set.contains("a"));

        set.insert("b".into());
        assert!(!set.contains("a"));
        assert!(set.contains("b"));
        assert_eq!(set.len(), 1);
    }
}
