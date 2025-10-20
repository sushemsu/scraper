CREATE DATABASE pulse;
\c pulse;
CREATE TABLE IF NOT EXISTS charts (
	host TEXT,
	cpu DOUBLE PRECISION,
	disk DOUBLE PRECISION,
	diskread BIGINT,
	diskwrite BIGINT,
	memory DOUBLE PRECISION,
	netin BIGINT,
	netout BIGINT,
	tstamp BIGINT
);

CREATE TABLE IF NOT EXISTS nodes (
	host TEXT,
	cpu DOUBLE PRECISION,
	disk DOUBLE PRECISION,
	memory DOUBLE PRECISION
);
CREATE TABLE IF NOT EXISTS stats (
	oldestdatatimestamp BIGINT
);
CREATE TABLE IF NOT EXISTS storage (
	hostdisk TEXT,
	disk DOUBLE PRECISION
);
CREATE TABLE IF NOT EXISTS time_stamp (
	time_stamp BIGINT
);
