import { clean, bind } from "./dom";
import { onDetails } from "./details";
import { $ } from "./dollard";
import Polling from "./Polling";

function onDashboard(data) {
    clean('page-dashboard-user');
    const dashboard = $.id('page-dashboard-user');

    bind(dashboard, {
        'proprietaire': data.proprietaire.societe || data.proprietaire.prenom + ' ' + data.proprietaire.nom,
        'locataire': data.locataire.prenom + ' ' + data.locataire.nom,
        'pieces': data.nb_pieces + ' pièce' + (data.nb_pieces != '1' ? 's' : '') || 'Non renseignée',
        'ville': data.ville || 'Non renseignée'
    });

    dashboard.querySelector('[action="open-modal"]').onclick = () => {
        onDetails(dashboard, data.foyer)
    };

    onArrayUser('table-releve-user', data);

    Polling.instance.send();
}

function onArrayUser(tableId, data) {
    const table: HTMLTableElement = <HTMLTableElement> $.id(tableId);

    const reversed = data.releves.reverse();
    reversed.forEach(function(item, index) {
        let row: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
        let cellDate: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellValeur: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellProgression: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        cellDate.innerHTML = item.date;
        cellValeur.innerHTML = item.valeur;
        cellProgression.innerHTML = (index < reversed.length - 1 ? item.valeur - reversed[index + 1].valeur : "").toString();
    });
}

export { onDashboard };
