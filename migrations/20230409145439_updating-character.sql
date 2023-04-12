-- Add migration script here
ALTER TABLE characters DROP COLUMN groupe_id;
ALTER TABLE characters DROP COLUMN active;
CREATE TABLE invitations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES accounts(id),
    groupe_id INTEGER REFERENCES groupes(id),
    accepted BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE TABLE groupes_access(
    id_groupe INTEGER NOT NULL REFERENCES groupes(id) ON DELETE CASCADE,
    id_user INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id_groupe, id_user)
);
CREATE TABLE assets (
    id SERIAL PRIMARY KEY,
    original_name TEXT NOT NULL,
    bucket_name TEXT NOT NULL,
    upload_date TIMESTAMP NOT NULL DEFAULT NOW(),
    uploader_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE
);
ALTER TABLE characters
ADD COLUMN asset_id INTEGER REFERENCES assets(id) ON DELETE
SET NULL;