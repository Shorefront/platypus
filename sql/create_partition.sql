CREATE OR REPLACE FUNCTION create_partition (module_name TEXT)
RETURNS void AS $$
DECLARE
	final_table_name VARCHAR(32);
BEGIN
	final_table_name := 'tmf_' || module_name;
	EXECUTE format('CREATE TABLE IF NOT EXISTS data.%I AS SELECT * FROM data.tmf LIMIT 1;',final_table_name);
	EXECUTE format('TRUNCATE TABLE data.%I',final_table_name);
	--EXECUTE format('ALTER TABLE data.%I ADD PRIMARY KEY (id)',final_table_name);
	EXECUTE format('ALTER TABLE data.%I ALTER COLUMN id SET NOT NULL',final_table_name);
	EXECUTE format('ALTER TABLE data.%I ALTER COLUMN href SET NOT NULL',final_table_name);
	EXECUTE format('ALTER TABLE data.%I ALTER COLUMN module SET NOT NULL',final_table_name);
    EXECUTE format('ALTER TABLE data.%I ALTER COLUMN json SET NOT NULL',final_table_name);
	EXECUTE format('ALTER TABLE data.tmf ATTACH PARTITION data.%I FOR VALUES IN (''%I'')',final_table_name,module_name);
	RAISE NOTICE 'Table % created or already exists', final_table_name;
END;
$$ LANGUAGE plpgsql;