CREATE TABLE IF NOT EXISTS tbl_item (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    table_id INT NOT NULL,
    time_to_prepare INT NOT NULL,
    quantity INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON tbl_item(name, table_id);
COMMENT ON TABLE tbl_item IS 'Table for storing items ordered in restaurant';
COMMENT ON COLUMN tbl_item.id IS 'Primary key for tbl_item';
COMMENT ON COLUMN tbl_item.name IS 'Description of the item';
COMMENT ON COLUMN tbl_item.table_id IS 'Table number where the item is ordered';
COMMENT ON COLUMN tbl_item.time_to_prepare IS 'Time in minutes to prepare the item';
COMMENT ON COLUMN tbl_item.quantity IS 'Quantity of the ordered items. Default is 1 if not specified';
COMMENT ON COLUMN tbl_item.created_at IS 'Technical column to store the time of creation of the item';