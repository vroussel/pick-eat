function showToast(message, type='error') {
    const alert = document.createElement('div');
    alert.className = `alert alert-${type}`;
    alert.innerHTML = `${message}`;

    const toast = document.getElementById('toast');
    if (toast) {
        toast.appendChild(alert);
        setTimeout(() => alert.remove(), 3000);
    }
}

document.body.addEventListener('htmx:responseError', function(evt) {
    showToast("Une erreur s'est produite, désolé!<br>Veuillez rééssayer plus tard.")
});
