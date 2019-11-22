import { clean, bind } from "./dom";
import { onDetails } from "./details";
import { $ } from "./dollard";
import Polling from "./Polling";
import File from "./File";
import {closeModal, closingModal} from "./modal";

function onDashboard(data) {
    clean('page-dashboard-user');
    const dashboard = $.id('page-dashboard-user');

    bind(dashboard, {
        'proprietaire': data.proprietaire.societe || data.proprietaire.prenom + ' ' + data.proprietaire.nom,
        'locataire': data.locataire.prenom + ' ' + data.locataire.nom,
        'pieces': data.nb_pieces + ' pièce' + (data.nb_pieces != '1' ? 's' : '') || 'Non renseignée',
        'ville': data.ville || 'Non renseignée'
    });

    // Open modal for add detail (releve) to a foyer
    dashboard.querySelector('[action="open-modal"]').onclick = () => {
        onDetails(dashboard, data.foyer)
    };

    // Open modal for upload file
    dashboard.querySelector('[action="open-modal-file"]').onclick = () => {
        uploadFileToFoyer($.id('user-dashboard-main'), data.foyer)
    };

    onArrayUser('table-releve-user', data);
    onArrayFiles('access-files-dashboard', data.fichiers);

    Polling.send(data.foyer);
}

function uploadFileToFoyer(context, foyer) {
    // Open modal
    const modal = context.getElementsByClassName('modal')[0];
    modal.classList.toggle('active');
    closingModal(modal);
    const form = modal.getElementsByTagName('form')[0];
    File.setInput(form.getElementsByTagName('input')[0]);
    File.setFoyer(foyer);
    form.getElementsByTagName('button')[0].onclick = () => {
        File.upload();
        closeModal(modal);
        return false;
    };
}

function onArrayFiles(id, files) {
    const contextTable = $.id(id);
    contextTable.getElementsByTagName('tbody')[0].innerHTML = '';
    files.forEach(el => {
       contextTable.innerHTML += `<tr><td>${el.replace('/files/','')}</td></tr>`;
        contextTable.onclick = () => {
            window.open(window.location.href.replace(/\/$/, '') + el, '_blank');
        }
    });
}

function onArrayUser(tableId, data) {
    const table: HTMLTableElement = <HTMLTableElement> $.id(tableId);

    while (table.hasChildNodes()) {
        table.removeChild(table.firstChild);
    }

    let rowHead: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
    const head = [ "Date", "Relevé", "Evolution" ];
    head.forEach(function (item) {
       let cell: HTMLTableCellElement = <HTMLTableCellElement> document.createElement("th");
       cell.innerHTML = item;
       rowHead.appendChild(cell);
    });

    const reversed = data.releves.reverse();
    reversed.forEach(function(item, index) {
        let row: HTMLTableRowElement = <HTMLTableRowElement> table.insertRow();
        let cellDate: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellValeur: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        let cellProgression: HTMLTableCellElement = <HTMLTableCellElement> row.insertCell();
        cellDate.innerHTML = item.date;
        cellValeur.innerHTML = item.valeur;
        const difference = index < reversed.length - 1 ? item.valeur - reversed[index + 1].valeur : 0;
        cellProgression.innerHTML = (difference > 0 ? "&#8598; " : "&#8600; ") + difference.toString();
        if (difference > 0) {
            cellProgression.className = "warning";
        } else {
            cellProgression.className = "success";
        }
    });
}

export { onDashboard };
