import { $ } from "./dollard";
import { onDashboard } from "./dashboard";
import { onDashboardAdmin } from "./dashboard_admin"
import {clean} from "./dom";
import Backend from "./Backend";
import {onForget} from "./forget";

function initLogin() {
    clean('page-login');
    const login = $.id("btn-login");
    login.onclick = () => {
        Backend.login(
            $.id('login').value,
            $.id('password').value
        );
        return false;
    };
    const routeForget = $.id('page-login').querySelector('[to="page-forget"]');
    routeForget.onclick = () => {
        onForget();
    }
}

function onLogin(data, status, admin=false) {
    const containerMessage = $.id('page-login').getElementsByClassName('message')[0];

    if (!status) { // Connexion failed
        containerMessage.classList.add('warning');
        containerMessage.innerHTML = `<strong>Tentative de connexion</strong><br>${data.message}`;
    } else {
        containerMessage.classList.add('success');
        containerMessage.innerHTML = '<strong>Connexion approuvée</strong><br>Vous allez être redirigé sous peu.';
        if (admin) {
            setTimeout(onDashboardAdmin, 1000, data)
        } else {
            setTimeout(onDashboard, 1000, data);
        }
    }
}

export { onLogin, initLogin };
