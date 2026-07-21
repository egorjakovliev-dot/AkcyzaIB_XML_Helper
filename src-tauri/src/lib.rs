use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppDb(Mutex<Connection>);

const MIGRATION: &str = include_str!("../migrations/0001_initial.sql");

#[derive(Debug, Serialize)]
struct Product {
    id: i64,
    sku: String,
    name: String,
    allegro_offer_id: Option<String>,
    ean: Option<String>,
    category: String,
    brand: Option<String>,
    status: String,
    purchase_price: f64,
    selling_price: f64,
    currency: String,
    stock_quantity: i64,
    min_stock_quantity: i64,
    url: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct ProductInput {
    sku: String,
    name: String,
    allegro_offer_id: Option<String>,
    ean: Option<String>,
    category: String,
    brand: Option<String>,
    status: String,
    purchase_price: f64,
    selling_price: f64,
    currency: String,
    stock_quantity: i64,
    min_stock_quantity: i64,
    url: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
struct Competitor {
    id: i64,
    name: String,
    marketplace: String,
    url: String,
    rating: Option<f64>,
    review_count: Option<i64>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct CompetitorInput {
    name: String,
    marketplace: String,
    url: String,
    rating: Option<f64>,
    review_count: Option<i64>,
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SettingsPayload {
    store_name: String,
    currency: String,
    language: String,
    theme: String,
}

#[tauri::command]
fn list_products(db: State<AppDb>) -> Result<Vec<Product>, String> {
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    let mut stmt = conn.prepare("SELECT id, sku, name, allegro_offer_id, ean, category, brand, status, purchase_price, selling_price, currency, stock_quantity, min_stock_quantity, url, notes, created_at, updated_at FROM products ORDER BY updated_at DESC").map_err(|err| err.to_string())?;
    let rows = stmt.query_map([], product_from_row).map_err(|err| err.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|err| err.to_string())
}

#[tauri::command]
fn get_product(id: i64, db: State<AppDb>) -> Result<Option<Product>, String> {
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    conn.query_row("SELECT id, sku, name, allegro_offer_id, ean, category, brand, status, purchase_price, selling_price, currency, stock_quantity, min_stock_quantity, url, notes, created_at, updated_at FROM products WHERE id = ?1", [id], product_from_row)
        .optional()
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn create_product(input: ProductInput, db: State<AppDb>) -> Result<Product, String> {
    validate_required(&input.name, "Product name")?;
    validate_required(&input.sku, "SKU")?;
    let now = now();
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    conn.execute("INSERT INTO products (sku, name, allegro_offer_id, ean, category, brand, status, purchase_price, selling_price, currency, stock_quantity, min_stock_quantity, url, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?15)", params![input.sku, input.name, empty_to_none(input.allegro_offer_id), empty_to_none(input.ean), input.category, empty_to_none(input.brand), input.status, input.purchase_price, input.selling_price, input.currency, input.stock_quantity, input.min_stock_quantity, empty_to_none(input.url), empty_to_none(input.notes), now]).map_err(|err| err.to_string())?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_product(id, db)?.ok_or_else(|| "Created product was not found".to_string())
}

#[tauri::command]
fn update_product(id: i64, input: ProductInput, db: State<AppDb>) -> Result<Product, String> {
    validate_required(&input.name, "Product name")?;
    validate_required(&input.sku, "SKU")?;
    let now = now();
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    conn.execute("UPDATE products SET sku=?1, name=?2, allegro_offer_id=?3, ean=?4, category=?5, brand=?6, status=?7, purchase_price=?8, selling_price=?9, currency=?10, stock_quantity=?11, min_stock_quantity=?12, url=?13, notes=?14, updated_at=?15 WHERE id=?16", params![input.sku, input.name, empty_to_none(input.allegro_offer_id), empty_to_none(input.ean), input.category, empty_to_none(input.brand), input.status, input.purchase_price, input.selling_price, input.currency, input.stock_quantity, input.min_stock_quantity, empty_to_none(input.url), empty_to_none(input.notes), now, id]).map_err(|err| err.to_string())?;
    drop(conn);
    get_product(id, db)?.ok_or_else(|| "Product was not found".to_string())
}

#[tauri::command]
fn list_competitors(db: State<AppDb>) -> Result<Vec<Competitor>, String> {
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    let mut stmt = conn.prepare("SELECT id, name, marketplace, url, rating, review_count, notes, created_at, updated_at FROM competitors ORDER BY updated_at DESC").map_err(|err| err.to_string())?;
    let rows = stmt.query_map([], competitor_from_row).map_err(|err| err.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|err| err.to_string())
}

#[tauri::command]
fn create_competitor(input: CompetitorInput, db: State<AppDb>) -> Result<Competitor, String> {
    validate_required(&input.name, "Competitor name")?;
    let now = now();
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    conn.execute("INSERT INTO competitors (name, marketplace, url, rating, review_count, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)", params![input.name, input.marketplace, input.url, input.rating, input.review_count, empty_to_none(input.notes), now]).map_err(|err| err.to_string())?;
    let id = conn.last_insert_rowid();
    conn.query_row("SELECT id, name, marketplace, url, rating, review_count, notes, created_at, updated_at FROM competitors WHERE id = ?1", [id], competitor_from_row).map_err(|err| err.to_string())
}

#[tauri::command]
fn get_settings(db: State<AppDb>) -> Result<SettingsPayload, String> {
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    let value: Option<String> = conn.query_row("SELECT value FROM settings WHERE key='app'", [], |row| row.get(0)).optional().map_err(|err| err.to_string())?;
    Ok(value.and_then(|raw| serde_json::from_str(&raw).ok()).unwrap_or(SettingsPayload { store_name: "My Allegro Store".into(), currency: "PLN".into(), language: "ru".into(), theme: "dark".into() }))
}

#[tauri::command]
fn save_settings(input: SettingsPayload, db: State<AppDb>) -> Result<SettingsPayload, String> {
    let conn = db.0.lock().map_err(|err| err.to_string())?;
    let raw = serde_json::to_string(&input).map_err(|err| err.to_string())?;
    conn.execute("INSERT INTO settings (key, value, updated_at) VALUES ('app', ?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=excluded.updated_at", params![raw, now()]).map_err(|err| err.to_string())?;
    Ok(input)
}

fn product_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Product> {
    Ok(Product { id: row.get(0)?, sku: row.get(1)?, name: row.get(2)?, allegro_offer_id: row.get(3)?, ean: row.get(4)?, category: row.get(5)?, brand: row.get(6)?, status: row.get(7)?, purchase_price: row.get(8)?, selling_price: row.get(9)?, currency: row.get(10)?, stock_quantity: row.get(11)?, min_stock_quantity: row.get(12)?, url: row.get(13)?, notes: row.get(14)?, created_at: row.get(15)?, updated_at: row.get(16)? })
}

fn competitor_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Competitor> {
    Ok(Competitor { id: row.get(0)?, name: row.get(1)?, marketplace: row.get(2)?, url: row.get(3)?, rating: row.get(4)?, review_count: row.get(5)?, notes: row.get(6)?, created_at: row.get(7)?, updated_at: row.get(8)? })
}

fn database_path(app: &tauri::App) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|err| err.to_string())?;
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir.join("akcyzaib.sqlite"))
}

fn init_db(app: &tauri::App) -> Result<AppDb, String> {
    let conn = Connection::open(database_path(app)?).map_err(|err| err.to_string())?;
    conn.execute_batch("PRAGMA foreign_keys = ON;").map_err(|err| err.to_string())?;
    conn.execute_batch(MIGRATION).map_err(|err| err.to_string())?;
    Ok(AppDb(Mutex::new(conn)))
}

fn now() -> String { chrono::Utc::now().to_rfc3339() }
fn validate_required(value: &str, label: &str) -> Result<(), String> { if value.trim().is_empty() { Err(format!("{label} is required")) } else { Ok(()) } }
fn empty_to_none(value: Option<String>) -> Option<String> { value.and_then(|item| { let trimmed = item.trim().to_string(); (!trimmed.is_empty()).then_some(trimmed) }) }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db = init_db(app).map_err(|err| Box::<dyn std::error::Error>::from(err))?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_products, get_product, create_product, update_product, list_competitors, create_competitor, get_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
