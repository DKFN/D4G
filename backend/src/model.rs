pub struct Logement {
    foyer: String,
    l_type: i32,
    surface: String,
    chauffage : String,
    date_construction: i32,
    n_voie: String,
    voie1: String,
    code_postal: String,
    ville: String,
    proprietaire: Proprietaire,
    locataire: Locataire,
    releves: Vec<Releve>,
}

pub struct Proprietaire {
    nom: String,
    prenom: String,
    societe: String,
    adresse: String
}

pub struct Locataire {
    nom: String,
    prenom: String,
}

pub struct Releve {
    date: String,
    valeur: String,
}

pub struct Utilisateur {
    login: String,
    password: String,
    active: bool,
    foyer: Logement,
}
