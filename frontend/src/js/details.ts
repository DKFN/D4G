const context = 'page-details';

function onDetails(context, foyer) {
    // Open modal
    const modal = context.getElementsByClassName('modal')[0];
    modal.classList.toggle('active');
    modal.getElementsByTagName('form')[0].action += '/' + foyer;
    modal.querySelector('[data-id="amount"]').value = foyer;
}

export { onDetails };
