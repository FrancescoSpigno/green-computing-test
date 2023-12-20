const express = require('express')
const app = express()

app.use('/', express.static("file-test"))

app.listen(8000)

console.log("Listening on http://localhost:8000")