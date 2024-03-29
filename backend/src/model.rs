
#[derive(Deserialize, Serialize)]
pub struct Logement {
    pub foyer: String,
    pub l_type: i32,
    pub surface: f32,
    pub nb_pieces : i32,
    pub chauffage : String,
    pub date_construction: i32,
    pub n_voie: String,
    pub voie1: String,
    pub code_postal: String,
    pub ville: String,
    pub proprietaire: Proprietaire,
    pub locataire: Locataire,
    pub releves: Vec<Releve>,
    pub fichiers: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct Resume {
    pub foyer: String,
    pub l_type: i32,
    pub ville: String,
    pub proprietaire: String,
    pub locataire: String,
}

#[derive(Deserialize, Serialize)]
pub struct Proprietaire {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub societe: Option<String>,
    pub adresse: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Locataire {
    pub nom: String,
    pub prenom: String,
}

#[derive(Deserialize, Serialize)]
pub struct Releve {
    pub date: String,
    pub valeur: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Utilisateur {
    pub login: String,
    pub password: String,
    pub active: bool,
    pub foyer: Logement,
}

#[derive(Deserialize, Serialize)]
pub struct AddReleve {
    pub foyer: String,
    pub date: String,
    pub valeur: i32,
}
