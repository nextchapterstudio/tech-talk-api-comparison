import express from "express";
import { Pool } from "pg";
import { drizzle } from "drizzle-orm/node-postgres";
import {
  integer,
  pgTable,
  serial,
  text,
  timestamp,
  varchar,
  boolean,
} from "drizzle-orm/pg-core";
import { eq } from "drizzle-orm";
import { migrate } from "drizzle-orm/postgres-js/migrator";

const app = express();
const port = 3001;
const pool = new Pool({
  connectionString:
    "postgres://splash_app_user:postgresPW@localhost:5455/postgresDB",
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
  const foundTodos = await db
    .select({
      id: todos.id,
      completed: todos.completed,
    })
    .from(todos)
    .where(eq(todos.id, Number(req.params.id)));

  if (foundTodos.length == 0) {
    return res.status(404);
  }

  const item = await db
    .update(todos)
    .set({
      completed: !foundTodos[0].completed,
    })
    .returning();

  res.json(item);
});

migrate(db, { migrationsFolder: "./drizzle" }).then(() => {
  app.listen(port, () => {
    console.log(`Example app listening on port ${port}`);
  });
});
