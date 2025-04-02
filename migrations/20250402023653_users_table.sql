CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (email, password_hash) 
VALUES 
    ('admin@example.com', '$2b$12$kxy4H0mvJNk2spK5.0QLoOraHnFuJ52IcRZ/EwOMe3Xq5v.0VuWou') -- hashed mysecret
ON CONFLICT (email) DO NOTHING;



-- curl -X POST http://localhost:3000/hashme -H "Content-Type: application/json"   -d '{"password": "mysecret"}'
-- curl -X POST http://localhost:3000/signin -H "Content-Type: application/json" -d '{"email": "admin@example.com", "password": "mysecret"}'