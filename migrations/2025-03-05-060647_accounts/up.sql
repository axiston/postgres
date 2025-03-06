CREATE TABLE accounts
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    display_name  TEXT        NOT NULL,
    email_address TEXT        NOT NULL,
    password_hash TEXT        NOT NULL,

    -- Validation constraints to prevent empty or invalid entries.
    CONSTRAINT accounts_non_empty_display_name CHECK (display_name <> ''),
    CONSTRAINT accounts_non_empty_email_address CHECK (email_address <> ''),
    CONSTRAINT accounts_non_empty_password_hash CHECK (password_hash <> ''),

    -- Indicates whether the account has been activated.
    is_activated  BOOL        NOT NULL DEFAULT FALSE,

    -- Timestamps for tracking record lifecycle.
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
    'Manages user account information, including security and lifecycle tracking.';
COMMENT ON COLUMN accounts.id IS
    'Unique identifier for each account.';
COMMENT ON COLUMN accounts.display_name IS
    'Publicly visible name used for user identification.';
COMMENT ON COLUMN accounts.email_address IS
    'Unique email address used for authentication.';
COMMENT ON COLUMN accounts.password_hash IS
    'Securely hashed password used for authentication.';

---

CREATE TABLE account_sessions
(
    access_seq UUID        NOT NULL DEFAULT gen_random_uuid(),
    update_seq UUID        NOT NULL DEFAULT gen_random_uuid(),

    account_id UUID        NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    region_id  CHAR(2)     NOT NULL DEFAULT 'A0',

    -- Ensures unique session identifiers per account.
    CONSTRAINT account_sessions_pkey PRIMARY KEY (account_id, access_seq),
    -- Ensures the region code consists of exactly two alphanumeric characters.
    CONSTRAINT account_sessions_region_alphanumeric CHECK (region_id ~ '^[A-Z0-9]{2}$'),

    -- Security-related metadata.
    ip_address INET        NOT NULL,
    user_agent TEXT        NOT NULL,

    -- Timestamps for tracking session lifecycle.
    issued_at  TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    expired_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp + INTERVAL '7 days',
    deleted_at TIMESTAMPTZ          DEFAULT NULL,

    -- Ensures logical timestamp progression.
    CONSTRAINT account_sessions_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_sessions_deleted_after_issued CHECK (deleted_at IS NULL OR deleted_at >= issued_at)
);

CREATE INDEX account_sessions_access_seq_idx
    ON account_sessions (account_id, access_seq)
    WHERE deleted_at IS NULL;

CREATE INDEX account_sessions_update_seq_idx
    ON account_sessions (account_id, update_seq)
    WHERE deleted_at IS NULL;

COMMENT ON TABLE account_sessions IS
    'Tracks user sessions with unique identifiers and security metadata.';
COMMENT ON COLUMN account_sessions.access_seq IS
    'Unique identifier for each session.';
COMMENT ON COLUMN account_sessions.update_seq IS
    'Unique identifier for session refresh tokens.';
COMMENT ON COLUMN account_sessions.account_id IS
    'Reference to the associated user account.';
COMMENT ON COLUMN account_sessions.region_id IS
    'Two-character region code indicating session origin (e.g., "US", "EU").';

---

CREATE TABLE account_permissions
(
    account_id  UUID PRIMARY KEY REFERENCES accounts (id) ON DELETE CASCADE,
    permissions JSONB       NOT NULL DEFAULT '{}'::JSONB,

    -- Timestamps for tracking record lifecycle.
    created_at  TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT account_permissions_updated_after_created CHECK (updated_at >= created_at)
);

SELECT manage_updated_at('account_permissions');

COMMENT ON TABLE account_permissions IS
    'Defines granular permissions for each user account.';
COMMENT ON COLUMN account_permissions.account_id IS
    'Reference to the associated account.';
COMMENT ON COLUMN account_permissions.permissions IS
    'JSON object containing permission flags (e.g., region:action).';

---

CREATE TYPE TOKEN_ACTION AS ENUM (
    'activate_account', -- Verify and enable a new account.
    'deactivate_account', -- Disable or suspend an account.
    'update_email', -- Change the registered email address.
    'reset_password' -- Initiate password recovery.
    );

COMMENT ON TYPE TOKEN_ACTION IS
    'Enumerates the types of action tokens used for critical operations.';

---

CREATE TABLE account_tokens
(
    action_token UUID         NOT NULL DEFAULT gen_random_uuid(),
    account_id   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    action_type  TOKEN_ACTION NOT NULL,
    action_data  JSONB        NOT NULL DEFAULT '{}'::JSONB,

    -- Ensures unique action tokens per account.
    CONSTRAINT account_tokens_pkey PRIMARY KEY (account_id, action_token),
    -- Limits metadata size to prevent excessive storage usage.
    CONSTRAINT account_tokens_action_data_limit CHECK (length(action_data::TEXT) <= 2048),

    -- Security-related metadata.
    ip_address   INET         NOT NULL,
    user_agent   TEXT         NOT NULL,

    -- Timestamps for tracking token lifecycle.
    issued_at    TIMESTAMPTZ  NOT NULL DEFAULT current_timestamp,
    expired_at   TIMESTAMPTZ  NOT NULL DEFAULT current_timestamp + INTERVAL '7 days',
    used_at      TIMESTAMPTZ           DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT account_tokens_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_tokens_used_after_issued CHECK (used_at IS NULL OR used_at >= issued_at),
    CONSTRAINT account_tokens_expired_after_used CHECK (expired_at IS NULL OR used_at IS NULL OR expired_at >= used_at)
);

CREATE INDEX account_tokens_idx
    ON account_tokens (account_id, action_token)
    WHERE used_at IS NULL;

COMMENT ON TABLE account_tokens IS
    'Manages secure, time-limited action tokens for critical operations.';
COMMENT ON COLUMN account_tokens.action_token IS
    'Unique identifier for each action token.';
COMMENT ON COLUMN account_tokens.account_id IS
    'Reference to the associated account.';
COMMENT ON COLUMN account_tokens.action_type IS
    'Specifies the type of action (e.g., confirm email, reset password).';
COMMENT ON COLUMN account_tokens.action_data IS
    'Contains additional metadata related to the token (e.g., target email, source IP).';
