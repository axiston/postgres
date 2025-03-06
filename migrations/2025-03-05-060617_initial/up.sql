CREATE OR REPLACE FUNCTION on_updated_at() RETURNS TRIGGER AS
$$
BEGIN
    IF (
        new IS DISTINCT FROM old AND
        new.updated_at IS NOT DISTINCT FROM old.updated_at
        ) THEN
        new.updated_at := current_timestamp;
    END IF;
    RETURN new;
EXCEPTION
    WHEN OTHERS THEN
        RAISE EXCEPTION 'Error updating updated_at column: %', sqlerrm;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION on_updated_at() IS
    'Updates the `updated_at` timestamp if it was not explicitly modified.';

CREATE OR REPLACE FUNCTION manage_updated_at(_tbl REGCLASS) RETURNS VOID AS
$$
BEGIN
    EXECUTE format(
            'CREATE OR REPLACE TRIGGER %s_manage_updated_at
             BEFORE UPDATE ON %s
             FOR EACH ROW EXECUTE FUNCTION on_updated_at()',
            _tbl, _tbl
            );
EXCEPTION
    WHEN OTHERS THEN
        RAISE EXCEPTION 'Error managing updated_at trigger for table %: %', _tbl, sqlerrm;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION manage_updated_at(_tbl REGCLASS) IS
    'Configures a trigger to manage the `updated_at` timestamp.';
