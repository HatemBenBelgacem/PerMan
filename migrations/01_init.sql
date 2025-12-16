-- Tabelle Buchung
CREATE TABLE IF NOT EXISTS buchung (
    id SERIAL PRIMARY KEY, -- 'INTEGER PRIMARY KEY AUTOINCREMENT' -> 'SERIAL PRIMARY KEY'
    datum DATE NOT NULL,
    bezeichnung TEXT NOT NULL,
    betrag DOUBLE PRECISION NOT NULL, -- 'FLOAT' ist ok, 'DOUBLE PRECISION' ist expliziter in Postgres
    intervall TEXT NOT NULL,
    art TEXT NOT NULL
);

-- Tabelle Periode
CREATE TABLE IF NOT EXISTS periode (
    id SERIAL PRIMARY KEY,
    bezeichnung DATE NOT NULL
);

-- Tabelle Typ
CREATE TABLE IF NOT EXISTS typ (
    id SERIAL PRIMARY KEY,
    bezeichnung DATE NOT NULL -- Prüfen Sie, ob hier wirklich DATE gemeint ist oder TEXT?
);

-- Tabelle Benutzer
CREATE TABLE IF NOT EXISTS benutzer (
    id SERIAL PRIMARY KEY,
    benutzername TEXT NOT NULL,
    email TEXT NOT NULL,
    passwort TEXT NOT NULL
);

-- Tabelle Abo
CREATE TABLE IF NOT EXISTS abo (
    id TEXT PRIMARY KEY UNIQUE, -- TEXT als ID ist okay (z.B. für UUIDs)
    bezeichnung TEXT NOT NULL,
    beginn DATE NOT NULL,
    dauer DOUBLE PRECISION NOT NULL,
    knd_frist DOUBLE PRECISION NOT NULL
);