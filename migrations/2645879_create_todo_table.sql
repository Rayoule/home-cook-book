CREATE TABLE IF NOT EXISTS recipes
(
  id            INTEGER NOT NULL PRIMARY KEY,
  recipe_name   TEXT,
  recipe_tags   TEXT,
  recipe        TEXT
);