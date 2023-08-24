create table "bind_code"
(
    "id" SERIAL PRIMARY KEY,
    "code" CHAR(6) NOT NULL,
    "email" VARCHAR(100) NOT NULL,
    "status" SMALLINT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ
);
