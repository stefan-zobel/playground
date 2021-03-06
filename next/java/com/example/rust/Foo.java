// Automatically generated by flapigen
package com.example.rust;

/**
 * Class comment description for Foo.
 */
public final class Foo {
    /**
     * some text about the new function
     * 
     * ```
     * some markdown example in the text
     * ```
     * 
     * @param val - some number
     * @param name - more information
     */
    public Foo(int val, String name) {
        mNativeObj = init(val, name);
    }
    private static native long init(int val, String name);

    public final int calcF(int a0, int a1) {
        int ret = do_calcF(mNativeObj, a0, a1);

        return ret;
    }
    private static native int do_calcF(long self, int a0, int a1);

    public final double f_double(double a0, double a1) {
        double ret = do_f_double(mNativeObj, a0, a1);

        return ret;
    }
    private static native double do_f_double(long self, double a0, double a1);

    public final String getName() {
        String ret = do_getName(mNativeObj);

        return ret;
    }
    private static native String do_getName(long self);

    public static native double fHypot(double a, double b);

    public static java.util.Date now() {
        long ret = do_now();
        java.util.Date convRet = new java.util.Date(ret);

        return convRet;
    }
    private static native long do_now();

    public static java.util.Date chrono_now() {
        long ret = do_chrono_now();
        java.util.Date convRet = new java.util.Date(ret);

        return convRet;
    }
    private static native long do_chrono_now();

    public static java.util.Optional<java.util.Date> chrono_now_opt(boolean flag) {
        java.util.OptionalLong ret = do_chrono_now_opt(flag);
        java.util.Optional<java.util.Date> convRet;
        if (ret.isPresent()) {
            convRet = java.util.Optional.of(new java.util.Date(ret.getAsLong()));
        } else {
            convRet = java.util.Optional.empty();
        }

        return convRet;
    }
    private static native java.util.OptionalLong do_chrono_now_opt(boolean flag);

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ Foo(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}