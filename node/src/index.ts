import express from 'express';
const app = express()
const port = 3001

type todoItem = {
    id: number,
    description: string,
    completed: boolean,
}
const todoList : todoItem[] = []

let counter = 0;

app.use(express.json())

app.get('/get-items', (req, res) => {
  res.json(todoList)
})

app.post('/create-item', (req, res)=>{
    const item = {
        id: counter++,
        description : req.body.description,
        completed: false,
    }

    todoList.push(item)
    res.json(item)
})

app.post('/toggle-item/:id', (req, res) =>{
    
    const item = todoList.find((item)=>{
       return Number(req.params.id) == item.id
    })
    if (item !== undefined)
    {
        item.completed = !item.completed
    } 

    res.json(item)
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})