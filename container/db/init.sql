CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL UNIQUE,
    role VARCHAR(20) NOT NULL CHECK (role IN ('USER', 'ADMIN')),
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    exp BIGINT NOT NULL,
    iat BIGINT NOT NULL,
    typ VARCHAR(20) NOT NULL CHECK (typ IN ('access', 'refresh')),
    family UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,

    geo_data GEOMETRY(GEOMETRY, 4326) NOT NULL -- komplexe darstellung: punkt, line, polygon, multi-polygon
);

---- Testdaten
INSERT INTO locations (title, description, geo_data)
VALUES (
           'Hallescher Marktplatz',
           'Zentraler Punkt in der Innenstadt von Halle',
           ST_SetSRID(ST_MakePoint(11.974, 51.482), 4326)
       );

INSERT INTO locations (title, description, geo_data)
VALUES (
           'Saale-Uferweg',
           'Wanderweg entlang der Saale durch Halle',
           ST_GeomFromText('LINESTRING(11.950 51.510, 11.965 51.495, 11.980 51.480, 11.990 51.465)', 4326)
       );

INSERT INTO locations (title, description, geo_data)
VALUES (
           'Stadtpark-Rundweg',
           'Rundweg um den Stadtpark in Halle',
           ST_GeomFromText('LINESTRING(11.960 51.490, 11.970 51.488, 11.975 51.495, 11.965 51.498, 11.960 51.490)', 4326)
       );
INSERT INTO locations (title, description, geo_data)
VALUES (
           'Botanischer Garten',
           'Gelände des Botanischen Gartens der Martin-Luther-Universität',
           ST_GeomFromText('POLYGON((11.960 51.485, 11.970 51.484, 11.975 51.490, 11.965 51.492, 11.960 51.485))', 4326)
       );
INSERT INTO locations (title, description, geo_data)
VALUES (
           'Sehenswürdigkeiten Halle',
           'Wichtige Sehenswürdigkeiten in Halle (Marktplatz, Händel-Haus, Dom)',
           ST_GeomFromText('MULTIPOINT((11.974 51.482), (11.965 51.478), (11.970 51.484))', 4326)
       );
INSERT INTO locations (title, description, geo_data)
VALUES (
           'Straßenbahnnetz',
           'Ausgewählte Straßenbahnlinien in Halle',
           ST_GeomFromText('MULTILINESTRING((11.950 51.510, 11.965 51.495, 11.980 51.480), (11.960 51.515, 11.975 51.500, 11.990 51.485))', 4326)
       );
INSERT INTO locations (title, description, geo_data)
VALUES (
           'Parkanlagen',
           'Verschiedene Grünflächen und Parks in Halle',
           ST_GeomFromText('MULTIPOLYGON(((11.960 51.485, 11.970 51.484, 11.975 51.490, 11.965 51.492, 11.960 51.485)), ((11.990 51.470, 12.000 51.468, 12.005 51.475, 11.995 51.478, 11.990 51.470)))', 4326)
       );
