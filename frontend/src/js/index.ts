import {$} from "./dollard";

// This is the bootstrapping function of the frontend
const bootFront =  () => {
    const name = `Succulence IT!`;
    const button1 = $.id("superButton");

    button1.onclick = () => {
        $.id("superTest").textContent = "Hi there is a lot to code now !";
    };

    const superButton = $.class("superButton2")[0];

    superButton.onclick = () => {
        $.id("superTest").append(
            $.elementFromString(
                `<div class="appended">${name}</div>`
            )
        );
    };
};

document.addEventListener("DOMContentLoaded", () => {
    bootFront();
});
