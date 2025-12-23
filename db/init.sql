CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE IF NOT EXISTS dining_halls (
  id BIGSERIAL PRIMARY KEY,
  osm_id TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL,
  lat DOUBLE PRECISION NOT NULL,
  lon DOUBLE PRECISION NOT NULL,
  cuisine TEXT,
  opening_hours TEXT
);

CREATE TABLE IF NOT EXISTS menu_items (
  id BIGSERIAL PRIMARY KEY,
  hall_id BIGINT NOT NULL REFERENCES dining_halls(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  brand TEXT,
  calories INTEGER,
  allergens TEXT[],
  vegan BOOLEAN,
  vegetarian BOOLEAN,
  popularity_score DOUBLE PRECISION NOT NULL DEFAULT 0
);

-- indexes for filtering
CREATE INDEX IF NOT EXISTS idx_menu_items_hall_id ON menu_items(hall_id);
CREATE INDEX IF NOT EXISTS idx_menu_items_vegan ON menu_items(vegan);
CREATE INDEX IF NOT EXISTS idx_menu_items_vegetarian ON menu_items(vegetarian);
CREATE INDEX IF NOT EXISTS idx_menu_items_calories ON menu_items(calories);
CREATE INDEX IF NOT EXISTS idx_halls_name_trgm ON dining_halls USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_items_name_trgm ON menu_items USING gin (name gin_trgm_ops);
