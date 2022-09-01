CREATE TABLE "todo-items" (
    "id" SERIAL PRIMARY KEY,
    "title" VARCHAR(256) NOT NULL,
    "content" TEXT,
    "created" TIMESTAMP NOT NULL,
    "done" BOOLEAN NOT NULL DEFAULT FALSE,

    UNIQUE(title)
);