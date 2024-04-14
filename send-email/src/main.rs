
use lettre::{
    message::{header::ContentType, Attachment, Body, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

fn create_mailer() -> SmtpTransport {
    // Get the username and password from the env file
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");


    // Create the credentials
    let creds = Credentials::new(username, password);

    println!("Creds: {:?}", creds);

    SmtpTransport::relay(&(std::env::var("SMTP_SERVER").unwrap()))
        .unwrap()
        .credentials(creds)
        .build()
}

fn send_email() {
    // Build the email
    let receiver = std::env::var("RECEIVER").unwrap();
    let email = Message::builder()
        .from("Sender <sender@gmail.com>".parse().unwrap()) 
        .to(format!("Receiver <{}>", receiver).parse().unwrap())
        .subject("Email Test")
        .body("Hello, this is a test email!".to_string())
        .unwrap();

    let mailer = create_mailer();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }
}



// fn main() {
//     dotenv::dotenv().ok();
//     println!("Sending emails...");

//    send_email();
// //    send_email_with_html();
//     println!("Emails sent!");
// }

fn main() {
    dotenv::dotenv().ok();
    println!("Sending emails...");

    let receiver = std::env::var("RECEIVER").unwrap();
    let email = Message::builder()
        .from("nobody@domain.tld".parse().unwrap())
        .to(format!("Receiver <{}>", receiver).parse().unwrap())
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();
 
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");
    let server = std::env::var("SMTP_SERVER").expect("SMTP_SERVER not set");
    let port = "2525";
    let url = format!("smtp://{username}:{password}@{server}:{port}");
 
    let mailer =
    SmtpTransport::from_url( &url )
        .unwrap().build();
     
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}