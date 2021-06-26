# Container

Run container locally:

```shell
docker build -t foruma:local .
docker run --rm -p 18080:18080 -e app_cors__origins=http://localhost:8080 -e app_geo_ip__path=geoip -e app_http_server__host=0.0.0.0 -e app_http_server__port=18080 -e app_postgres__database_name=foruma -e app_postgres__host=host.docker.internal -e app_postgres__password=password -e app_postgres__port=5432 -e app_postgres__require_ssl=false -e app_postgres__username=postgres -e app_postgres__migration__create_database=true -e app_postgres__migration__path=./migrations foruma:local
```
