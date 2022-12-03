-- Your SQL goes here
CREATE SEQUENCE hibernate_sequence INCREMENT BY 1 START WITH 1 MAXVALUE 9223372036854775807 NO CYCLE;

CREATE TABLE car (
  id int8 PRIMARY KEY,
  vin varchar(40) NOT NULL,
  make varchar(255) NOT NULL,
  model varchar(255) NOT NULL,
  year int4 NOT NULL,
  color varchar(20) NOT NULL,
  price numeric(19, 2) NOT NULL,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('car');
