CREATE TYPE buchungs_intervall AS ENUM ('taeglich', 'woechentlich', 'monatlich', 'jaehrlich', 'einmalig');
CREATE TYPE art_enum AS ENUM ('einahmen', 'ausgaben'); -- 'art' ist ein riskanter Name, besser 'art_type' oder ähnlich

CREATE TABLE IF NOT EXISTS buchung (
    id SERIAL PRIMARY KEY,
    datum DATE NOT NULL,
    bezeichnung TEXT NOT NULL,
    betrag DOUBLE PRECISION NOT NULL,
    intervall buchungs_intervall NOT NULL, -- Hier den neuen Typ nutzen
    art art_enum NOT NULL                  -- Hier den neuen Typ nutzen
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