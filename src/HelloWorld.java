public class HelloWorld {
    private static native String hello(String input);

    static {
        System.loadLibrary("jniffi");
    }

    public static void main(String[] args) {
        System.out.println(HelloWorld.hello("shane"));
    }
}
