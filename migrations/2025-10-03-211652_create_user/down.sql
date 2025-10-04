-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "users";

DROP TRIGGER IF EXISTS users_update_user_updated_at ON "users";
DROP FUNCTION IF EXISTS users_update_updated_at_column();