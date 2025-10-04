-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT uuidv7(),
	"email" VARCHAR NOT NULL,
	"username" VARCHAR NOT NULL,
	"password" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"is_active" BOOL NOT NULL DEFAULT TRUE
);

-- Trigger to update the updated_at column on row update
CREATE OR REPLACE FUNCTION users_update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';

-- Add the trigger to the users table
CREATE TRIGGER users_update_updated_at
BEFORE UPDATE ON "users"
FOR EACH ROW
EXECUTE PROCEDURE users_update_updated_at_column();