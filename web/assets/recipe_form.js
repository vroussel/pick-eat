let cropper_instance = null
let cropped_file = null
let image_input = document.getElementById('image-input')
let image_preview = document.getElementById('image-preview')
let modal = document.querySelector(".modal")

function reset_input_error(el) {
    el.closest('div').querySelector('.text-error')?.remove()
    el.classList.remove('input-error')
}

image_input.addEventListener('change', function(evt) {
    let f = image_input.files[0]

    let img = modal.querySelector("img")
    if (img === null) {
        img = document.createElement('img')
        img.alt = "Prévisualisation de la photo de la recette, avant crop"
        img.className = "block max-w-full"
        modal.querySelector(".modal-box").prepend(img)
    }
    img.src = URL.createObjectURL(f)

    if (cropper_instance === null) {
        cropper_instance = new Cropper(img, { aspectRatio: 1, viewMode: 2, background: false });
    } else {
        cropper_instance.replace(img.src)
    }
    modal.showModal()
})

function edit_crop() {
    modal.showModal()
}

function validate_crop() {
    console.log('coucou')
    cropper_instance.getCroppedCanvas().toBlob(blob => {
        const mimeType = image_input.files[0]?.type || 'image/jpeg';
        const name = image_input.files[0].name
        cropped_file = new File([blob], name, { type: mimeType });
        let dt = new DataTransfer()
        dt.items.add(cropped_file)
        image_input.files = dt.files
        image_preview.src = URL.createObjectURL(cropped_file)
        image_preview.hidden = false
    })
    modal.close()
}

function cancel_crop() {
    if (cropped_file === null) {
        image_input.value = null
    } else {
        let dt = new DataTransfer()
        dt.items.add(cropped_file)
        image_input.files = dt.files
    }
}
