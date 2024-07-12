use clap::{App, Arg, SubCommand};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use reqwest::Client;
use serde_json::json;


async fn send_email(to: &str, subject: &str, body: &str) {
    let email = Message::builder()
        .from("your-email@example.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    let creds = Credentials::new("smtp-username".to_string(), "smtp-password".to_string());

    let mailer = SmtpTransport::relay("smtp.example.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
}

async fn send_slack_message(webhook_url: &str, message: &str) {
    let client = Client::new();
    let res = client.post(webhook_url)
        .json(&json!({ "text": message }))
        .send()
        .await;

    match res {
        Ok(_) => println!("Message sent to Slack!"),
        Err(e) => eprintln!("Could not send Slack message: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("Notifier")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Sends notifications")
        .subcommand(
            SubCommand::with_name("email")
                .about("Send an email notification")
                .arg(Arg::with_name("to").required(true).help("Recipient email address"))
                .arg(Arg::with_name("subject").required(true).help("Email subject"))
                .arg(Arg::with_name("body").required(true).help("Email body")),
        )
        .subcommand(
            SubCommand::with_name("slack")
                .about("Send a Slack notification")
                .arg(Arg::with_name("webhook_url").required(true).help("Slack webhook URL"))
                .arg(Arg::with_name("message").required(true).help("Message to send")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("email") {
        let to = matches.value_of("to").unwrap();
        let subject = matches.value_of("subject").unwrap();
        let body = matches.value_of("body").unwrap();
        send_email(to, subject, body).await;
    } else if let Some(matches) = matches.subcommand_matches("slack") {
        let webhook_url = matches.value_of("webhook_url").unwrap();
        let message = matches.value_of("message").unwrap();
        send_slack_message(webhook_url, message).await;
    }
}
