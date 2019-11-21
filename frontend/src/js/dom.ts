function clean(except) {
    [...document.getElementsByTagName('main')].forEach(el => {
        if (el.id === except) {
            el.classList.add('visible');
            return;
        }
        el.classList.remove('visible');
    });
}

export { clean };