use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;

use crate::models::{Order, Product};

/// Parse an XML order file into an Order struct.
///
/// Expected XML structure:
/// ```xml
/// <commande id="FA003195">
///   <client><![CDATA[...]]></client>
///   <adresse><![CDATA[...]]></adresse>
///   <date>...</date>
///   <produit>
///     <categorie><![CDATA[...]]></categorie>
///     <reference>...</reference>
///     <description><![CDATA[...]]></description>
///     <matiere><![CDATA[...]]></matiere>
///     <quantite>...</quantite>
///   </produit>
/// </commande>
/// ```
pub fn parse_order_xml(path: &str) -> Result<Order> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire le fichier : {}", path))?;

    let mut reader = Reader::from_str(&content);

    let mut order = Order {
        id: String::new(),
        client: String::new(),
        address: String::new(),
        date: String::new(),
        products: Vec::new(),
    };

    let mut current_tag = String::new();
    let mut in_produit = false;
    let mut current_product = new_product();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_tag = name.clone();

                match name.as_str() {
                    "commande" => {
                        // Extract the id attribute
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"id" {
                                order.id = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    "produit" => {
                        in_produit = true;
                        current_product = new_product();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "produit" {
                    in_produit = false;
                    order.products.push(current_product.clone());
                }
                current_tag.clear();
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().trim().to_string();
                if !text.is_empty() {
                    apply_text(&current_tag, &text, in_produit, &mut order, &mut current_product);
                }
            }
            Ok(Event::CData(ref e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).trim().to_string();
                if !text.is_empty() {
                    apply_text(&current_tag, &text, in_produit, &mut order, &mut current_product);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Erreur de parsing XML à la position {} : {:?}",
                    reader.error_position(),
                    e
                ));
            }
            _ => {}
        }
    }

    if order.id.is_empty() {
        return Err(anyhow::anyhow!(
            "ID de commande manquant dans l'attribut 'id' de <commande>"
        ));
    }

    Ok(order)
}

fn apply_text(
    tag: &str,
    text: &str,
    in_produit: bool,
    order: &mut Order,
    product: &mut Product,
) {
    if in_produit {
        match tag {
            "categorie" => product.category = text.to_string(),
            "reference" => product.reference = text.to_string(),
            "description" => product.description = text.to_string(),
            "matiere" => product.material = text.to_string(),
            "quantite" => product.quantity = text.parse().unwrap_or(1),
            _ => {}
        }
    } else {
        match tag {
            "client" => order.client = text.to_string(),
            "adresse" => order.address = text.to_string(),
            "date" => order.date = text.to_string(),
            _ => {}
        }
    }
}

fn new_product() -> Product {
    Product {
        category: String::new(),
        reference: String::new(),
        description: String::new(),
        material: String::new(),
        quantity: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_test_xml(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_parse_basic_order() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<commande id="FA003195">
  <client><![CDATA[PERIGUEUX]]></client>
  <adresse><![CDATA[Chemin du prêtre]]></adresse>
  <date>2020-01-15</date>
  <produit>
    <categorie><![CDATA[Electricité / SR04 / Motorisation]]></categorie>
    <reference>04-200</reference>
    <description><![CDATA[Kit motorisation]]></description>
    <matiere><![CDATA[PVC 3mm]]></matiere>
    <quantite>2</quantite>
  </produit>
</commande>"#;

        let file = write_test_xml(xml);
        let order = parse_order_xml(file.path().to_str().unwrap()).unwrap();

        assert_eq!(order.id, "FA003195");
        assert_eq!(order.client, "PERIGUEUX");
        assert_eq!(order.products.len(), 1);
        assert_eq!(order.products[0].reference, "04-200");
        assert_eq!(order.products[0].material, "PVC 3mm");
        assert_eq!(order.products[0].quantity, 2);
    }
}
