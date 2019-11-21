import { $ } from "./dollard";
import Backend from "./Backend";
import {clean} from "./dom";
import { onForget } from "./forget";

// "Model" of response
interface ResponseApi {
  name: string;
}

const bootFront = () => {
// This is the bootstrapping function of the frontend
  const url = new URL(window.location.href);
  if (url.searchParams.has('token')) {
    onForget();
    return;
  }
  clean('page-login');
  const login = $.id("btn-login");
  login.onclick = () => {
    Backend.login(
        $.id('login').value,
        $.id('password').value
    );
    return false;
  };
  const routeForget = $.id('page-login').querySelector('[to="page-forget"]');
  routeForget.onclick = () => {
    onForget();
  }
};

// It is important to use all functions inside it to avoid possible missing an HTMLElement because the script was invoked before the page is done painting
document.addEventListener("DOMContentLoaded", () => {
  Backend.initialize();

  bootFront();
  let pid = setInterval(() => {
    if (Backend.socket.readyState === WebSocket.OPEN) {
        console.log("OPENED CONNECTION OK");
        clearInterval(pid);
      } else {
        console.warn("Waiting for connection");
      }
  }, 50);
});
