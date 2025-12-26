

CREATE TYPE intervall AS ENUM ('taeglich', 'woechentlich', 'monatlich', 'jaehrlich', 'einmalig');
CREATE TYPE art AS ENUM ('einahmen', 'ausgaben'); -- 'art' ist ein riskanter Name, besser 'art_type' oder ähnlich

CREATE TABLE IF NOT EXISTS buchung (
    id UUID PRIMARY KEY,
    datum DATE NOT NULL,
    bezeichnung TEXT NOT NULL,
    betrag DOUBLE PRECISION NOT NULL,
    intervall intervall NOT NULL, -- Hier den neuen Typ nutzen
    art art NOT NULL                  -- Hier den neuen Typ nutzen
);


-- Tabelle Benutzer
CREATE TABLE IF NOT EXISTS benutzer (
    id UUID PRIMARY KEY UNIQUE,
    benutzername TEXT NOT NULL,
    email TEXT NOT NULL,
    passwort TEXT NOT NULL
);

-- Tabelle Abo
CREATE TABLE IF NOT EXISTS abo (
    id UUID PRIMARY KEY, -- TEXT als ID ist okay (z.B. für UUIDs)
    bezeichnung TEXT NOT NULL,
    beginn DATE NOT NULL,
    dauer DOUBLE PRECISION NOT NULL,
    knd_frist DOUBLE PRECISION NOT NULL
);