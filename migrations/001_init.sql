--- Create a tyto schema
CREATE SCHEMA IF NOT EXISTS tyto;

-- Create table users
CREATE TABLE IF NOT EXISTS tyto.users (
	id bigserial NOT NULL, /* Unique ID for a user. */
	apikey varchar(255) NULL, /* API Key for programatic calling REST API. For future use. */  
	banned bool NOT NULL DEFAULT false, /* Indicates if user is banned for creating Tiny URL.*/
	email varchar(255) NOT NULL, /* User's email. */
	"password" varchar(255) NOT NULL, /* User's Password. */
	deleted bool NOT NULL DEFAULT false, /* Indicates if user is soft deleted. */
	activation_code varchar(255), /* Activation code sent to user. */
	activation_code_generated_at timestamptz, /* Timestamp indicating when account activation is done. */
	activated bool NOT NULL DEFAULT false, /* Indicates if user is activated. */
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP, /* Timestamp indicating when account is created.*/
	updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP, /* Timestamp indicating when account is updated.*/
	CONSTRAINT users_email_unique UNIQUE (email),
	CONSTRAINT users_pkey PRIMARY KEY (id)
);

-- Create table urls
CREATE TABLE IF NOT EXISTS tyto.urls (
	id bigserial NOT NULL, /* Unique ID for a URL. */
	address varchar(255) NOT NULL, /* Shortened URL part. Like XXXX in localhost"3000/XXXX */
	description varchar(255) NULL, /* Description for a URL record. */
	banned bool NOT NULL DEFAULT false, /* Indicates if URL is banned. */
	target varchar(2040) NOT NULL, /* URL to be opened. Like localhost"3000/XXXX opens www.linkedin.com/users/vbmade2000/ksvdjskdvj */
	visit_count int4 NOT NULL DEFAULT 0, /* Number of visits paid to a URL. */
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP, /* Timestamp indicating when URL is created. */
	updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP, /* Timestamp indicating when URL is updated. */
	user_id bigserial references tyto.users(id), /* Reference to a User the URL belongs to. */
	CONSTRAINT urls_pkey PRIMARY KEY (id),
	CONSTRAINT address_unique UNIQUE(address)
);

