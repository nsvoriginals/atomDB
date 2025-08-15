use crate::database::{StorageEngine, Row};

pub fn setup_demo_data(storage: &mut StorageEngine) -> Result<(), String> {
    storage.create_table("users", vec![
        "id".to_string(),
        "name".to_string(),
        "email".to_string(),
        "age".to_string(),
    ])?;

    let users = vec![
        ("1", "Alice", "alice@example.com", "25"),
        ("2", "Bob", "bob@example.com", "30"),
        ("3", "Charlie", "charlie@example.com", "28"),
    ];

    for (id, name, email, age) in users {
        let mut row = Row::new();
        row.insert("id".to_string(), id.to_string());
        row.insert("name".to_string(), name.to_string());
        row.insert("email".to_string(), email.to_string());
        row.insert("age".to_string(), age.to_string());
        storage.insert_row("users", row)?;
    }

    Ok(())
}

pub fn autosave_database(storage: &StorageEngine) {
    if let Err(e) = storage.save_to_binary_file("database.bin") {
        eprintln!("Autosave failed: {}", e);
    }
}
