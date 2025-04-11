-- Table for storing relations between vegetables

CREATE TABLE compatibility_relations (
    id SERIAL PRIMARY KEY,
    vegetable_id_1 INT NOT NULL,
    vegetable_id_2 INT NOT NULL,
    compatibility INT CHECK (compatibility IN (1, 2, 0)) NOT NULL,
    explanation TEXT NOT NULL,
    CONSTRAINT fk_vegetable_1 FOREIGN KEY (vegetable_id_1) REFERENCES vegetables(id) ON DELETE CASCADE,
    CONSTRAINT fk_vegetable_2 FOREIGN KEY (vegetable_id_2) REFERENCES vegetables(id) ON DELETE CASCADE,
    CONSTRAINT unique_vegetable_pair UNIQUE (vegetable_id_1, vegetable_id_2)
);

CREATE INDEX idx_vegetable_pair ON compatibility_relations (LEAST(vegetable_id_1, vegetable_id_2), GREATEST(vegetable_id_1, vegetable_id_2));