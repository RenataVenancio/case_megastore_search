use crate::models::Product;
use std::fs::File;
use std::io::Write;
use serde_json;

pub fn export_to_json(produtos: &[Product], caminho: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(produtos)?;
    let mut file = File::create(caminho)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn export_to_csv(produtos: &[Product], caminho: &str) -> std::io::Result<()> {
    let mut wtr = csv::Writer::from_path(caminho)?;

    // Cabeçalhos
    wtr.write_record(&["id", "name", "description", "category", "store", "price", "link", "quantity"])?;

    for p in produtos {
        wtr.write_record(&[
            &p.id.to_string(),
            p.name.as_str(),
            p.description.as_str(),
            p.category.as_str(),
            p.store.as_str(),
            &format!("{:.2}", p.price),
            p.link.as_str(),
            &p.quantity.to_string()
        ])?;
        
    }

    wtr.flush()?;
    Ok(())
}
