import { $ } from "./dollard";
import Backend from "./Backend";

// "Model" of response
interface ResponseApi {
  name: string;
}

// This is the bootstrapping function of the frontend
const bootFront = () => {
  const name = `Succulence IT!`;
  const button1 = $.id("superButton");

  // Here we add a callback when the button is clicked on
  button1.onclick = () => {
    $.id("superTest").textContent = "Hi there is a lot to code now !";
  };

  // Here we fetch with the classname, get via class can fetch multiple elements this is why we get the first one
  const superButton = $.class("superButton2")[0];

  // Same as before we attach an onclick callback on the element
  superButton.onclick = () => {
    // Here, when you click on the second buttton we want to add the name of the company
    $.id("superTest").append(
      // We use this function to transform our string into a "real" HTML Element
      $.elementFromString(`<div class="appended">${name}</div>`)
    );

    // Just for fun, we count how much things we spawned
    console.log(`There is : ${$.class("appended").length}`);
  };

  // This is an example of a button click that triggers an api call and then renders on screen
  $.id("callAPI").onclick = async () => {
    let response: ResponseApi = await fetch("/api/test").then(r => r.json());
    $.id("api-result").textContent = `${response.name}`;
  };

  Backend.instance.send("coucou");
};

// It is important to use all functions inside it to avoid possible missing an HTMLElement because the script was invoked before the page is done painting
document.addEventListener("DOMContentLoaded", () => {
  bootFront();
});
