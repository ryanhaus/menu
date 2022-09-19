const image_url = `/file/${config.PAGE_ID}`;

var svg;
let texts = [], images = [];

window.addEventListener("load", () => {
    let request = new XMLHttpRequest();
    request.onload = function() {
        const content_type = this.getResponseHeader("content-type");
        if (content_type != "image/svg" && content_type != "image/svg+xml") is_not_svg();
        
        svg = this.responseXML;

        // parse texts
        Array.from(svg.getElementsByTagName("text")).forEach((element, i) => {
            if (element.children.length != 0)
                element = element.children[0];

            texts.push(element);
            
            document.querySelector("span#text_editors").innerHTML +=
                `<input type="text" value="${element.innerHTML}" oninput="update_text(${i}, this.value)" /><br/>`;
        });

        // parse images
        Array.from(svg.getElementsByTagName("image")).forEach((element, i) => {
            images.push(element);

            document.querySelector("span#image_editors").innerHTML +=
                `<img style="max-width: 512px;" src="${element.attributes["xlink:href"].value}" id="img_${i}" /><br/>Replace: <input type="file" accept="image/*" oninput="update_image(${i}, this.files[0])" /><hr />`;
        });
                
        set_image();
    }

    request.onerror = is_not_svg;

    request.open("GET", image_url);
    request.send();
});

function update_text(index, value) {
    texts[index].innerHTML = value;
    set_image();
}

function update_image(index, value) {
    let reader = new FileReader();

    reader.onload = (e) => {
        images[index].setAttribute("xlink:href", e.target.result);
        document.querySelector(`img#img_${index}`).setAttribute("src", e.target.result);

        set_image();
    }

    reader.readAsDataURL(value);
}

function set_image() {
    let string_src = new XMLSerializer().serializeToString(svg);
    document.querySelector("img#editing_image").setAttribute("src", `data:image/svg+xml;utf8,${encodeURIComponent(string_src)}`);
}

function is_not_svg() {
    alert("Could not edit this image because it either does not exist or is not a .svg file");
    window.close();
}

function upload() {
    let
        request = new XMLHttpRequest(),
        svg_src = document.querySelector("img#editing_image").getAttribute("src"),
        file = new File([decodeURIComponent(svg_src.substring(24))], "edited.svg", { type: "image/svg+xml" }),
        form_data = new FormData();

    form_data.append("monitor_id", config.PAGE_ID);
    form_data.append("monitor_image", file);

    request.open("POST", "/upload_image");
    request.send(form_data);
}