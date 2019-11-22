import Backend from "./Backend";
import { clean } from "./dom";
import { $ } from "./dollard";
import {initLogin} from "./login";

const context = 'page-forget';

function receiveForgetPassword(data) {
    const page = $.id(context);
    if (page) {
        const containerMessage = page.getElementsByClassName('message')[0];
        containerMessage.className = "message";
        containerMessage.classList.add(data.error ? 'warning' : 'success');
        containerMessage.innerHTML = `<strong>${data.error ? 'Erreur' : 'Félicitation'}</strong><br>${data.error || data.message}`;
    }
}

function receiveRenewPassword(data) {
    const page = $.id(context);
    if (page) {
        const containerMessage = page.getElementsByClassName('message')[0];
        containerMessage.className = "message";
        containerMessage.classList.add(data.error ? 'warning' : 'success');
        containerMessage.innerHTML = `<strong>${data.error ? 'Erreur' : 'Félicitation'}</strong><br>${data.error || data.message}`;
    }
}

function onForget() {
    clean(context);
    const page = $.id(context);
    page.querySelector('[to]').onclick = e => {
        initLogin();
    };
    const submissionButton = page.getElementsByTagName('button')[0];
    const url = new URL(window.location.href);
    const containerMessage = page.getElementsByClassName('message')[0];
    const followingForm = page.querySelector('[if="nextForm"]');
    const firstForm = page.querySelector('[if="firstForm"]');

    if (url.searchParams.has('token')) {
        firstForm.classList.add('if-none');
        followingForm.classList.remove('if-none');
        submissionButton.onclick = () => {
            const validate = checkForm(page, true);
            if (validate.status) {
                Backend.renewPassword(
                    url.searchParams.get('token'),
                    validate.data.password
                );
            } else {
                containerMessage.classList.add('warning');
                containerMessage.innerHTML = '<strong>Erreur</strong><br>' + validate.message;
            }
            return false;
        };
    } else {
        firstForm.classList.remove('if-none');
        followingForm.classList.add('if-none');
        submissionButton.onclick = () => {
            const validate = checkForm(page);
            // Success
            if (validate.status) {
                Backend.forgetPassword(validate.data);
            } else {
                containerMessage.classList.add('warning');
                containerMessage.innerHTML = '<strong>Erreur</strong><br>Une erreur est survenue !';
            }
            return false;
        };
    }
}

function getFormData(context) {
    return {
        login: context.querySelector('[data-id="login"]').value,
        password: context.querySelector('[data-id="password"]').value,
        confirm: context.querySelector('[data-id="confirm"]').value
    }
}

/**
 * Return validated data
 */
function checkForm(context, withToken = false) {
    const data = getFormData(context);
    if (!withToken && data.login.length < 1) {
        return {
            status: false,
            message: 'Veuillez remplir le champ login'
        };
    }

    if (withToken && (data.password.length < 1 || data.confirm.length < 1)) {
        return {
            status: false,
            message: 'Veuillez remplir les champs mot de passe'
        };
    }

    if (withToken && data.password !== data.confirm) {
        return {
            status: false,
            message: 'Les mots de passe ne correspondent pas'
        };
    }

    return {status: true, data: (!withToken ? data.login : data) };
}

export { onForget, receiveForgetPassword, receiveRenewPassword };
