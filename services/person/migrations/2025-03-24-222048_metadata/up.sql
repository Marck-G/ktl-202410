CREATE TABLE PRS_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() UNIQUE NOT NULL,
    person_id UUID NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    date_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (person_id) REFERENCES PRS_main(id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('PRS_metadata');
