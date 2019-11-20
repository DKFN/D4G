

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
    console.log(event);
  }

  onclose(event: CloseEvent): void {
    console.log(event);
  }

  onerror(event: ErrorEvent): void {
    console.log(event);
  }

  onmessage(event: MessageEvent): void {
    console.log(event);

    const json = JSON.parse(event.data);
    switch (json.topic) {
      case "test":
        console.warn("test", json.data);
        break;

      default:
        console.warn("default", json);
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

  register(locataire, proprietaire, user) {
    this.send(
      JSON.stringify({
        topic: "register",
        data: {
          locataire,
          proprietaire,
          user
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
