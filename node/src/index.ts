import { eq, sql } from "drizzle-orm";
import { drizzle } from "drizzle-orm/node-postgres";
import { boolean, pgTable, serial, text } from "drizzle-orm/pg-core";
import { migrate } from "drizzle-orm/postgres-js/migrator";
import express from "express";
import { Pool } from "pg";

const app = express();
const port = 8080;
const pool = new Pool({
  connectionString: "postgres://postgres:changeme@localhost:5432/mydb",
});
const db = drizzle(pool);

export const todos = pgTable("todo", {
  id: serial("id").primaryKey(),
  description: text("description").notNull(),
  completed: boolean("completed").notNull().default(false),
});

type Todo = typeof todos.$inferInsert;

app.use(express.json());

app.get("/get-items", async (req, res) => {
  const todoList = await db.select().from(todos);
  return res.json(todoList);
});

app.post("/create-item", async (req, res) => {
  const todo: Todo = { description: "use drizzle" };

  const result = await db.insert(todos).values(todo).returning();
  return res.json(result);
});

app.post("/toggle-item/:id", async (req, res) => {
  const item = await db
    .update(todos)
    .set({
      completed: sql`NOT ${todos.completed}`,
    })
    .where(eq(todos.id, Number(req.params.id)))
    .returning();

  res.json(item);
});

migrate(db, { migrationsFolder: "./drizzle" }).then(() => {
  app.listen(port, () => {
    console.log(`Example app listening on port ${port}`);
  });
});
