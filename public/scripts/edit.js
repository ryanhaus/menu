const image_url = `/file/${config.PAGE_ID}`;

window.addEventListener("load", () => {
    let request = new XMLHttpRequest();
    request.onload = function() {
        let response = this.responseXML,
            children = response.children[0].children;

        for (let i = 0; i < children.length; i++)
            response.children[0].children[i].innerHTML = "Hello, world!";

        let string_src = new XMLSerializer().serializeToString(response);
        document.querySelector("img#editing_image").setAttribute("src", `data:image/svg+xml;utf8,${string_src}`);
    }

    request.open("GET", image_url);
    request.send();
});