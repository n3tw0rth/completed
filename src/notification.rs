use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::error::CompletedError;
use crate::CompletedResult;

pub struct Notification<'b> {
    config: &'b super::Config,
    profiles: &'b [String],
    title: String,
    msg: String,
}

impl<'b> Notification<'b> {
    pub fn new(
        config: &'b super::Config,
        profiles: &'b [String],
        name: Option<&str>,
        title: String,
        msg: String,
    ) -> Notification<'b> {
        let title = match name {
            Some(name) => format!("{name}: {title}"),
            None => title,
        };

        Notification {
            config,
            profiles,
            title,
            msg,
        }
    }

    /// Send desktop notifications using the crate `notify_rust`
    /// desktop notification is the default preference
    pub async fn send_desktop(&self) -> CompletedResult<()> {
        notify_rust::Notification::new()
            .summary(&self.title)
            .body(&self.msg)
            .show()
            .map_err(|e| {
                CompletedError::NotificationError(format!(
                    "could not show desktop notification: {e}"
                ))
            })?;

        Ok(())
    }

    /// Send notification to g chat using the webhooks
    pub async fn send_gchat(&self, gchat_config: &super::GChatConfig) -> CompletedResult<()> {
        let response = reqwest::Client::new()
            .post(&gchat_config.webhook)
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&serde_json::json!({
                "text": format!("{}\n*{}*", self.title, self.msg),
            }))
            .send()
            .await
            .map_err(|e| {
                CompletedError::NotificationError(format!("could not reach gchat webhook: {e}"))
            })?;

        if !response.status().is_success() {
            return Err(CompletedError::NotificationError(format!(
                "gchat webhook returned {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Send emails using the smtp
    pub async fn send_mail(&self, email_config: &super::EmailConfig) -> CompletedResult<()> {
        let email: Message = Message::builder()
            .from(email_config.from.parse().map_err(|e| {
                CompletedError::NotificationError(format!(
                    "invalid 'from' address '{}': {e}",
                    email_config.from
                ))
            })?)
            .to(email_config.to.parse().map_err(|e| {
                CompletedError::NotificationError(format!(
                    "invalid 'to' address '{}': {e}",
                    email_config.to
                ))
            })?)
            .subject(self.title.clone())
            .body(self.msg.clone())
            .map_err(|e| {
                CompletedError::NotificationError(format!("could not build email: {e}"))
            })?;

        let creds: Credentials = Credentials::new(
            email_config.username.clone(),
            email_config.password.clone(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&email_config.host)
            .map_err(|e| {
                CompletedError::NotificationError(format!(
                    "invalid smtp host '{}': {e}",
                    email_config.host
                ))
            })?
            .port(email_config.port)
            .credentials(creds)
            .build();

        mailer.send(email).await.map_err(|e| {
            CompletedError::NotificationError(format!("could not send email: {e}"))
        })?;

        Ok(())
    }

    /// Parse the preferences defined on the configuration file and send noifications to
    /// destinations accordingly
    pub async fn send(&self) -> CompletedResult<()> {
        let mut failures: Vec<String> = Vec::new();

        for profile in self.profiles {
            let Some(profile_config) = self.config.profiles.get(profile) else {
                let err = CompletedError::UnknownProfile(profile.clone());
                eprintln!("completed: {err}");
                failures.push(err.to_string());
                continue;
            };

            for destination in &profile_config.sendto {
                if let Err(err) = self.send_to(destination).await {
                    eprintln!("completed: failed to notify '{destination}': {err}");
                    failures.push(format!("{destination}: {err}"));
                }
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(CompletedError::NotificationError(failures.join("; ")))
        }
    }

    /// Dispatch to the right channel for a `sendto` entry, e.g. "desktop",
    /// "email.default" or "gchat.work"
    async fn send_to(&self, destination: &str) -> CompletedResult<()> {
        match destination.split_once('.') {
            Some(("email", name)) => {
                let email_config = self
                    .config
                    .email
                    .as_ref()
                    .and_then(|configs| configs.get(name))
                    .ok_or_else(|| CompletedError::UnknownDestination(destination.to_string()))?;
                self.send_mail(email_config).await
            }
            Some(("gchat", name)) => {
                let gchat_config = self
                    .config
                    .gchat
                    .as_ref()
                    .and_then(|configs| configs.get(name))
                    .ok_or_else(|| CompletedError::UnknownDestination(destination.to_string()))?;
                self.send_gchat(gchat_config).await
            }
            None if destination == "desktop" => self.send_desktop().await,
            _ => Err(CompletedError::UnknownDestination(destination.to_string())),
        }
    }

    /// Updates the message to indicate the notification is a trigger and calls self.send()
    pub async fn send_trigger(&mut self) -> CompletedResult<()> {
        self.msg = format!("Triggers invoked {}", self.msg);
        self.send().await?;
        Ok(())
    }
}
