CREATE TABLE messages (
    id VARCHAR PRIMARY KEY,
    author VARCHAR REFERENCES users(id),
    content VARCHAR NOT NULL
)
