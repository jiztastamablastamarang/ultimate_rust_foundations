CREATE TABLE IF NOT EXISTS timeseries
(
    id           SERIAL PRIMARY KEY,
    collector_id VARCHAR(255),
    received     TIMESTAMP,
    total_memory UNSIGNED BIGINT,
    used_memory  UNSIGNED BIGINT,
    average_cpu  FLOAT
)