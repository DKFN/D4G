function closingModal(modal) {
    modal.onclick = e => {
        if (e.target.className !== 'modal active') {
            return;
        }
        closeModal(modal);
    };
    modal.querySelector('[action="cancel-modal"]').onclick = () => {
        closeModal(modal);
    };
}

function closeModal(modal) {
    modal.classList.remove('active');
}

export { closingModal, closeModal }