CREATE TABLE todo_items (
    "id" SERIAL PRIMARY KEY,
    "title" VARCHAR(256) NOT NULL,
    "content" TEXT,
    "done" BOOLEAN DEFAULT FALSE,
    "created" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(title)
);