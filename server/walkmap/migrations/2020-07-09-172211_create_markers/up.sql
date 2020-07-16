CREATE TABLE markers (
  id TEXT NOT NULL PRIMARY KEY,
  map_id TEXT REFERENCES maps(id) NOT NULL,
  order_parameter FLOAT NOT NULL,
  lat FLOAT NOT NULL,
  lon FLOAT NOT NULL,
  annotation TEXT,
  image_url TEXT
);