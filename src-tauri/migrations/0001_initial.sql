CREATE TABLE IF NOT EXISTS products (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sku TEXT NOT NULL,
  name TEXT NOT NULL,
  allegro_offer_id TEXT,
  ean TEXT,
  category TEXT NOT NULL DEFAULT '',
  brand TEXT,
  status TEXT NOT NULL DEFAULT 'draft',
  purchase_price REAL NOT NULL DEFAULT 0,
  selling_price REAL NOT NULL DEFAULT 0,
  currency TEXT NOT NULL DEFAULT 'PLN',
  stock_quantity INTEGER NOT NULL DEFAULT 0,
  min_stock_quantity INTEGER NOT NULL DEFAULT 0,
  url TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS competitors (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  marketplace TEXT NOT NULL DEFAULT 'Allegro',
  url TEXT NOT NULL DEFAULT '',
  rating REAL,
  review_count INTEGER,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS competitor_offers (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  product_id INTEGER NOT NULL,
  competitor_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  url TEXT NOT NULL DEFAULT '',
  price REAL NOT NULL DEFAULT 0,
  currency TEXT NOT NULL DEFAULT 'PLN',
  delivery_price REAL,
  availability TEXT,
  last_checked_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE CASCADE,
  FOREIGN KEY(competitor_id) REFERENCES competitors(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL, updated_at TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, description TEXT, status TEXT NOT NULL DEFAULT 'open', priority TEXT NOT NULL DEFAULT 'medium', related_entity_type TEXT, related_entity_id INTEGER, due_date TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS app_events (id INTEGER PRIMARY KEY AUTOINCREMENT, event_type TEXT NOT NULL, entity_type TEXT, entity_id INTEGER, payload TEXT, created_at TEXT NOT NULL);
