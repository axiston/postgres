-- Enables the extensions in the specified database.
CREATE EXTENSION IF NOT EXISTS pg_cron;

-- Optionally, grant usage to regular users.
-- GRANT USAGE ON SCHEMA cron TO marco;

-- Vacuum every day at 10:00am (GMT)
SELECT cron.schedule('nightly-vacuum', '0 10 * * *', 'VACUUM');
