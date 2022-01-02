-- Add down migration script here
ALTER TABLE posts
DROP CONSTRAINT posts_user_id_fkey,
ADD CONSTRAINT posts_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE tokens
DROP CONSTRAINT tokens_user_id_fkey,
ADD CONSTRAINT tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);