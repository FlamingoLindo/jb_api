use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::info;
use sea_orm::prelude::Decimal;
use std::fs;

pub struct Mailer;

impl Mailer {
    fn smtp_credentials() -> (String, String, String) {
        let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        (smtp_host, smtp_username, smtp_password)
    }

    fn build_mailer(
        smtp_host: &str,
        smtp_username: &str,
        smtp_password: &str,
    ) -> Result<SmtpTransport, Box<dyn std::error::Error>> {
        let credentials = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());
        Ok(SmtpTransport::relay(smtp_host)?
            .credentials(credentials)
            .build())
    }

    fn smtp_from(smtp_username: &str) -> Result<Mailbox, Box<dyn std::error::Error>> {
        Ok(Mailbox::new(Some("JB".to_owned()), smtp_username.parse()?))
    }

    fn load_budget_template(
        client_name: &str,
        total_price: &Decimal,
    ) -> Result<String, std::io::Error> {
        let template = fs::read_to_string("assets/emails/budget.html")?;
        let now = chrono::Local::now().format("%d/%m/%Y").to_string();
        Ok(template
            .replace("{{client_name}}", client_name)
            .replace("{{total_price}}", &total_price.normalize().to_string())
            .replace("{{date}}", &now))
    }

    fn load_password_template(username: &str, token: &str) -> Result<String, std::io::Error> {
        let template = fs::read_to_string("assets/emails/password.html")?;
        Ok(template
            .replace("{{username}}", username)
            .replace("{{token}}", token))
    }

    pub fn send_budget(
        recipient: &str,
        client_name: &str,
        total_price: &Decimal,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (smtp_host, smtp_username, smtp_password) = Self::smtp_credentials();
        let body = Self::load_budget_template(client_name, total_price)?;

        let html_part = SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(body);

        let file_bytes = fs::read(file_path)?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("budget.pdf")
            .to_owned();

        let attachment = Attachment::new(file_name).body(file_bytes, "application/pdf".parse()?);

        let multipart = MultiPart::mixed()
            .singlepart(html_part)
            .singlepart(attachment);

        let email = Message::builder()
            .from(Self::smtp_from(&smtp_username)?)
            .to(Mailbox::new(None, recipient.parse()?))
            .subject("Orçamento - Ferros e Aços JB")
            .multipart(multipart)?;

        let mailer = Self::build_mailer(&smtp_host, &smtp_username, &smtp_password)?;
        mailer.send(&email)?;

        info!("Budget email sent to: {}", recipient);
        Ok(())
    }

    pub fn send_forgot_password(
        recipient: &str,
        username: &str,
        token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (smtp_host, smtp_username, smtp_password) = Self::smtp_credentials();
        let body = Self::load_password_template(username, token)?;

        let html_part = SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(body);

        let email = Message::builder()
            .from(Self::smtp_from(&smtp_username)?)
            .to(Mailbox::new(None, recipient.parse()?))
            .subject("Redefinição de Senha - Ferros e Aços JB")
            .singlepart(html_part)?;

        let mailer = Self::build_mailer(&smtp_host, &smtp_username, &smtp_password)?;
        mailer.send(&email)?;

        info!("Password reset email sent to: {}", recipient);
        Ok(())
    }
}
