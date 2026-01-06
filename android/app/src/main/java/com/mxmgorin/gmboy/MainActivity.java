package com.mxmgorin.gmboy;

import android.app.Activity;
import android.content.ContentResolver;
import android.content.Intent;
import android.database.Cursor;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.provider.DocumentsContract;
import android.provider.OpenableColumns;
import android.view.View;

import androidx.annotation.Nullable;
import androidx.documentfile.provider.DocumentFile;

import org.libsdl.app.SDLActivity;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;

public class MainActivity extends SDLActivity {
    private static final int FILE_PICKER_REQUEST = 1001;
    private static final int DIRECTORY_PICKER_REQUEST = 42;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        hideSystemBars();
        nativeInit(); // initialize the JVM for Rust
    }

    @Override
    protected void onResume() {
        super.onResume();
        hideSystemBars();
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            hideSystemBars();
        }
    }

    // Called from Rust via JNI
    public void openFilePicker() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        intent.addFlags(
                Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION |
                        Intent.FLAG_GRANT_READ_URI_PERMISSION
        );
        intent.setType("*/*");
        startActivityForResult(intent, FILE_PICKER_REQUEST);
    }

    // Called from Rust via JNI
    public void openDirectoryPicker() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
        intent.addFlags(
                Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION |
                        Intent.FLAG_GRANT_READ_URI_PERMISSION
        );
        startActivityForResult(intent, DIRECTORY_PICKER_REQUEST);
    }

    public static List<String> getFilesInDirectory(String uriStr) {
        List<String> files = new ArrayList<>();
        Uri treeUri = Uri.parse(uriStr);

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            if (!DocumentsContract.isTreeUri(treeUri)) {
                return files;
            }
        }

        DocumentFile dir = DocumentFile.fromTreeUri(getContext(), treeUri);

        if (dir == null || !dir.isDirectory()) {
            return files;
        }

        for (DocumentFile file : dir.listFiles()) {
            if (!file.isFile()) continue;

            // Optional: filter extensions
            String name = file.getName();
            if (name == null) continue;

            files.add(file.getUri().toString());
        }

        return files;
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, @Nullable Intent data) {
        super.onActivityResult(requestCode, resultCode, data);

        if (resultCode != Activity.RESULT_OK || data == null) {
            handlePickedResult(requestCode, null);
            return;
        }

        Uri uri = data.getData();
        if (uri == null) {
            handlePickedResult(requestCode, null);
            return;
        }

        final int takeFlags =
                data.getFlags() & Intent.FLAG_GRANT_READ_URI_PERMISSION;

        if (takeFlags != 0) {
            try {
                getContentResolver().takePersistableUriPermission(uri, takeFlags);
            } catch (SecurityException ignored) {
                // Some providers don't support persistence; that's OK
            }
        }

        handlePickedResult(requestCode, uri.toString());
    }

    // Pass the result back to Rust
    private void handlePickedResult(int requestCode, @Nullable String uriStr) {
        if (requestCode == FILE_PICKER_REQUEST) {
            nativeOnFilePicked(uriStr);
        }

        if (requestCode == DIRECTORY_PICKER_REQUEST) {
            nativeOnDirectoryPicked(uriStr);
        }
    }

    // Declare native callback implemented in Rust
    private static native void nativeOnFilePicked(@Nullable String uri);

    // Declare native callback implemented in Rust
    private static native void nativeOnDirectoryPicked(@Nullable String uri);

    private void hideSystemBars() {
        getWindow().getDecorView().setSystemUiVisibility(
                View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                        | View.SYSTEM_UI_FLAG_FULLSCREEN
                        | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                        | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                        | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                        | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
        );
    }

    public static byte[] readUriBytes(String uriStr) {
        try {
            Uri uri = Uri.parse(uriStr);
            ContentResolver resolver = getContext().getContentResolver();
            InputStream inputStream = resolver.openInputStream(uri);

            if (inputStream == null) return null;

            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            byte[] data = new byte[4096];
            int nRead;
            while ((nRead = inputStream.read(data, 0, data.length)) != -1) {
                buffer.write(data, 0, nRead);
            }
            inputStream.close();
            return buffer.toByteArray();
        } catch (IOException e) {
            e.printStackTrace();
            return null;
        }
    }

    public static String getFileName(String uriStr) {
        try {
            Uri uri = Uri.parse(uriStr);
            ContentResolver resolver = getContext().getContentResolver();
            Cursor cursor = resolver.query(uri, null, null, null, null);
            if (cursor != null) {
                try {
                    int nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME);
                    if (nameIndex != -1 && cursor.moveToFirst()) {
                        return cursor.getString(nameIndex);
                    }
                } finally {
                    cursor.close();
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return null; // fallback if filename can't be found
    }

    public static MainActivity getContext() {
        return (MainActivity) SDLActivity.getContext();
    }

    // Called from Rust to store the JVM reference
    public static native void nativeInit();
}
