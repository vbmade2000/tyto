# tyto
A URL shortner written in Rust

# Why name tyto?
**Tyto** (Tyto aurantia) is a name of endangered specie of owl commonly known as Golden Masked Owl. To spread awareness about such endangered species the name is kept. You can find more information at https://en.wikipedia.org/wiki/Golden_masked_owl.

# Compile
```
$ export DATABASE_URL="postgres://tyto@localhost/tyto
$ cargo build
```

# Run tyto locally
### Run Postgresql database container for testing
```sudo docker run --rm -p 5432:5432 -e POSTGRES_USER=tyto -e POSTGRES_HOST_AUTH_METHOD=trust  -v /tmp/postgres_docker_volume:/var/lib/postgresql/data postgres```
### Run database migrations
tyto uses [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) for database migrations.
After installing **sql-cli** from above link, execute following command in project directory.
```$ sqlx migrate run```
