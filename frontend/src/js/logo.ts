function loadLogo(context) {
    const logo = context.querySelector('aside img');
    if (logo) {
        logo.setAttribute('src', logo.getAttribute('data-src'));
    }
}

export { loadLogo }