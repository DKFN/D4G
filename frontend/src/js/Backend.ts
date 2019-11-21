import { onLogin } from "./login";

class Backend {
  socket: WebSocket = null;

  initialize() {
    this.socket = new WebSocket(`ws://${window.location.host}/socket`);

    this.socket.onclose = this.onclose;
    this.socket.onerror = this.onerror;
    this.socket.onmessage = this.onmessage;
    this.socket.onopen = this.onopen;
  }

  onopen(event: Event): void {
    console.info(event);
  }

  onclose(event: CloseEvent): void {
    console.info(event);
  }

  onerror(event: ErrorEvent): void {
    console.info(event);
  }

  onmessage(event: MessageEvent): void {
    console.info(event);

    const json = JSON.parse(event.data);
    switch (json.topic) {
      case "test":
        console.warn("test", json.data);
        break;

      case "ko-login":
        onLogin(json.data, false);
        break;

      case "ok-login":
        onLogin(json.data, true);
        break;
    }
  }

  login(login: string, password: string) {
    this.send(JSON.stringify({
      topic: "try-login",
      data: {
        login,
        password
      }
    }));
  }

  forgetPassword(login: string) {
    console.log('forget : ', login);
    this.send(JSON.stringify({
      topic: "forget-password",
      data: {
        login
      }
    }))
  }

  renewPassword(login: string, password: string, token: string) {
    console.log('renew : ', login);
    this.send(JSON.stringify({
      topic: "renew-password",
      data: {
        login,
        password,
        token
      }
    }))
  }

  register(login, password, logement) {
    this.send(
      JSON.stringify({
        topic: "register",
        data: {
          login,
          password,
          logement
        }
      })
    );
  }

  send(message: string): void {
    if (this.socket && this.socket.readyState) {
      this.socket.send(message);
    }
  }
};

const backend = new Backend();

export default backend;
