import Backend from './Backend';

interface Releve {
  date: string;
  value: string;
}

export default class Polling {
  static SEND_TIMEOUT: number = 5000;

  static _instance: Polling = null;

  static get instance(): Polling {
    return Polling._instance || new Polling();
  }

  timeoutId: NodeJS.Timeout;

  Polling() {
    Polling._instance = this;
  }

  send() {
    if (!this.timeoutId) {
      this.timeoutId = setTimeout(() => {
        Backend.pollData();
      }, Polling.SEND_TIMEOUT);
    }
  }

  receive(releves: Array<Releve>) {
    clearTimeout(this.timeoutId);
    this.timeoutId = null;
    this.send();
  }
}
