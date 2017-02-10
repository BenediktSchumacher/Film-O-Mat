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
}
