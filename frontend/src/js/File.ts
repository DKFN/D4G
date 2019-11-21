class File {
    private path: string = 'file/';
    private foyer: string = '';
    private input: HTMLInputElement;
    private file;

    constructor() {

    }

    upload() {
        let formData = new FormData();
        formData.set('file', this.file);
        const request = new XMLHttpRequest();
        request.open("POST", this.route());
        request.send(formData);
    }

    route () {
        return window.location.href + this.path + this.foyer;
    }

    setFoyer(foyer: string) {
        this.foyer = foyer;
    }

    setInput(input: HTMLInputElement) {
        this.input = input;
        this.input.onchange = (e) => {
            this.file = e.target.files[0] || e.dataTransfer.files[0];
        }
    }
}

const file = new File();

export default file;
