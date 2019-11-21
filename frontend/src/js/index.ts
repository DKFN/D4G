import { $ } from "./dollard";
import Backend from "./Backend";
import {clean} from "./dom";

// "Model" of response
interface ResponseApi {
  name: string;
}

// This is the bootstrapping function of the frontend
const bootFront = () => {
  clean('page-login');
  const login = $.id("btn-login");
  login.onclick = () => {
    Backend.login(
        $.id('login').value,
        $.id('password').value
    );
    return false;
  };
};

// It is important to use all functions inside it to avoid possible missing an HTMLElement because the script was invoked before the page is done painting
document.addEventListener("DOMContentLoaded", () => {
  Backend.initialize();

  let pid = setInterval(() => {
      if (Backend.socket.readyState === WebSocket.OPEN) {
        console.log("OPENED CONNECTION OK");
        clearInterval(pid);
        bootFront();
      } else {
        console.warn("Waiting for connection");
      }
  }, 50);
});
