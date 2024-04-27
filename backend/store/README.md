To run database migrations in dev mode:

```shell
cd store
rm -rf alerts.sqlite
export DATABASE_URL="sqlite:alerts.sqlite"
sqlx database create
sqlx migrate run
```