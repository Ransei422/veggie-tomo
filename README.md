# Veggie Tomo

## Server installation

To run do the steps bellow:
1. cd veggie-tomo
2. sudo chmod +x run.sh
3. ./run.sh
------------------------------------------------


## Server initialization

To check everythings working acess:
http://<YOUR_SERVER_IP>:3000/

To create a admin account access:
http://localhost:4000/register  (Available only from server's local network)

→ Or POST form if you can't use GUI

```
curl -X POST http://localhost:4000/register \
  -d "email=<YOUR_EMAIL>" \
  -d "password=<YOUR_PASSWORD>" \
  -d "confirm_password=<YOUR_PASSWORD>"
```
------------------------------------------------


## API calls

```
<!-- Get JWT Token -->
curl -X POST http://localhost:3000/signin \
    -H "Content-Type: application/json" \
    -d '{"email": "<YOUR_REGISTERED_EMAIL>", "password": "<YOUR_PASSWORD>"}'



<!-- Register vegetable type -->
curl -X POST http://localhost:3000/register_vegetable \
    -H "Authorization: Bearer <JWT_TOKEN_YOU_GOT>" \
    -H "Content-Type: application/json" \
    -d '{"name": "はす"}'
```