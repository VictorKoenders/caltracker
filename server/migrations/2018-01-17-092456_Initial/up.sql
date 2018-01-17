-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE Day (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	year SMALLINT NOT NULL,
	month SMALLINT NOT NULL,
	day_of_month SMALLINT NOT NULL,
	UNIQUE(year, month, day_of_month)
);

CREATE TABLE DayEntry (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	day UUID REFERENCES Day (id) NOT NULL,
	name TEXT NOT NULL,
	value FLOAT NOT NULL
);

