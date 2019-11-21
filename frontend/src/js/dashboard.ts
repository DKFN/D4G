import { clean } from "./dom";
import { $ } from "./dollard";

function onDashboard(data) {
    clean('page-dashboard-user');
    const dashboard = $.id('page-dashboard-user');
    dashboard.querySelector(['[bind="proprietaire"]']).innerHTML = data.proprietaire.societe || data.proprietaire.prenom + ' ' + data.proprietaire.nom;
    dashboard.querySelector(['[bind="locataire"]']).innerHTML = data.locataire.prenom + ' ' + data.locataire.nom;
    dashboard.querySelector(['[bind="pieces"]']).innerHTML = data.nb_pieces + ' pièce' + (data.nb_pieces != '1' ? 's' : '') || 'Non renseignée';
    dashboard.querySelector(['[bind="ville"]']).innerHTML = data.ville || 'Non renseignée';

    console.log('ON DASHBOARD : ', data);

}

export { onDashboard };