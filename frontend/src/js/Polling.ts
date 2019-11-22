import Backend from './Backend';
import { onDashboard } from './dashboard';
import { onDashboardAdmin } from './dashboard_admin';
import { $ } from './dollard';

class Polling {
  static SEND_TIMEOUT: number = 3000;

  intervalId: NodeJS.Timeout = null;

  send(data = null) {
    clearInterval(this.intervalId);

    this.intervalId = setInterval(() => {
      Backend.pollData(data);
    }, Polling.SEND_TIMEOUT);
  }

  receive(data) {
    if ($.currentPage() === 'page-dashboard-user') {
      onDashboard(data);
    } else {
      onDashboardAdmin(data);
    }
  }
}

const polling = new Polling();

export default polling;
