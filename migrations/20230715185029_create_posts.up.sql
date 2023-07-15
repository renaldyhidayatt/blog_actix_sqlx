CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS "posts" (
        "id" UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        "title" varchar(200) NOT NULL,
        "body" text NOT NULL,
        "category_id" UUID NOT NULL,
        "user_id" UUID NOT NULL,
        "user_name" varchar(200) NOT NULL,
        "created_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            "updated_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            FOREIGN KEY (category_id) REFERENCES categories(id) ON UPDATE CASCADE ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE
    );