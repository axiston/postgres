CREATE TABLE projects
(
    id           UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    display_name TEXT        NOT NULL DEFAULT 'Untitled',
    metadata     JSONB       NOT NULL DEFAULT '{}'::JSONB,

    -- Validation constraints to prevent empty or invalid entries.
    CONSTRAINT projects_non_empty_display_name CHECK (display_name <> ''),
    -- Limits the size of the metadata field to prevent excessively large JSON data.
    CONSTRAINT projects_metadata_limit CHECK (length(metadata::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMPTZ          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT projects_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT projects_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT projects_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

SELECT manage_updated_at('projects');

COMMENT ON TABLE projects IS
    'Manages projects with metadata tracking.';
COMMENT ON COLUMN projects.id IS
    'Unique identifier for each project.';
COMMENT ON COLUMN projects.display_name IS
    'User-defined project name.';
COMMENT ON COLUMN projects.metadata IS
    'Additional project metadata in JSON format (e.g., description, tags).';

---

CREATE TYPE PROJECT_ROLE AS ENUM (
    'owner', 'member'
    );

COMMENT ON TYPE PROJECT_ROLE IS
    'Defines possible roles a user can have in a project.';

---

CREATE TABLE project_members
(
    project_id   UUID         NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    account_id   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    account_role PROJECT_ROLE NOT NULL DEFAULT 'member',

    -- Ensures each project-account pair is unique.
    CONSTRAINT project_members_pkey PRIMARY KEY (project_id, account_id),

    show_order   INT          NOT NULL DEFAULT 0,
    is_pinned    BOOLEAN      NOT NULL DEFAULT FALSE,
    is_hidden    BOOLEAN      NOT NULL DEFAULT FALSE,

    created_by   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    updated_by   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking the membership record lifecycle.
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT current_timestamp,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT project_members_updated_after_created CHECK (updated_at >= created_at)
);

CREATE INDEX project_members_account_id_idx
    ON project_members (account_id);
CREATE INDEX project_members_project_id_idx
    ON project_members (project_id);

SELECT manage_updated_at('project_members');

COMMENT ON TABLE project_members IS
    'Manages project memberships and permissions.';
COMMENT ON COLUMN project_members.project_id IS
    'Reference to the associated project.';
COMMENT ON COLUMN project_members.account_id IS
    'Reference to the associated account.';
COMMENT ON COLUMN project_members.account_role IS
    'Role of the member within the project (default: member).';

COMMENT ON COLUMN project_members.show_order IS
    'Defines the display order of projects for the user.';
COMMENT ON COLUMN project_members.is_pinned IS
    'Indicates whether the project is pinned for quick access.';
COMMENT ON COLUMN project_members.is_hidden IS
    'Indicates whether the project is hidden from the user’s dashboard.';

COMMENT ON COLUMN project_members.created_by IS
    'Tracks the creator of this membership record.';
COMMENT ON COLUMN project_members.updated_by IS
    'Tracks the last updater of this membership record.';

---

CREATE TYPE INVITE_STATUS AS ENUM (
    'pending', 'accepted', 'declined', 'canceled'
    );

COMMENT ON TYPE INVITE_STATUS IS
    'Defines possible statuses for project invitations.';

---

CREATE TABLE project_invites
(
    invite_id     UUID          NOT NULL DEFAULT gen_random_uuid(),
    project_id    UUID          NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    account_id    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    invite_status INVITE_STATUS NOT NULL DEFAULT 'pending',

    -- Ensures each project-invite pair is unique.
    CONSTRAINT project_invites_pkey PRIMARY KEY (project_id, invite_id),

    created_by    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    updated_by    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking the invite lifecycle.
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMPTZ   NOT NULL DEFAULT current_timestamp,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT project_invites_updated_after_created CHECK (updated_at >= created_at)
);

SELECT manage_updated_at('project_invites');

COMMENT ON TABLE project_invites IS
    'Manages invitations for users to join projects.';
COMMENT ON COLUMN project_invites.invite_id IS
    'Unique identifier for each invitation.';
COMMENT ON COLUMN project_invites.project_id IS
    'Reference to the associated project.';
COMMENT ON COLUMN project_invites.account_id IS
    'Reference to the invited account.';
COMMENT ON COLUMN project_invites.invite_status IS
    'Current status of the invitation (default: pending).';

COMMENT ON COLUMN project_invites.created_by IS
    'Tracks who sent the invitation (e.g., an admin or project owner).';
COMMENT ON COLUMN project_invites.updated_by IS
    'Tracks who last updated the invitation (e.g., the invitee or an admin).';

ALTER TYPE TOKEN_ACTION ADD VALUE 'pending_invite';

CREATE INDEX project_invites_account_id_idx
    ON project_invites (account_id);
CREATE INDEX project_invites_project_id_idx
    ON project_invites (project_id);

---

CREATE TABLE project_schedules
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    project_id UUID        NOT NULL REFERENCES projects (id) ON DELETE CASCADE,

    interval   INTEGER     NOT NULL DEFAULT 3600,
    metadata   JSONB       NOT NULL DEFAULT '{}'::JSONB,

    -- Ensures the update interval is at least 1 second.
    CONSTRAINT project_schedules_interval_non_zero CHECK (interval > 0),
    -- Limits metadata size to prevent excessively large JSON data.
    CONSTRAINT project_schedules_metadata_limit CHECK (length(metadata::TEXT) <= 2048),

    -- Timestamps for tracking the schedule lifecycle.
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT project_schedules_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT project_schedules_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT project_schedules_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

SELECT manage_updated_at('project_schedules');

COMMENT ON TABLE project_schedules IS
    'Manages schedules associated with projects.';
COMMENT ON COLUMN project_schedules.id IS
    'Unique identifier for each schedule.';
COMMENT ON COLUMN project_schedules.project_id IS
    'Reference to the associated project.';
COMMENT ON COLUMN project_schedules.interval IS
    'Defines the schedule update interval (in seconds).';
COMMENT ON COLUMN project_schedules.metadata IS
    'User-provided metadata (e.g., cron expression, tags).';

---

CREATE TABLE project_webhooks
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    project_id UUID        NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    metadata   JSONB       NOT NULL DEFAULT '{}'::JSONB,

    -- Limits metadata size to prevent excessively large JSON data.
    CONSTRAINT project_webhooks_metadata_limit CHECK (length(metadata::TEXT) <= 2048),

    -- Timestamps for tracking the webhook lifecycle.
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT project_webhooks_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT project_webhooks_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT project_webhooks_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

SELECT manage_updated_at('project_webhooks');

COMMENT ON TABLE project_webhooks IS
    'Manages webhooks associated with projects.';
COMMENT ON COLUMN project_webhooks.id IS
    'Unique identifier for each webhook.';
COMMENT ON COLUMN project_webhooks.project_id IS
    'Reference to the associated project.';
COMMENT ON COLUMN project_webhooks.metadata IS
    'User-provided metadata (e.g., event triggers, tags).';
