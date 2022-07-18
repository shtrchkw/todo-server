-- Your SQL goes here
CREATE TABLE todo_status (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

INSERT INTO todo_status VALUES (1, 'Proposed');
INSERT INTO todo_status VALUES (2, 'Started');
INSERT INTO todo_status VALUES (3, 'Done');

CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    todo_status_id INTEGER NOT NULL REFERENCES todo_status (id),
    user_id INTEGER NOT NULL REFERENCES users (id)
);