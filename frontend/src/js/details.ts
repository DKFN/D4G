import {closeModal, closingModal} from "./modal";
import Backend from "./Backend";
import {$} from "./dollard";

const context = 'page-details';

function onDetails(context, foyer) {
    // Open modal
    const modal = context.getElementsByClassName('modal')[2];
    modal.classList.toggle('active');
    modal.getElementsByTagName('button')[0].onclick = () => {
        const validated = checkForm(context);

        if (!validated.status) {
            return;
        }

        Backend.addDetail(foyer, validated.date, validated.amount);

        return false;
    };
    closingModal(modal);
}

function onResponseDetail(data) {
    const modal = $.id('page-dashboard-user').getElementsByClassName('modal')[2];
    const message = modal.querySelector('.message');

    message.classList.add(data.error ? 'warning' : 'success');
    message.innerHTML = `<strong>${data.error ? 'Erreur' : 'Succès'}</strong><br>${data.error || data.message}`;

    if (!data.error) {
        setTimeout(closeDetailModal, 1000, modal, message);
    }
}

function closeDetailModal(modal, message) {
    modal.querySelector('[data-id="date"]').value = "";
    modal.querySelector('[data-id="amount"]').value = "";
    message.classList.remove('success');
    message.innerHTML = '';
    closeModal(modal);
}

function getFormData(context) {
    return {
        date: context.querySelector('[data-id="date"]').value,
        amount: parseInt(context.querySelector('[data-id="amount"]').value),
    }
}

function checkForm(context) {
    const formData = getFormData(context);
    return {
        status: true,
        ...formData
    }
}

export { onDetails, onResponseDetail };
