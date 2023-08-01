use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
struct CustomEntry {
    key: Vec<u8>,
    value: Vec<u8>,
}

fn write_custom_format_to_file(filename: &str, data: &[CustomEntry]) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for entry in data {
        file.write_all(&(entry.key.len() as u8).to_ne_bytes())?;
        file.write_all(&(entry.value.len() as u8).to_ne_bytes())?;
        file.write_all(&entry.key)?;
        file.write_all(&entry.value)?;
    }

    Ok(())
}

fn read_custom_format_from_file(filename: &str) -> io::Result<Vec<CustomEntry>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut entries = Vec::new();
    let mut offset = 0;

    while offset < buffer.len() {
        let key_len = buffer[offset] as usize;
        let value_len = buffer[offset + 1] as usize;

        let key = buffer[offset + 2..offset + 2 + key_len].to_vec();
        let value = buffer[offset + 2 + key_len..offset + 2 + key_len + value_len].to_vec();

        entries.push(CustomEntry { key, value });

        offset += 2 + key_len + value_len;
    }

    Ok(entries)
}

fn main() {
    // Beispielverwendung des benutzerdefinierten Formats
    let mut data: Vec<CustomEntry> = Vec::new();

    let mut entry1 = HashMap::new();
    entry1.insert("Alter", "30");
    entry1.insert("Stadt", "Musterstadt");
    data.push(CustomEntry {
        key: "Max Mustermann".to_string().into_bytes(),
        value: serialize_map(entry1),
    });

    let mut entry2 = HashMap::new();
    entry2.insert("Alter", "25");
    entry2.insert("Stadt", "Beispielstadt");
    data.push(CustomEntry {
        key: "John Doe".to_string().into_bytes(),
        value: serialize_map(entry2),
    });

    // Schreibe die Daten in eine binäre Datei
    if let Err(err) = write_custom_format_to_file("data.bin", &data) {
        eprintln!("Fehler beim Schreiben der Datei: {}", err);
        return;
    }

    // Lese die Daten aus der binären Datei
    if let Ok(entries) = read_custom_format_from_file("data.bin") {
        println!("Alle Einträge:");
        for entry in &entries {
            println!("{:?}", entry);
        }

        // Suche nach dem Alter von Max Mustermann
        let search_name = "Max Mustermann".to_string().into_bytes();
        if let Some(entry) = entries.iter().find(|entry| entry.key == search_name) {
            let value_map = deserialize_map(&entry.value);
            if let Some(age) = value_map.get("Alter") {
                println!("Das Alter von Max Mustermann ist {}", age);
            } else {
                println!("Das Alter von Max Mustermann wurde nicht gefunden.");
            }
        } else {
            println!("Max Mustermann wurde nicht gefunden.");
        }

        // Suche nach dem Alter von John Doe
        let search_name = "John Doe".to_string().into_bytes();
        if let Some(entry) = entries.iter().find(|entry| entry.key == search_name) {
            let value_map = deserialize_map(&entry.value);
            if let Some(age) = value_map.get("Alter") {
                println!("Das Alter von John Doe ist {}", age);
            } else {
                println!("Das Alter von John Doe wurde nicht gefunden.");
            }
        } else {
            println!("John Doe wurde nicht gefunden.");
        }
    } else {
        eprintln!("Fehler beim Lesen der Datei.");
    }
}

fn serialize_map(map: HashMap<&str, &str>) -> Vec<u8> {
    let mut result = Vec::new();
    for (key, value) in map {
        result.push(key.len() as u8);
        result.push(value.len() as u8);
        result.extend_from_slice(key.as_bytes());
        result.extend_from_slice(value.as_bytes());
    }
    result
}

fn deserialize_map(data: &[u8]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut offset = 0;

    while offset < data.len() {
        let key_len = data[offset] as usize;
        let value_len = data[offset + 1] as usize;

        let key = String::from_utf8_lossy(&data[offset + 2..offset + 2 + key_len]).to_string();
        let value =
            String::from_utf8_lossy(&data[offset + 2 + key_len..offset + 2 + key_len + value_len])
                .to_string();

        map.insert(key, value);

        offset += 2 + key_len + value_len;
    }

    map
}
