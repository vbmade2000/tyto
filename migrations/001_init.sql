CREATE SCHEMA IF NOT EXISTS tyto;

CREATE TABLE IF NOT EXISTS tyto.links (
	id serial NOT NULL,
	address varchar(255) NOT NULL, /* Shortened URL part. Like XXXX in localhost"3000/XXXX */
	description varchar(255) NULL,
	banned bool NOT NULL DEFAULT false,
	target varchar(2040) NOT NULL, /* Which URL it will open. Like localhost"3000/XXXX opens www.linkedin.com/users/vbmade2000/ksvdjskdvj */
	visit_count int4 NOT NULL DEFAULT 0,
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT links_pkey PRIMARY KEY (id),
	CONSTRAINT address_unique UNIQUE(address)
);