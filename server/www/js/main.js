function init() {
    document.getElementById('fileInput').addEventListener('change', handleFileSelect, false);
}

function handleFileSelect() {
    document.getElementById('image').classList.remove("top-[12px]");
    document.getElementById("uploaded").innerHTML = `Selected: ${document.getElementById('fileInput').files[0].name}`
}

async function upload() {
    if (document.getElementById('fileInput').files.length === 0) {
        send_message("Upload failed", "Please click the upload button and add a file first");
        return;
    }
    if (!document.getElementById('urlInput').value) {
        send_message("Upload failed", "Please add a url to discord webhook url box");
        return;
    }
    if (!/https:\/\/[^.]*\.discord\.com\/api\/webhooks\/[0-9]*\/[^\/]*/.test(document.getElementById('urlInput').value)) {
        send_message("Upload failed", "This is not a correct webhook url it should look something like https://discord.com/api/webhooks/...");
        return;
    }
    let i;
    let start = Date.now();
    let bar = document.getElementById("myBar");
    let file = document.getElementById('fileInput').files[0];
    let url = document.getElementById('urlInput').value;
    let offset = 0;
    let request = await fetch(url);
    let channel = (await request.json())["channel_id"];
    bar.style.width = "1%";
    if (!channel) {
        send_message("Upload failed", "Could not find this webhook url, check again to make sure its right");
        return;
    }
    let returns = [file.name, file.size];
    let index = 0;
    while (file.size > offset) {
        let boundary = "--------";
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        for (i = 0; i < 20; i++) {
            boundary += chars[Math.floor(Math.random() * chars.length)];
        }
        let sends = `--${boundary}\r\nContent-Disposition: form-data; name=\"file1\"; filename=\"${"part_" + index}\"\r\nContent-Type: application/octet-stream\r\n\r\n`;
        let send = new Blob([sends, file.slice(offset, offset + 8388608 - sends.length - 34), `\r\n--${boundary}--`]);
        let response = await fetch(url, {
            method: "POST",
            body: send,
            headers: {
                "content-type": `multipart/form-data; boundary=${boundary}`
            }
        });
        if (response.ok) {
            offset += 8388608 - sends.length - 34;
            returns.push(JSON.parse(await response.text())["attachments"][0]["id"]);
            index++;
            bar.style.width = `${offset / file.size * 100}%`;
        } else {
            await new Promise(r => setTimeout(r, 1000));
        }
    }
    let boundary = "--------";
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    for (i = 0; i < 20; i++) {
        boundary += chars[Math.floor(Math.random() * chars.length)];
    }
    let send = new Blob([`--${boundary}\r\nContent-Disposition: form-data; name=\"file1\"; filename=\"data\"\r\nContent-Type: application/octet-stream\r\n\r\n`, pako.deflate(returns.join("&")), `\r\n--${boundary}--`]);
    let response = await fetch(url, {
        method: "POST",
        body: send,
        headers: {
            "content-type": `multipart/form-data; boundary=${boundary}`
        }
    });
    let convert = [channel, JSON.parse(await response.text())["attachments"][0]["id"]];
    let base = "https://amtitan-sharex.herokuapp.com/";
    let char_list = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    base += char_list[convert[0].length];
    for (i = 0; i < 2; i++) {
        for (let x = 0; x < 11; x++) {
            base += char_list[BigInt(convert[i]) / 62n ** BigInt(x) % 62n];
        }
    }
    send_message("Upload Successful", `Yay! To download the file go to <a class="text-blue-500" href="${base}">${base}</a>`);
    console.log(base);
}

function send_message(title, description) {
    document.getElementById("popup").classList.remove("ease-in", "duration-200", "opacity-0", "pointer-events-none");
    document.getElementById("popup").classList.add("ease-out", "duration-300", "opacity-100");
    document.getElementById("panel").classList.remove("ease-in", "duration-200", "opacity-0", "translate-y-4", "sm:translate-y-0", "sm:scale-95");
    document.getElementById("panel").classList.add("opacity-100", "translate-y-0", "sm:scale-100", "ease-out", "duration-300");
    document.getElementById("modal-title").innerHTML = title;
    document.getElementById("modal-description").innerHTML = description;
}

function remove_message() {
    document.getElementById("popup").classList.add("ease-in", "duration-200", "opacity-0", "pointer-events-none");
    document.getElementById("popup").classList.remove("ease-out", "duration-300", "opacity-100");
    document.getElementById("panel").classList.add("ease-in", "duration-200", "opacity-0", "translate-y-4", "sm:translate-y-0", "sm:scale-95");
    document.getElementById("panel").classList.remove("opacity-100", "translate-y-0", "sm:scale-100", "ease-out", "duration-300");
}