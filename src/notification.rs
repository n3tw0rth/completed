use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Notification<'b> {
    config: &'b super::Config,
    profiles: &'b Vec<String>,
    title: String,
    msg: String,
}

pub struct EmailClient<'a> {
    from: &'a String,
    to: &'a String,
    host: &'a String,
    api_key: &'a Option<String>,
    msg: &'a String,
    title: &'a String,
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

    pub async fn read_local_config() {}

    pub async fn send_desktop(&self) -> anyhow::Result<()> {
        let _ = notify_rust::Notification::new()
            .summary(&self.title)
            .body(&self.msg)
            .show();

        Ok(())
    }

    pub async fn send_mail(&self, email_config: &super::EmailConfig) -> anyhow::Result<()> {
        let email: Message = Message::builder()
            .from(email_config.from.parse().unwrap())
            .to(email_config.to.parse().unwrap())
            .subject(self.title.clone())
            .body(self.msg.to_string())
            .unwrap();

        let creds: Credentials =
            Credentials::new("apikey".to_string(), email_config.api_key.clone().unwrap());

        // Open a remote connection to gmail
        let mailer: SmtpTransport = SmtpTransport::relay(&email_config.host.to_string())
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }

        Ok(())
    }

    pub async fn send(&self) -> anyhow::Result<()> {
        self.send_mail(self.config.email.get("default").unwrap())
            .await
            .unwrap();

        self.send_desktop().await.unwrap();

        Ok(())
    }
}

impl<'a> EmailClient<'a> {
    pub fn new(
        from: &'a String,
        to: &'a String,
        host: &'a String,
        api_key: &'a Option<String>,
        msg: &'a String,
        title: &'a String,
    ) -> EmailClient<'a> {
        EmailClient {
            from,
            to,
            host,
            api_key,
            title,
            msg,
        }
    }
}
