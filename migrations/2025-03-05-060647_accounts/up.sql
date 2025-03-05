CREATE TABLE accounts
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    display_name  TEXT        NOT NULL,
    email_address TEXT        NOT NULL,
    password_hash TEXT        NOT NULL,

    --- Validation constraints to prevent empty or invalid entries.
    CONSTRAINT accounts_non_empty_display_name CHECK (display_name <> ''),
    CONSTRAINT accounts_non_empty_email_address CHECK (email_address <> ''),
    CONSTRAINT accounts_non_empty_password_hash CHECK (password_hash <> ''),

    -- Account activation status for additional security.
    is_activated  BOOL        NOT NULL DEFAULT FALSE,

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at    TIMESTAMPTZ          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT accounts_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT accounts_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT accounts_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

SELECT manage_updated_at('accounts');

CREATE UNIQUE INDEX accounts_email_address_idx
    ON accounts (email_address)
    WHERE deleted_at IS NULL;

CREATE INDEX accounts_credentials_idx
    ON accounts (email_address, password_hash)
    WHERE deleted_at IS NULL;

COMMENT ON TABLE accounts IS
    'Account data management with security and lifecycle tracking.';
COMMENT ON COLUMN accounts.id IS
    'Unique identifier for each account, used as a public resource.';

COMMENT ON COLUMN accounts.display_name IS
    'Publicly visible name, used for user identification.';
COMMENT ON COLUMN accounts.email_address IS
    'Unique email address, used for authentication.';
COMMENT ON COLUMN accounts.password_hash IS
    'Securely hashed password, used for authentication.';
