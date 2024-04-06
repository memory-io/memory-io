import { handler } from './build/handler.js';
import express from 'express';
import proxy from 'express-http-proxy';
const app = express();
const port = 3000;
const backend = 8000;

app.use('/api', proxy(`http://server:${backend}`, {
  proxyReqPathResolver: function (req) {
    return '/api' + req.url;
  }
}));

app.use(handler);

app.listen(port, () => {
  console.log(`Memory Web listening on port ${port}`);
  console.log(`Reverse proxy forwarding to port ${backend}`);
});