// Automatically generated by flapigen
package com.example.rust;


public final class TestPassObjectsAsParams {

    public TestPassObjectsAsParams() {
        mNativeObj = init();
    }
    private static native long init();

    public TestPassObjectsAsParams(Foo a0) {
        long a00 = a0.mNativeObj;
        a0.mNativeObj = 0;

        mNativeObj = init(a00);
        JNIReachabilityFence.reachabilityFence1(a0);
    }
    private static native long init(long a0);

    public final void f1(Foo a0) {
        long a00 = a0.mNativeObj;
        do_f1(mNativeObj, a00);

        JNIReachabilityFence.reachabilityFence1(a0);
    }
    private static native void do_f1(long self, long a0);

    public final void f2(Foo a0) {
        long a00 = a0.mNativeObj;
        do_f2(mNativeObj, a00);

        JNIReachabilityFence.reachabilityFence1(a0);
    }
    private static native void do_f2(long self, long a0);

    public final void f3(Foo a0) {
        long a00 = a0.mNativeObj;
        a0.mNativeObj = 0;

        do_f3(mNativeObj, a00);

        JNIReachabilityFence.reachabilityFence1(a0);
    }
    private static native void do_f3(long self, long a0);

    public final void f_get_like_me(TestPassObjectsAsParams a0) {
        long a00 = a0.mNativeObj;
        a0.mNativeObj = 0;

        do_f_get_like_me(mNativeObj, a00);

        JNIReachabilityFence.reachabilityFence1(a0);
    }
    private static native void do_f_get_like_me(long self, long a0);

    public final int get_data() {
        int ret = do_get_data(mNativeObj);

        return ret;
    }
    private static native int do_get_data(long self);

    public final String get_name() {
        String ret = do_get_name(mNativeObj);

        return ret;
    }
    private static native String do_get_name(long self);

    public static String f4(Foo a0) {
        long a00 = a0.mNativeObj;
        String ret = do_f4(a00);

        JNIReachabilityFence.reachabilityFence1(a0);

        return ret;
    }
    private static native String do_f4(long a0);

    public static String f5(int a0, String a1, Foo a2) {
        long a20 = a2.mNativeObj;
        String ret = do_f5(a0, a1, a20);

        JNIReachabilityFence.reachabilityFence1(a2);

        return ret;
    }
    private static native String do_f5(int a0, String a1, long a2);

    public static String f6(Foo a0) {
        long a00 = a0.mNativeObj;
        a0.mNativeObj = 0;

        String ret = do_f6(a00);

        JNIReachabilityFence.reachabilityFence1(a0);

        return ret;
    }
    private static native String do_f6(long a0);

    public static TestPassObjectsAsParams factory_method(int a0, String a1) {
        long ret = do_factory_method(a0, a1);
        TestPassObjectsAsParams convRet = new TestPassObjectsAsParams(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_factory_method(int a0, String a1);

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
    /*package*/ TestPassObjectsAsParams(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}