import {closeModal, closingModal} from "./modal";
import Backend from "./Backend";
import {$} from "./dollard";

const context = 'page-details';

function onDetails(context, foyer) {
    // Open modal
    const modal = context.getElementsByClassName('modal')[1];
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
    const modal = $.id('page-dashboard-user').getElementsByClassName('modal')[1];
    const message = modal.querySelector('.message');
    message.classList.add('success');
    message.innerHTML = '<strong>Félicitation</strong><br>Votre relevé a bien été rajouté !';
    setTimeout(closeDetailModal, 1000, modal, message);
}

function closeDetailModal(modal, message) {
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
