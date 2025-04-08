# Veggie Tomo

To run do the steps bellow:
1. cd veggie-tomo
2. sudo chmod +x run.sh
3. ./run.sh


To check everythings working acess:
http://<YOUR_SERVER_IP>:3000/

To create a admin account access:
http://localhost:4000/register  (Available only from server's local network)



API calls

```
<!-- Get JWT Token -->
curl -X POST http://localhost:3000/signin \
    -H "Content-Type: application/json" \
    -d '{"email": "test@test.test", "password": "1234567890"}'



<!-- Register vegetable type -->
curl -X POST http://localhost:3000/register_vegetable \
    -H "Authorization: Bearer <JWT TOKEN>" \
    -H "Content-Type: application/json" \
    -d '{"name": "はす"}'
```