CREATE TABLE
  employees (
    id UUID PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL
  );

INSERT INTO
  employees (id, first_name, last_name)
VALUES
  (
    'dbac4cf5-e5b0-4a4f-ae83-b5bd356c46dc',
    'philip',
    'janik'
  );

INSERT INTO
  employees (id, first_name, last_name)
VALUES
  (
    'c011824f-3312-4034-b8b5-5b5cefb17461',
    'john',
    'doe'
  );