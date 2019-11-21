function clean(except) {
    [...document.getElementsByTagName('main')].forEach(el => {
        if (el.id === except) {
            el.classList.add('visible');
            return;
        }
        el.classList.remove('visible');
    });
}

function bind(context: HTMLElement, metaData: Object) {
    Object.keys(metaData).forEach(key => {
        context.querySelector([`[bind="${key}"]`]).innerHTML = metaData[key]
    });
}

export { clean, bind };