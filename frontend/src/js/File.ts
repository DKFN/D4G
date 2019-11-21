class File {
    private path: string = '/file/';
    private foyer: string = '';
    private input: HTMLInputElement;
    private file;

    constructor() {

    }

    upload() {
        let formData = new FormData();
        console.log(this.file);
        formData.set('file', this.file);
        console.log('formdata: ', formData);
        const request = new XMLHttpRequest();
        request.open("POST", this.route());
        // request.setRequestHeader("Content-Type", "multipart/form-data");
        request.send(formData);
    }

    route () {
        //return 'http://localhost' + this.path + this.foyer;
        return 'file';
    }

    setFoyer(foyer: string) {
        this.foyer = foyer;
    }

    setInput(input: HTMLInputElement) {
        this.input = input;
        this.input.onchange = (e) => {
            this.file = e.target.files[0] || e.dataTransfer.files[0];
            console.log(this.file);
        }
    }
}

const file = new File();

export default file;
