import { $ } from "./dollard";
import { onDashboard } from "./dashboard";

function onLogin(message, status) {
    const containerMessage = $.id('page-login').getElementsByClassName('message')[0];
    // Connexion failed
    if (!status) {
        containerMessage.classList.add('warning');
        containerMessage.innerHTML = '<strong>Tentative de connexion</strong><br>message';
        console.log(containerMessage);
    } else {
        containerMessage.classList.add('success');
        containerMessage.innerHTML = '<strong>Connexion approuvée</strong><br>Vous allez être redirigé sous peu.';
        setTimeout(onDashboard, 1000, message);
    }
}

export { onLogin };