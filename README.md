verifying email binder
======================

## Run

```
export DATABASE_URL=postgres://aa_email:email@localhost:5432/aa_email
```

## API

```bash
curl -X POST http://localhost:3000 -H "Content-Type:application/json" --data '{
    "jsonrpc":"2.0",
                "method":"send_code",
                "params": ["test@test.com"],
    "id":1
}'
```