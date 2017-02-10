mod database;

extern crate rusqlite;

use database::*;

fn main() {

    // TODO: Pruefem, ob Datenbank vorhanden ist
    // Falls nein => Dateipfad erstellen, Datenbank anlegen und anschliessend fuellen
    // Falls ja => Datenbankverbindung herstellen, Anfrage bearbeiten
    if !db_exists() {
        create_database();
    }

    import_movie("Batman", "2005");
    add_genres("Batman", "2005", "Thriller");
    add_rating("Batman", "2005", "8.324", 500);

    let movie_1 = Movie {
        title: "Batman".to_string(),
        year: "2005".into(),
    };

    let name = "Heath Ledgers";

    add_actor(name, vec![movie_1]);
}
