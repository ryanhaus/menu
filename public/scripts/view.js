const ws_addr = `ws://${config.IP_ADDR}:${config.WS_PORT}`;

var ws;
ws_close();

function ws_message(m) {
    let content = JSON.parse(m.data); // parse the JSON

    switch (content.message_type) {
        case "RELOAD":
            document.querySelector("img#main_image").src = `/file/${config.PAGE_ID}?${new Date().getTime()}`;
            break;
    }
}

function ws_open() {
    ws.send(JSON.stringify({ message_type: "GIVE_PAGE", page: config.PAGE_ID }));
}

function ws_close() {
    ws = new WebSocket(ws_addr);

    ws.onmessage = ws_message;
    ws.onclose = ws_close;
    ws.onopen = ws_open;
}