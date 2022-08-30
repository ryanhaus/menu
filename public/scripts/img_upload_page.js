let current_selected = null;

function change_preview(id) {
    const selected_image = document.querySelector(`img#img_${id}`);

    if (current_selected)
        current_selected.style = {};

    selected_image.style.border = "3px solid red";
    current_selected = selected_image;
}

function enlarge(image) {
    document.querySelector("img#enlarged_image").setAttribute("src", image.getAttribute("src"));
    document.querySelector("div#enlarged_image_container").style.display = "initial";
}

window.addEventListener("load", () => {
    change_preview("1");

    const
        image_input = document.querySelector("input#image_input"),
        input_preview = document.querySelector("img#upload_image_preview");
    image_input.onchange = (e) => {
        input_preview.src = URL.createObjectURL(image_input.files[0]);
        input_preview.style.display = "initial";
    };
});