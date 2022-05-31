-- Add migration script here
CREATE TABLE channels(
    user_id VARCHAR(512) NOT NULL,
    service_id VARCHAR(50) NOT NULL,
    service_username VARCHAR(512) NOT NULL,
    PRIMARY KEY (user_id, service_id, service_username)
)