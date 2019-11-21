import { $ } from "./dollard";
import { onDashboard } from "./dashboard";

function onLogin(data, status, admin=false) {
    const containerMessage = $.id('page-login').getElementsByClassName('message')[0];

    if (!status) { // Connexion failed
        containerMessage.classList.add('warning');
        containerMessage.innerHTML = `<strong>Tentative de connexion</strong><br>${data.message}`;
    } else {
        containerMessage.classList.add('success');
        containerMessage.innerHTML = '<strong>Connexion approuvée</strong><br>Vous allez être redirigé sous peu.';
        setTimeout(onDashboard, 1000, data, admin);
    }
}

export { onLogin };
