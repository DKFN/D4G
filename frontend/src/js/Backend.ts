

export default class Backend {
  static _instance: Backend = null;

  static get instance() {
    return Backend._instance || new Backend();
  }

  socket: WebSocket = null;

  Socket() {
    Backend._instance = this;

    this.socket = new WebSocket(`ws://${window.location.hostname}/socket`);

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
  }

  send(message: string): void {
    if (this.socket && this.socket.readyState) {
      this.socket.send(message);
    }
  }
};
