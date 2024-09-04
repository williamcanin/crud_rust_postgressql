# :crab: CRUD example1 between [Rust](https://www.rust-lang.org/) and [PostgreSQL](https://www.postgresql.org/)

## Usage

1 - Create a schema and table in PostGreSQL with the SQL below:

```
CREATE SCHEMA IF NOT EXISTS mydb;

CREATE TABLE IF NOT EXISTS mydb.users
(
	id serial,
	datetime TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	name char(50) not null,
	email char(50) not null,
	age integer not null,
	driver_license boolean,
	PRIMARY KEY (id)
);
```

2 - Create a `.env` file in this directory with the following variables:

```
DB_URL=
SCHEMA=
```

Example url: ```postgres://<USERNAME>:<PASSWORD>@<HOST>:<PORT>/<DATABASE>```

### Commands

3 - Run the tests:

```
make tests
````

4 - For more command, run:

```
make
````

---
[LICENSE](LICENSE)
