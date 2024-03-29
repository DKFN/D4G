import {clean} from "./dom";
import {$} from "./dollard";
import "./Backend";
import backend from "./Backend";
import Polling from "./Polling";
import { onRegister, displayProprietaire, get_data } from "./register";
import {loadLogo} from "./logo";

function onDashboardAdmin(data) {
    clean('page-dashboard-admin');
    const dashboard = $.id('page-dashboard-admin');
    loadLogo(dashboard);
    localStorage.setItem('admin-data', JSON.stringify(data));

    dashboard.querySelector('[action="open-modal"]').onclick = () => {
        onRegister(dashboard);
    };

    handle_register_onclick();

    onArrayAdmin('table-releve-admin', data)

    Polling.send();
}

function onArrayAdmin(tableId, data) {
    const table: HTMLTableElement = <HTMLTableElement> $.id(tableId);

    while (table.hasChildNodes()) {
        table.removeChild(table.firstChild);
    }

    let rowHead: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
    const head = [ "Type", "Ville", "Locataire", "Propriétaire" ];
    head.forEach(function (item) {
        let cell: HTMLTableCellElement = <HTMLTableCellElement> document.createElement("th");
        cell.innerHTML = item;
        rowHead.appendChild(cell);
    });

    data.forEach(function (item) {
        let row: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
        let cellType: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellVille: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellLocataire: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellProprietaire: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();

        cellType.innerHTML = item.l_type === 1 ? "Appartement" : "Maison";
        cellVille.innerHTML = item.ville;
        cellLocataire.innerHTML = item.locataire;
        cellProprietaire.innerHTML = item.proprietaire;

        row.addEventListener("click", (() => {
            const target: HTMLDivElement = <HTMLDivElement> $.id("user-dashboard-main");
            let back_button: HTMLButtonElement = <HTMLButtonElement> document.createElement("button");
            back_button.innerHTML = "&#8676 Retour sur l'interface admin";
            back_button.addEventListener("click", (() => {
                const target: HTMLDivElement = <HTMLDivElement> $.id("user-dashboard-main");
                target.removeChild(target.firstChild);
                onDashboardAdmin(JSON.parse(localStorage.getItem("admin-data")));
            }));

            target.insertBefore(back_button, target.firstChild);
            backend.infoLogement(item.foyer);
        }))
    });
}

function handle_register_onclick() {
    let radio_personne: HTMLInputElement = <HTMLInputElement> $.id("personne");
    let radio_entreprise: HTMLInputElement = <HTMLInputElement> $.id("entreprise");
    let send_register: HTMLButtonElement = <HTMLButtonElement> $.id("send_register");

    radio_personne.addEventListener("click", (() => {
        displayProprietaire("personne");
    }));
    radio_entreprise.addEventListener("click", (() => {
        displayProprietaire("entreprise");
    }));
    send_register.addEventListener("click", function(e) {
        e.preventDefault();
        get_data();
    });
}

export { onDashboardAdmin }
