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


function onArrayAdmin(tableId, data) {
    const table: HTMLTableElement = <HTMLTableElement> $.id(tableId);

    data.forEach(function (item) {
       let row: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
       let cellFoyer: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
       let cellType: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
       let cellVille: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
       let cellLocataire: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
       let cellProprietaire: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
       let cellButton: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
       cellFoyer.innerHTML = item.foyer;
       cellType.innerHTML = item.l_type;
       cellVille.innerHTML = item.ville;
       cellLocataire.innerHTML = item.locataire_prenom + " " + item.locataire_nom;
       cellProprietaire.innerHTML = item.proprietaire_societe ? item.proprietaire_societe : item.proprietaire_prenom + " " + item.proprietaire_nom;
       cellButton.innerHTML = '<a href="#">Plus</a>';
    });
}

export { onDashboard, onDashboardAdmin };
