package com.mxmgorin.gmboy;

import android.app.Activity;
import android.content.ContentResolver;
import android.content.Intent;
import android.database.Cursor;
import android.net.Uri;
import android.os.Bundle;
import android.provider.OpenableColumns;
import android.view.View;

import androidx.annotation.Nullable;

import org.libsdl.app.SDLActivity;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;

public class MainActivity extends SDLActivity {
    private static final int FILE_PICKER_REQUEST = 1001;
    private static final int OPEN_DIRECTORY_REQUEST = 42;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        enableImmersiveMode();
        nativeInit(); // <-- this initializes the JVM for Rust
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            enableImmersiveMode();
        }
    }

    // Called from Rust via JNI
    public void openFilePicker() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addFlags(Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION);
        intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        intent.setType("*/*"); // You can set e.g. "application/json"
        startActivityForResult(intent, FILE_PICKER_REQUEST);
    }

    // Called from Rust via JNI
    public void openDirectoryPicker() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
        intent.addFlags(Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION);
        intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION);
        intent.addFlags(Intent.FLAG_GRANT_WRITE_URI_PERMISSION);
        startActivityForResult(intent, OPEN_DIRECTORY_REQUEST);
    }

    public static List<String> getFilesInDirectory(String uriStr, String[] extensions) {
        List<String> files = new ArrayList<>();

        try {
            Uri treeUri = Uri.parse(uriStr);
            ContentResolver cr = context.getContentResolver();
            Uri childrenUri = DocumentsContract.buildChildDocumentsUriUsingTree(
                    treeUri, DocumentsContract.getTreeDocumentId(treeUri));

            Cursor cursor = cr.query(childrenUri,
                    new String[]{
                            DocumentsContract.Document.COLUMN_DOCUMENT_ID,
                            DocumentsContract.Document.COLUMN_DISPLAY_NAME
                    },
                    null, null, null);

            if (cursor != null) {
                while (cursor.moveToNext()) {
                    String docId = cursor.getString(0);
                    String displayName = cursor.getString(1);

                    // Check extension
                    if (matchesExtension(displayName, extensions)) {
                        Uri fileUri = DocumentsContract.buildDocumentUriUsingTree(treeUri, docId);
                        files.add(fileUri.toString());
                    }
                }
                cursor.close();
            }
        } catch (Exception e) {
            files.add("error:" + e.toString());
        }

        return files;
    }

    private static boolean matchesExtension(String name, String[] extensions) {
        for (String ext : extensions) {
            if (name.toLowerCase().endsWith(ext.toLowerCase())) {
                return true;
            }
        }
        return false;
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, @Nullable Intent data) {
        super.onActivityResult(requestCode, resultCode, data);

        if (resultCode != Activity.RESULT_OK || data == null) return;

        Uri uri = data.getData();

        if (uri == null) return;

        final int takeFlags = data.getFlags()
                & (Intent.FLAG_GRANT_READ_URI_PERMISSION | Intent.FLAG_GRANT_WRITE_URI_PERMISSION);
        getContentResolver().takePersistableUriPermission(uri, takeFlags);

        String uriStr = uri.toString();

        if (requestCode == FILE_PICKER_REQUEST) {
            // Pass the result back to Rust
            nativeOnFilePicked(uriStr);
        }

        if (requestCode == OPEN_DIRECTORY_REQUEST) {
            // Pass the result back to Rust
            nativeOnDirectoryPicked(uriStr);
        }
    }

    // Declare native callback implemented in Rust
    private static native void nativeOnFilePicked(String uri);

    // Declare native callback implemented in Rust
    private static native void nativeOnDirectoryPicked(String uri);

    private void enableImmersiveMode() {
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
