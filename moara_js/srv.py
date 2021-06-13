#Use to create local host
import http.server
import socketserver

PORT = 7800

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map.update({
      ".js": "application/javascript",
      ".wasm": "application/wasm"
})

httpd = socketserver.TCPServer(("", PORT), Handler)
httpd.serve_forever()