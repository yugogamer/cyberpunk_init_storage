-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS accounts (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4(),
    "email" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);
CREATE TABLE auth (
    password TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS groupes (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    owner_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS characters (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    groupe_id uuid NOT NULL REFERENCES groupes(id) ON DELETE CASCADE,
    base_ref INT NOT NULL,
    modifier INT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS tokens (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    token TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS current_groupe(
    id_token uuid NOT NULL REFERENCES tokens(id) ON DELETE CASCADE,
    id_groupe uuid NOT NULL REFERENCES groupes(id) ON DELETE CASCADE,
    PRIMARY KEY (id_token, id_groupe)
);