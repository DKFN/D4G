import {clean} from "./dom";


function onDashboardAdmin(data) {
    clean('page-dashboard-admin');
    onArrayAdmin('table-releve-admin', data)
}