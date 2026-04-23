CREATE OR REPLACE FUNCTION create_partition (base_name TEXT)
RETURNS void AS $$
DECLARE
    final_table_name TEXT;
BEGIN
    final_table_name := 'data.tmf_' || base_name;
    EXECUTE format('CREATE TABLE IF NOT EXISTS %I AS SELECT * FROM data.tmf LIMIT 1;', final_table_name);
    -- TRUNCATE TABLE final_table_name;
    -- ALTER TABLE final_table_name ADD PRIMARY KEY (id);
    -- ALTER TABLE final_table_name ALTER COLUMN id SET NOT NULL;
    -- ALTER TABLE final_table_name ALTER COLUMN href SET NOT NULL;
    -- ALTER TABLE final_table_name ALTER COLUMN module SET NOT NULL;
    -- ALTER TABLE final_table_name ALTER COLUMN json SET NOT NULL;
    -- ALTER TABLE data.tmf ATTACH PARTITION final_table_name FOR VALUES IN (base_name);

    RAISE NOTICE 'Table % created or already exists.', final_table_name;
END;
$$ LANGUAGE plpgsql;