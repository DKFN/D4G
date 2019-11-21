import Backend from './Backend';
import { onDashboard } from './dashboard';

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

  timeoutId: NodeJS.Timeout = null;

  Polling() {
    Polling._instance = this;
  }

  send() {
    if (this.timeoutId === null) {
      this.timeoutId = setTimeout(() => {
        Backend.pollData();
      }, Polling.SEND_TIMEOUT);
    }
  }

  receive(data) {
    clearTimeout(this.timeoutId);
    this.timeoutId = null;

    onDashboard(data);
  }
}
