var ws = new WebSocket("ws://192.168.1.246:7778");

ws.onmessage = (m) => {
    let content = JSON.parse(m.data); // parse the JSON

    switch (content.message_type) {
        case "RELOAD":
            document.querySelector("img#main_image").src = `/img/${page_id}?${new Date().getTime()}`;
            break;
    }
}

ws.onclose = ws_close;
ws.onopen = ws_open;

function ws_open() {
    ws.send(JSON.stringify({ message_type: "GIVE_PAGE", page: page_id }));
}

function ws_close() {
    ws = new WebSocket("ws://192.168.1.246:7778");

    ws.onclose = ws_close;
    ws.onopen = ws_open;
}