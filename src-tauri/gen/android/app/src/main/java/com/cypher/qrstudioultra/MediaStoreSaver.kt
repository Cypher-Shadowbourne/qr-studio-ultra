package com.cypher.qrstudioultra

import android.app.Activity
import android.content.ContentValues
import android.content.Intent
import android.os.Build
import android.os.Environment
import android.provider.MediaStore
import androidx.annotation.Keep
import androidx.core.content.ContextCompat.startActivity
import java.io.IOException

@Keep
object MediaStoreSaver {
    private var lastSavedUri: android.net.Uri? = null

    @Keep
    fun saveQrImage(activity: Activity, bytes: ByteArray, filename: String, mimeType: String): String {
        val resolver = activity.contentResolver
        val collection = MediaStore.Images.Media.getContentUri(MediaStore.VOLUME_EXTERNAL_PRIMARY)
        val relativePath = "${Environment.DIRECTORY_PICTURES}/QR Studio Ultra"

        val values = ContentValues().apply {
            put(MediaStore.MediaColumns.DISPLAY_NAME, filename)
            put(MediaStore.MediaColumns.MIME_TYPE, mimeType)
            put(MediaStore.MediaColumns.RELATIVE_PATH, relativePath)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                put(MediaStore.MediaColumns.IS_PENDING, 1)
            }
        }

        val uri = resolver.insert(collection, values)
            ?: throw IOException("Android could not create a gallery entry for $filename")

        try {
            resolver.openOutputStream(uri)?.use { output ->
                output.write(bytes)
                output.flush()
            } ?: throw IOException("Android could not open the gallery output stream")

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                val publishValues = ContentValues().apply {
                    put(MediaStore.MediaColumns.IS_PENDING, 0)
                }
                resolver.update(uri, publishValues, null, null)
            }

            lastSavedUri = uri
            return "Saved to Gallery/Photos in Pictures/QR Studio Ultra."
        } catch (e: Exception) {
            resolver.delete(uri, null, null)
            throw e
        }
    }

    @Keep
    fun openLastSavedImage(activity: Activity): String {
        val uri = lastSavedUri ?: throw IOException("No saved image is available yet.")
        val intent = Intent(Intent.ACTION_VIEW).apply {
            setDataAndType(uri, "image/*")
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        activity.startActivity(Intent.createChooser(intent, "Open QR image"))
        return "Opened saved image."
    }

    @Keep
    fun shareLastSavedImage(activity: Activity): String {
        val uri = lastSavedUri ?: throw IOException("No saved image is available yet.")
        val intent = Intent(Intent.ACTION_SEND).apply {
            type = "image/*"
            putExtra(Intent.EXTRA_STREAM, uri)
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        }
        activity.startActivity(Intent.createChooser(intent, "Share QR image"))
        return "Opened share sheet."
    }
}
