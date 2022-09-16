const image_url = `/file/${config.PAGE_ID}`;

window.addEventListener("load", () => {
    let request = new XMLHttpRequest();
    request.onload = function() {
        const content_type = this.getResponseHeader("content-type");

        if (content_type != "image/svg" && content_type != "image/svg+xml") is_not_svg();

        let response = this.responseXML,
            children = response.children[0].children;

        for (let i = 0; i < children.length; i++)
            if (children[i].tagName == "text")
                document.querySelector("span#text_editors").innerHTML +=
                `<input type="text" value="${children[i].innerHTML}" oninput="update_text(${i}, this.value)" /><br/>`;

        set_image(response);
    }

    request.onerror = is_not_svg;

    request.open("GET", image_url);
    request.send();
});

function update_text(child, value) {
    let edit_img = document.querySelector("img#editing_image");
    let xml_src = new DOMParser().parseFromString(edit_img.getAttribute("src").substring(24), "text/xml");

    xml_src.children[0].children[child].innerHTML = value;

    set_image(xml_src);
}

function set_image(xml) {
    let string_src = new XMLSerializer().serializeToString(xml);
    document.querySelector("img#editing_image").setAttribute("src", `data:image/svg+xml;utf8,${string_src}`);
}

function is_not_svg() {
    alert("Could not edit this image because it either does not exist or is not a .svg file");
    window.close();
}

function upload() {
    let
        request = new XMLHttpRequest(),
        svg_src = document.querySelector("img#editing_image").getAttribute("src"),
        file = new File([svg_src.substring(24)], "edited.svg", { type: "image/svg+xml" }),
        form_data = new FormData();

    form_data.append("monitor_id", config.PAGE_ID);
    form_data.append("monitor_image", file);

    request.open("POST", "/upload_image");
    request.send(form_data);
}