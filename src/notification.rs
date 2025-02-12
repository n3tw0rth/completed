use futures::stream::{self, StreamExt};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Notification<'b> {
    config: &'b super::Config,
    profiles: &'b Vec<String>,
    title: String,
    msg: String,
}

impl<'b> Notification<'b> {
    pub fn new(
        config: &'b super::Config,
        profiles: &'b Vec<String>,
        title: String,
        msg: String,
    ) -> Notification<'b> {
        Notification {
            config,
            profiles,
            title,
            msg,
        }
    }

    pub async fn send_desktop(&self) -> anyhow::Result<()> {
        let _ = notify_rust::Notification::new()
            .summary(&self.title)
            .body(&self.msg)
            .show();

        Ok(())
    }

    pub async fn send_gchat(&self, gchat_config: &super::GChatConfig) -> anyhow::Result<()> {
        let _ = reqwest::Client::new()
            .post(gchat_config.webhook.to_string())
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&serde_json::json!({
                "text": format!("{}\n*{}*",self.title,self.msg),
            }))
            .send()
            .await?;

        Ok(())
    }

    pub async fn send_mail(&self, email_config: &super::EmailConfig) -> anyhow::Result<()> {
        let email: Message = Message::builder()
            .from(email_config.from.parse().unwrap())
            .to(email_config.to.parse().unwrap())
            .subject(self.title.clone())
            .body(self.msg.to_string())
            .unwrap();

        let creds: Credentials = Credentials::new(
            email_config.username.to_string(),
            email_config.password.to_string(),
        );

        let mailer: SmtpTransport = SmtpTransport::relay(&email_config.host.to_string())
            .unwrap()
            .port(email_config.port)
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => {}
            Err(e) => panic!("Could not send email: {:?}", e),
        }

        Ok(())
    }

    pub async fn send(&self) -> anyhow::Result<()> {
        stream::iter(self.profiles)
            .for_each(|item| async move {
                let send_to = &self.config.profiles.get(item).unwrap().sendto;

                let _ = stream::iter(send_to)
                    .for_each(|cfg| async move {
                        if let [mode, name] = cfg.split(".").collect::<Vec<_>>()[0..] {
                            match mode {
                                "email" => {
                                    let _ = self
                                        .send_mail(
                                            self.config.email.as_ref().unwrap().get(name).unwrap(),
                                        )
                                        .await;
                                }
                                "gchat" => {
                                    let _ = self
                                        .send_gchat(
                                            self.config.gchat.as_ref().unwrap().get(name).unwrap(),
                                        )
                                        .await;
                                }
                                _ => {}
                            }
                        } else {
                            match cfg.as_str() {
                                "desktop" => {
                                    self.send_desktop().await.unwrap();
                                }
                                _ => {
                                    println!("Please update the profile configuration for {}", cfg)
                                }
                            };
                        }
                    })
                    .await;
            })
            .await;

        Ok(())
    }

    pub async fn send_trigger(&mut self) -> anyhow::Result<()> {
        // expect a list of triggers found and will update the notification massage based on that
        self.msg = format!("Triggers invoked {}", self.msg);
        self.send().await?;
        Ok(())
    }
}
