use std::{io::Read, time::SystemTime};

use chrono::{DateTime, Local};
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

use crate::path::{CAMERA_PHOTO_DIR, PHOTO_STRIP_DIR};

#[tauri::command]
pub async fn send_mail(mail_address: String, photos: Vec<String>, strip_image: Vec<u8>) {
    let now: DateTime<Local> = SystemTime::now().into();
    let img_name = format!("{}.png", now.format("%d-%m-%Y %H-%M-%S"));

    let img_path = PHOTO_STRIP_DIR.get().unwrap().join(img_name);

    println!("Writing to {}", img_path.display());

    std::fs::write(&img_path, &strip_image).unwrap();

    println!("Wrote to {}", img_path.display());

    println!("Building message");

    let own_mail_address = std::env::var("PHOTOBOOTH_MAIL_ADDRESS").unwrap();
    let own_mail_password = std::env::var("PHOTOBOOTH_MAIL_PASSWORD").unwrap();

    let smtp_host = std::env::var("PHOTOBOOTH_MAIL_SMTP_HOST").unwrap();
    let smtp_port = std::env::var("PHOTOBOOTH_MAIL_SMTP_PORT")
        .unwrap()
        .parse()
        .unwrap();

    let mut message = MessageBuilder::new()
        .from(("KMG Fotobox", own_mail_address.as_str()))
        .to(mail_address)
        .subject("KMG Fotobox")
        .text_body(format!(
            "Anbei die Fotos, die ihr am {} gemacht habt",
            now.format("%d.%m.%Y um %H:%M")
        ))
        .attachment("image/png", "Foto-Streifen.png", strip_image);

    for (idx, photo) in photos.into_iter().enumerate() {
        println!("Attaching photo {}", idx);

        let photo_path = CAMERA_PHOTO_DIR.get().unwrap().join(&photo);
        let mime_type = mime_guess::from_path(&photo_path).first_or_octet_stream();
        let mut photo_file = std::fs::File::open(photo_path).unwrap();

        let mut buf = Vec::new();
        photo_file.read_to_end(&mut buf).unwrap();

        message = message.attachment(mime_type.to_string(), photo, buf);
    }

    println!("Sending message");

    SmtpClientBuilder::new(smtp_host.as_str(), smtp_port)
        .implicit_tls(true)
        .credentials((own_mail_address.as_str(), own_mail_password.as_str()))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();

    println!("Message sent");
}
