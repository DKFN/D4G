import { clean } from "./dom";
import { $ } from "./dollard";

function onDashboard(data) {
    clean('page-dashboard-user');
    const dashboard = $.id('page-dashboard-user');
    dashboard.querySelector(['[bind="proprietaire"]']).innerHTML = data.proprietaire.societe || data.proprietaire.prenom + ' ' + data.proprietaire.nom;
    dashboard.querySelector(['[bind="locataire"]']).innerHTML = data.locataire.prenom + ' ' + data.locataire.nom;
    dashboard.querySelector(['[bind="pieces"]']).innerHTML = data.nb_pieces + ' pièce' + (data.nb_pieces != '1' ? 's' : '') || 'Non renseignée';
    dashboard.querySelector(['[bind="ville"]']).innerHTML = data.ville || 'Non renseignée';
    onArray(data);

    console.log('ON DASHBOARD : ', data);

}

function onArray(data) {
    const table: HTMLTableElement = <HTMLTableElement> $.id('table-releve-user');
    const reversed = data.releves.reverse();
    reversed.forEach(element => {
        let row: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
        let cellDate: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellValeur: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        cellDate.innerHTML = element.date;
        cellValeur.innerHTML = element.valeur;
    });
}

export { onDashboard };