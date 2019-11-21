import Backend from "./Backend";
import { clean } from "./dom";
import { $ } from "./dollard";

const context = 'page-forget';

function onForget() {
    clean(context);
    const page = $.id(context);
    console.log(page);
}

export { onForget };