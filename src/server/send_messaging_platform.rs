use std::io::Write;
use std::net::TcpStream;

pub fn send_messaging_platform(mut stream: TcpStream, filename: String) {
    let send = format!("<!DOCTYPE html>
    <head>
        <title>Filecord - {}</title>
        <meta property=\"og:type\" content=\"website\" />
        <meta name=\"description\" content=\"Filecord is a program to share large files for free using discord\" />
        <meta content=\"#1E293B\" name=\"theme-color\" />
        <link rel=\"apple-touch-icon\" sizes=\"180x180\" href=\"/image/favicon/apple-touch-icon.png\">
        <link rel=\"icon\" type=\"image/png\" sizes=\"32x32\" href=\"/image/favicon/favicon-32x32.png\">
        <link rel=\"icon\" type=\"image/png\" sizes=\"16x16\" href=\"/image/favicon/favicon-16x16.png\">
        <link rel=\"manifest\" href=\"/image/favicon/site.webmanifest\">
        <meta property=\"og:image\" content=\"/image/favicon/filecord.png\" />
    </head>
    <body>", filename);
    stream.write(format!("HTTP/1.1 200 Ok\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}", send.len(), send).as_bytes()).unwrap();
}
