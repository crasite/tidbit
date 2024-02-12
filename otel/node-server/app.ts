import express, { Express } from "express";
import { startSdk, extractContext } from "./tracing";
import { trace } from "@opentelemetry/api";

startSdk();
const tracer = trace.getTracer("dice-server", "0.1.0");
const PORT: number = parseInt(process.env.PORT || "8080");
const app: Express = express();

function getRandomNumber(min: number, max: number) {
  return Math.floor(Math.random() * (max - min) + min);
}

app.get("/rolldice", async (req, res) => {
  const span = tracer.startSpan("rolling dice", undefined, extractContext(req));
  let randNum = getRandomNumber(1, 6);
  span.setAttribute("dice-rolled", randNum);
  await new Promise((r) => setTimeout(r, randNum * 100));
  res.send(randNum.toString());
  span.end();
});

app.listen(PORT, () => {
  console.log(`Listening for requests on http://localhost:${PORT}`);
});
