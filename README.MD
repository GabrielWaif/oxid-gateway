# Oxid Gateway

## Setup

**postgres**:
```sh
docker run --name oxid-gateway-postgres -e POSTGRES_PASSWORD=admin -e POSTGRES_DB=oxid-gateway -p 5432:5432 -d postgres
```
