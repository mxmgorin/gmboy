package com.mxmgorin.gmboy;

import android.app.Activity;
import android.os.Bundle;

public class Main extends Activity {
    static {
        System.loadLibrary("SDL2");
        System.loadLibrary("gmboy");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        nativeMain();
    }

    public native void nativeMain();
}
