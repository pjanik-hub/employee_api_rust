-- Your SQL goes here
create table
  employees (
    id uuid primary key,
    first_name varchar(50) not null,
    last_name varchar(50) not null
  )