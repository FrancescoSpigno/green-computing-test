import {Elysia} from "elysia";

const app = new Elysia().get("/small", async () => {
        return Bun.file("file-test/small.json")
}).get("/medium", async () => {
    return Bun.file("file-test/medium.json")
}).get("/large", async () => {
    return Bun.file("file-test/large.json")
}).listen(3000);

console.log("Listening on http://localhost:8000");