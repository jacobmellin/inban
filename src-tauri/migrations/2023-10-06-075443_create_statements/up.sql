CREATE TABLE statements (
    id INT NOT NULL PRIMARY KEY,
    date_inserted TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    hash VARCHAR(128) NOT NULL,
    date_booked TIMESTAMP NOT NULL,
    valuata TIMESTAMP NOT NULL,
    bank VARCHAR NOT NULL,
    iban VARCHAR,
    reason VARCHAR,
    reference VARCHAR,
    balance_before DECIMAL(15,3) NOT NULL DEFAULT '0',
    balance_after DECIMAL(15,3) NOT NULL DEFAULT '0',
    account_id VARCHAR NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);
