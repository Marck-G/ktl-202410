CREATE TABLE PRS_main (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() UNIQUE NOT NULL,
    customer_id UUID UNIQUE,
    given_name VARCHAR(100) NOT NULL,
    family_name VARCHAR(200) NOT NULL,
    additional_name VARCHAR(200),
    birth_date DATE,
    gender VARCHAR(1),
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    date_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('PRS_main');