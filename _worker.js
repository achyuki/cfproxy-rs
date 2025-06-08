/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

import { connect } from "cloudflare:sockets";

let token = "";
const page = "Nya!";

export default {
  async fetch(request, env, _ctx) {
    const { TOKEN } = env;
    token = TOKEN || token;

    const upgrade = request.headers.get("Upgrade");
    const auth = request.headers.get("Token");
    const hostname = request.headers.get("Hostname");
    const port = parseInt(request.headers.get("Port"));

    if (upgrade !== "websocket")
      return new Response(page, {
        headers: {
          "content-type": "text/html;charset=UTF-8",
        },
      });
    if (auth !== token) return new Response(null, { status: 401 });

    const [client, server] = Object.values(new WebSocketPair());
    server.accept();
    try {
      const socket = connect({ hostname, port });
      new ReadableStream({
        start(controller) {
          server.onmessage = ({ data }) => controller.enqueue(data);
          server.onclose = () => controller.close();
          server.onerror = (e) => controller.error(e);
        },
        cancel(reason) {
          server.close();
        },
      }).pipeTo(socket.writable);
      socket.readable.pipeTo(
        new WritableStream({
          start(controller) {
            server.onerror = (e) => controller.error(e);
          },
          write(chunk) {
            server.send(chunk);
          },
        })
      );
    } catch (error) {
      server.close();
    }

    return new Response(null, { status: 101, webSocket: client });
  },
};
