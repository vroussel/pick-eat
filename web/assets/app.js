function reset_input_error(el) {
    el.closest('div').querySelector('.text-error')?.remove()
    el.classList.remove('input-error')
}
