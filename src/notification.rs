use std::collections::HashMap;

use futures::stream::{self, StreamExt};
use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{header::HeaderMap, Method, Request, Url};
use reqwest::{Client, Proxy};
pub struct Notification<'b> {
    config: &'b super::Config,
    profiles: &'b Vec<String>,
    title: String,
    msg: String,
}

pub struct EmailClient<'a> {
    from: &'a String,
    to: &'a String,
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
        EmailClient::new(
            &email_config.from,
            &email_config.to,
            &email_config.api_key,
            //Some(
            //    "mlsn.77afe80e65161674a1fc9809e1f69d5865c525120108f21a32f29b16a7b12d92".to_string(),
            //),
            &self.msg,
            &self.title,
        )
        .mailer_send()
        .await
        .unwrap();

        Ok(())
    }

    pub async fn send(&self) {
        self.send_mail(self.config.email.get("default").unwrap())
            .await
            .unwrap();

        self.send_desktop().await.unwrap();
    }
}

impl<'a> EmailClient<'a> {
    pub fn new(
        from: &'a String,
        to: &'a String,
        api_key: &'a Option<String>,
        msg: &'a String,
        title: &'a String,
    ) -> EmailClient<'a> {
        EmailClient {
            from,
            to,
            api_key,
            title,
            msg,
        }
    }

    pub async fn smtp(self) {}

    /// Send mail using https://www.mailersend.com/
    ///
    /// Documentation https://developers.mailersend.com/
    ///
    pub async fn mailer_send(self) -> anyhow::Result<()> {
        let proxy = Proxy::http("http://127.0.0.1:8081").unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        //-H ': ' \
        headers.insert(
            "X-Requested-With",
            HeaderValue::from_static("XMLHttpRequest"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key.clone().unwrap())).unwrap(),
        );

        let payload = serde_json::json!({
            "from": {
                "email": self.from
            },
            "to": [
                {
                    "email": self.to
                }
            ],
            "subject": self.title,
            "text": self.msg,
            "html": self.msg
        })
        .to_string();

        let _response = Client::builder()
            .proxy(proxy)
            .build()
            .unwrap()
            .post("https://api.mailersend.com/v1/email")
            .headers(headers)
            .body(payload)
            .send()
            .await
            .unwrap();

        Ok(())
    }
}
