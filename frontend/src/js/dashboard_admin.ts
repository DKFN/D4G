import {clean} from "./dom";
import {$} from "./dollard";


function onDashboardAdmin(data) {
    clean('page-dashboard-admin');
    localStorage.setItem('admin-data', data);
    onArrayAdmin('table-releve-admin', data)
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
        cellLocataire.innerHTML = item.locataire;
        cellProprietaire.innerHTML = item.proprietaire;
        cellButton.innerHTML = '<a href="#">Plus</a>';
    });
}

export { onDashboardAdmin }