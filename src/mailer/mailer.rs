use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::info;
use sea_orm::prelude::Decimal;
use std::fs;

pub struct Mailer;

impl Mailer {
    fn load_email_template(
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

    pub fn send_budget(
        recipient: &str,
        client_name: &str,
        total_price: &Decimal,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = Self::load_email_template(client_name, total_price)?;

        let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

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
            .from(Mailbox::new(
                Some("JB".to_owned()),
                smtp_username.clone().parse()?,
            ))
            .to(Mailbox::new(None, recipient.parse()?))
            .subject("Orçamento - Ferros e Aços JB")
            .multipart(multipart)?;

        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&smtp_host)?
            .credentials(credentials)
            .build();

        mailer.send(&email)?;
        info!("Budget email sent to: {}", recipient);
        Ok(())
    }
}
