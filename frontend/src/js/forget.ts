import Backend from "./Backend";
import { clean } from "./dom";
import { $ } from "./dollard";

const context = 'page-forget';

function receiveForgetPassword(data) {
    const page = $.id(context);
    if (page) {
        const containerMessage = page.getElementsByClassName('message')[0];
        containerMessage.classList.add(data.error ? 'warning' : 'success');
        containerMessage.innerHTML = `<strong>${data.error ? 'Erreur' : 'Félicitation'}</strong><br>${data.error || data.message}`;
    }
}

function onForget() {
    clean(context);
    const page = $.id(context);
    const submissionButton = page.getElementsByTagName('button')[0];
    const url = new URL(window.location.href);
    const containerMessage = page.getElementsByClassName('message')[0];
    const followingForm = page.querySelector('[if="nextForm"]');
    if (url.searchParams.has('token')) {
        followingForm.classList.remove('if-none');
        submissionButton.onclick = () => {
            const validate = checkForm(page, true);
            console.log(validate);
            if (validate.status) {
                containerMessage.classList.add('success');
                containerMessage.innerHTML = '<strong>Félicitation</strong><br>Vous allez être rediriger vers la page de connexion';
                // TODO : call backend by socket
                /*Backend.renewPassword(
                    login,
                    password,
                    url.searchParams.get('token')
                );*/
            } else {
                containerMessage.classList.add('warning');
                containerMessage.innerHTML = '<strong>Erreur</strong><br>' + validate.message;
            }
            return false;
        };
    } else {
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
    if (data.login.length < 1) {
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

export { onForget, receiveForgetPassword };
