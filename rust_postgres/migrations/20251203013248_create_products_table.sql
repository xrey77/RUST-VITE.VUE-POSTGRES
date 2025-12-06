-- Add migration script here
CREATE TABLE public.products (
	id BIGINT NOT NULL,
	category varchar(255) NOT NULL,
	descriptions varchar(255) NOT NULL,
	qty int4 DEFAULT 0 NULL,
	unit varchar(255) NOT NULL,
	costprice numeric(10, 2) NOT NULL,
	sellprice numeric(10, 2) NOT NULL,
	saleprice numeric(10, 2) NOT NULL,
	alertstocks int4 DEFAULT 0 NULL,
	criticalstocks int4 DEFAULT 0 NULL,
	productpicture varchar(255) NULL,
	created_at timestamptz DEFAULT now() NULL,
	updated_at timestamptz DEFAULT now() NULL,
	CONSTRAINT products_pk PRIMARY KEY (id)
);