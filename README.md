# 🥬 Veggie Tomo (Back-end)

## 1. Server installation

To run first time do the steps bellow:
1. cd veggie-tomo
2. sudo chmod +x install.sh
3. ./install.sh
------------------------------------------------

After that you can run server without root:
* Server will run with "INFO" level logging (change inside `run.sh` to `error`)
1. ./run.sh
------------------------------------------------


## 2. Server initialization

To check everythings working acess:
> http://<YOUR_SERVER_IP>:3000

get `YOUR_SERVER_IP` by executing `ip a | grep 192` on the server's terminal (Linux only).


To create an admin account access:
> http://localhost:4000/register  (Available only from server's local network)


→ Or POST form if you can't use GUI

```
curl -X POST http://localhost:4000/register \
  -d "email=<YOUR_EMAIL>" \
  -d "password=<YOUR_PASSWORD>" \
  -d "confirm_password=<YOUR_PASSWORD>"
```
------------------------------------------------


## 3. API calls

This will be used from front-end and phone-app.

```
<!-- Get JWT Token -->
curl -X POST http://localhost:3000/signin \
    -H "Content-Type: application/json" \
    -d '{"email": "<YOUR_REGISTERED_EMAIL>", "password": "<YOUR_PASSWORD>"}'



<!-- Check what vegetable is register-able from specific family -->
curl -X POST http://localhost:3000/api/check_available_vegetable \
    -H "Authorization: Bearer <JWT_TOKEN_YOU_GOT>" \
    -H "Content-Type: application/json" \
    -d '{"family_name": "アオイ"}'



<!-- Register vegetable type -->
curl -X POST http://localhost:3000/api/register_vegetable \
    -H "Authorization: Bearer <JWT_TOKEN_YOU_GOT>" \
    -H "Content-Type: application/json" \
    -d '{"name": "オクラ"}'



<!-- Check if vegetable type already registered -->
curl -X POST http://localhost:3000/api/check_registered_vegetable \
    -H "Authorization: Bearer <JWT_TOKEN_YOU_GOT>" \
    -H "Content-Type: application/json" \
    -d '{"name": "オクラ"}'



<!-- Register relationship between existing vegetables -->
<!-- compatibility: 0-> no-effect / 1-> good/ 2-> bad -->
curl -X POST http://localhost:3000/api/register_vegetable_relationship \
    -H "Authorization: Bearer <JWT_TOKEN_YOU_GOT>" \
    -H "Content-Type: application/json" \
    -d '{"vegetable_1_name": "セリ",
        "vegetable_2_name": "オクラ",
        "compatibility": 0,
        "explanation": "<SOME EXPLANATION>"
        }'
```