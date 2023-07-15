CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS "categories" (
        "id" UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        "name" varchar(200) NOT NULL,
        "created_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            "updated_at" TIMESTAMP
        WITH TIME ZONE DEFAULT NOW()
    );