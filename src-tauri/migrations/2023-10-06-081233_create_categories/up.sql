CREATE TABLE categories (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR NOT NULL,
    description VARCHAR,
    monthly_budget DECIMAL(15,3) NOT NULL DEFAULT '0'
)
