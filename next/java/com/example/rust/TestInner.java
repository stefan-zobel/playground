// Automatically generated by flapigen
package com.example.rust;


public final class TestInner {

    private TestInner() {}

    public static final class Inner {
        public final String name;

        Inner(String name) {
            this.name = name;
        }
    }
    public static native Inner getInner();
}