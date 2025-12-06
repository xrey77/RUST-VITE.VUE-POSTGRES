-- Add migration script here
CREATE TABLE public.users (
	id BIGINT NOT NULL,
	firstname varchar(32) NOT NULL,
	lastname varchar(32) NOT NULL,
	email varchar(100) NOT NULL,
	mobile varchar(32) NULL,
	username varchar(32) NOT NULL,
	"password" varchar(200) NOT NULL,
	roles varchar(10) DEFAULT 'ROLE_USER'::character varying NULL,
	isactivated int4 DEFAULT 0 NULL,
	isblocked int4 DEFAULT 0 NULL,
	mailtoken int4 DEFAULT 0 NULL,
	userpic varchar(100) DEFAULT 'pix.png'::character varying NULL,
	secret text NULL,
	qrcodeurl text NULL,
	created_at timestamptz DEFAULT now() NULL,
	updated_at timestamptz DEFAULT now() NULL,
	CONSTRAINT users_email_key UNIQUE (email),
	CONSTRAINT users_pkey PRIMARY KEY (id),
	CONSTRAINT users_username_key UNIQUE (username)
);