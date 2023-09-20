-- Add migration script here
CREATE TABLE IF NOT EXISTS "todo" (
	"id" serial PRIMARY KEY NOT NULL,
	"description" text NOT NULL,
	"completed" boolean DEFAULT false NOT NULL
);