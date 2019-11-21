import { $ } from "./dollard";
import backend from "./Backend";

function onRegister(context) {
    // Open modal
    const modal = context.getElementsByClassName('modal')[0];
    modal.classList.toggle('active');
}

function displayProprietaire(response) {
    const form = $.id("proprietaire_form");
    let labels = form.getElementsByTagName("label");
    for (let i = 0; i < labels.length; i++) {
        if (labels[i].htmlFor != '') {
            let elem = $.id(labels[i].htmlFor);
            if (elem)
                elem.label = labels[i];
        }
    }
    if (response === "personne") {
        $.id("proprietaire_nom").className = "";
        $.id("proprietaire_nom").label.className = "";
        $.id("proprietaire_prenom").className = "";
        $.id("proprietaire_prenom").label.className = "";
        $.id("nom_entreprise").className = "if-none";
        $.id("nom_entreprise").label.className = "if-none";
        $.id("proprietaire_adresse").className = "";
        $.id("proprietaire_adresse").label.className = "";
    } else {
        $.id("proprietaire_nom").className = "if-none";
        $.id("proprietaire_nom").label.className = "if-none";
        $.id("proprietaire_prenom").className = "if-none";
        $.id("proprietaire_prenom").label.className = "if-none";
        $.id("nom_entreprise").className = "";
        $.id("nom_entreprise").label.className = "";
        $.id("proprietaire_adresse").className = "";
        $.id("proprietaire_adresse").label.className = "";
    }
}

function get_data() {
    const login = $.id("identifiant").value;
    const password = $.id("mdp").value;
    const chauffage_select = $.id("chauffage");
    const chauffage = chauffage_select.options[chauffage_select.selectedIndex].value;
    const code_postal = $.id("cp").value;
    const date_construction = $.id("date_construction").value;
    const fichiers = [];
    const foyer = "";
    const l_type_select = $.id("type_logement");
    const l_type = l_type_select.options[l_type_select.selectedIndex].value;
    const locataire_nom = $.id("locataire_nom");
    const locataire_prenom = $.id("locataire_prenom");
    const n_voie = $.id("n_voie").value;
    const nb_pieces = $.id("nb_pieces").value;
    const proprietaire_adresse = $.id("proprietaire_adresse").value;
    const proprietaire_nom = $.id("proprietaire_nom").value;
    const proprietaire_prenom = $.id("proprietaire_prenom").value;
    const proprietaire_societe = $.id("proprietaire_entreprise").value;
    const releves = [];
    const surface = $.id("surface").value;
    const ville = $.id("ville").value;
    const voie1 = $.id("voie").value;

    const logement = {
        chauffage,
        code_postal,
        date_construction,
        fichiers,
        foyer,
        l_type,
        locataire: {
            nom: locataire_nom,
            prenom: locataire_prenom
        },
        n_voie,
        nb_pieces,
        proprietaire: {
            adresse: proprietaire_adresse,
            nom: proprietaire_nom,
            prenom: proprietaire_prenom,
            societe: proprietaire_societe
        },
        releves,
        surface,
        ville,
        voie1
    };

    backend.register(login, password, logement);
}

export { onRegister, displayProprietaire, get_data };