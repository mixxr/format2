DROP TABLE IF EXISTS "certificate";
DROP TABLE IF EXISTS "ticker";
DROP TABLE IF EXISTS "quote";

CREATE TABLE certificate (
    isin TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    tickers TEXT NOT NULL, -- string: ticker1, ticker2, ..., tickerN,
    start_date TEXT NOT NULL, -- string: 'YYYY-MM-DD'
    end_date TEXT NOT NULL -- string: 'YYYY-MM-DD'
);

CREATE TABLE ticker (
    ticker TEXT PRIMARY KEY, -- market:symbol
    market TEXT NOT NULL, -- BIT, NASDAC, ...
    name TEXT NOT NULL,
    sectors TEXT NOT NULL -- string: sector1, ..., sectorN,
);

CREATE TABLE quote (
    isin TEXT NOT NULL,
    obs_dt TEXT NOT NULL, -- ISO8601 string: 'YYYY-MM-DD HH:MM:SS'
    ask DECIMAL NOT NULL DEFAULT 0.0,
    bid DECIMAL NOT NULL DEFAULT 0.0, 
    currency TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (isin, obs_dt)
);

