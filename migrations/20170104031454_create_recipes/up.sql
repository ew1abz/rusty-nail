CREATE TABLE recipes (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  CONSTRAINT name_unique UNIQUE (name)
);

CREATE TABLE ingredients (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  available BOOLEAN NOT NULL,
  CONSTRAINT name_unique UNIQUE (name)
);

CREATE TABLE recipe_ingredients (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  recipe_id INTEGER NOT NULL,
  ingredient_id INTEGER NOT NULL,
  amount REAL NOT NULL
);
