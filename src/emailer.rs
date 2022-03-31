use async_trait::async_trait;

use crate::config::Config;
use crate::core::traits::Notifier;
use crate::error;
use actix_web::web;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

/// Used to send an email notification
pub struct EmailNotifier {
    /// Sender's email
    pub from: String,
    /// Destination email
    pub to: String,
    /// Mail subject
    pub subject: String,
    /// Mail body
    pub body: String,
    /// [Config] object
    pub cfg: web::Data<Config>,
}

#[async_trait()]
impl Notifier for EmailNotifier {
    async fn send(&self) -> Result<(), error::Error> {
        // TODO: Think about encrypting password.
        let creds = Credentials::new(
            self.cfg.email.username.to_string(),
            self.cfg.email.password.to_string(),
        );

        // Prepare a transport
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(self.cfg.email.server.to_string().as_str())
                .unwrap()
                .credentials(creds)
                .build();
        println!("TO: {:?}", self.to);
        // Build a message
        let email = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(self.subject.to_string())
            .body(self.body.to_string())
            .unwrap();

        // Send email
        mailer.send(email).await?;

        Ok(())
    }
}

impl EmailNotifier {
    pub fn new(
        cfg: web::Data<Config>,
        from: String,
        to: String,
        subject: String,
        body: String,
    ) -> Self {
        EmailNotifier {
            from,
            to,
            subject,
            body,
            cfg,
        }
    }
}
