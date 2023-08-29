verifying email binder
======================

## Build

```
docker build -t verifying-email-binder .
```

## Run

```
export DATABASE_URL=postgres://aa_email:email@localhost:5432/aa_email
export RPC_URL=https://babel-api.testnet.iotex.io
export GUARDIAN_ADDRESS=0xd8F31BC1E49d800D8B6B2AAE27219f47b94F5890
export SIGNER={SIGNER_PRIVATE_KEY}
export SMTP_PASSWORD=
export SMTP_HOST=smtp.larksuite.com
export SMTP_USER=iopay-recover@iotex.me
```

## API

```bash
curl -X POST https://email-binder.testnet.iotex.io/ -H "Content-Type:application/json" --data '{
    "jsonrpc":"2.0",
                "method":"send_code",
                "params": ["0x8803DAF0AB9Bad65a56F4D9AEcA56085491C299A", "test@test.com"],
    "id":1
}'

curl -X POST http://localhost:3000 -H "Content-Type:application/json" --data '{
    "jsonrpc":"2.0",
                "method":"verify_code",
                "params": ["0x8803DAF0AB9Bad65a56F4D9AEcA56085491C299A", "test@test.com", "123456"],
    "id":1
}'
```