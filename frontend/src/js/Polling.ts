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

  intervalId: NodeJS.Timeout = null;

  Polling() {
    Polling._instance = this;
  }

  send() {
    if (!this.intervalId) {
      this.intervalId = setInterval(() => {
        Backend.pollData();
      }, Polling.SEND_TIMEOUT);
    }
  }

  receive(data) {
    onDashboard(data);
  }
}
