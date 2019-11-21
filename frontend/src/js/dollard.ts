
export const $ = {
    id:(id: string): HTMLElement | undefined => document.getElementById(id),
    class: (className: string): HTMLElement[] => [].slice.call(document.getElementsByClassName(className)),
    elementFromString: (elString: string) => {
        var div = document.createElement('div');
        div.innerHTML = elString.trim();
        return div.firstChild;
    },
    currentPage: (): string => {
        const visibleMain = [].slice
            .call(document.getElementsByTagName('main'))
            .filter(element => element.classList.contains('visible'))[0];

        return visibleMain ? visibleMain.id : null;
    }
};
