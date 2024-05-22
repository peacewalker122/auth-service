CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    modified_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    auth_provider VARCHAR(255),
    auth_provider_user_id VARCHAR(255),
    secret VARCHAR(255),
    password VARCHAR(255) NOT NULL
);
