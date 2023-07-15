CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS "comments" (
        "id" UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        "id_post_comment" UUID NOT NULL,
        "user_name_comment" varchar(200) NOT NULL,
        "comment" varchar(200) NOT NULL,
        "created_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            "updated_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            FOREIGN KEY (id_post_comment) REFERENCES posts(id) ON UPDATE CASCADE ON DELETE CASCADE
    );