pub fn messaging_platform(response: String) -> bool {
    let messaging_platforms_useragent =
        ["bot", "whatsapp", "snapchat", "messenger", "teams", "skype"];
    for i in response
        .split("\r\n\r\n")
        .nth(0)
        .unwrap()
        .to_lowercase()
        .lines()
    {
        if i.starts_with("user-agent") {
            for x in messaging_platforms_useragent {
                if i.contains(x) {
                    return true;
                }
            }
        }
    }
    false
}