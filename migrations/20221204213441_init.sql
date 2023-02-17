-- Add migration script here
CREATE TABLE IF NOT EXISTS accounts (
    id serial NOT NULL,
    "email" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    UNIQUE ("email")
);
CREATE TABLE auth (
    password TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS groupes (
    id serial NOT NULL,
    name TEXT NOT NULL,
    owner_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS characters (
    id serial NOT NULL,
    name TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    groupe_id INTEGER NOT NULL REFERENCES groupes(id),
    base_ref INT NOT NULL,
    modifier INT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS active_in_groups(
    id_characters INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    id_groupe INTEGER NOT NULL REFERENCES groupes(id) ON DELETE CASCADE,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id_characters, id_groupe)
);
CREATE TABLE IF NOT EXISTS tokens (
    id serial NOT NULL,
    token TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS current_groupe(
    id_token INTEGER NOT NULL REFERENCES tokens(id) ON DELETE CASCADE,
    id_groupe INTEGER NOT NULL REFERENCES groupes(id) ON DELETE CASCADE,
    PRIMARY KEY (id_token, id_groupe)
);