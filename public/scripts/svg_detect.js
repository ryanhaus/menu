function svg_detect(element, id) {
    let request = new XMLHttpRequest();
    request.onload = function() {
        let content_type = this.getResponseHeader("content-type");
        if (content_type != "image/svg" && content_type != "image/svg+xml") {
            element.parentElement.children[1].style.setProperty("display", "none");
        }
    }

    request.open("HEAD", `/file/${id}`);
    request.send();
};