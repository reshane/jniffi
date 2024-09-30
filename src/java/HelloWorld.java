import java.io.IOException;
import java.io.OutputStream;
import java.io.InputStream;
import java.net.InetSocketAddress;

import com.sun.net.httpserver.HttpServer;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;

public class HelloWorld {
    private static native String hello(String input);
    private static native byte[] helloBytes(byte[] input);

    static {
        System.loadLibrary("jniffi");
    }

    public static void main(String[] args) throws IOException {
        System.out.println(HelloWorld.hello("shane"));
        HttpServer server = HttpServer.create(new InetSocketAddress(6969), 0);

        server.createContext("/test", new MyHandler());
        server.setExecutor(null);
        server.start();
    }

    static class MyHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange t) throws IOException {
            if (t.getRequestMethod().equals("POST")) {
                InputStream is = t.getRequestBody();

                byte[] bytes = is.readAllBytes();
                is.close();

                byte[] resp = HelloWorld.helloBytes(bytes);

                t.sendResponseHeaders(200, resp.length);
                OutputStream os = t.getResponseBody();
                os.write(resp);
                os.close();
                return;
            }
            String resp = HelloWorld.hello("shane");
            t.sendResponseHeaders(200, resp.length());
            OutputStream os = t.getResponseBody();
            os.write(resp.getBytes());
            os.close();
        }
    }
}
