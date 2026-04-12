package com.cypher.qrstudioultra

import android.app.Activity
import android.content.ContentValues
import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.Rect
import android.graphics.Typeface
import android.os.Build
import android.os.Environment
import android.provider.MediaStore
import androidx.annotation.Keep
import androidx.core.content.ContextCompat.startActivity
import androidx.print.PrintHelper
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference
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

    @Keep
    fun printQrImage(activity: Activity, bytes: ByteArray, title: String, mimeType: String): String {
        val bitmap = BitmapFactory.decodeByteArray(bytes, 0, bytes.size)
            ?: throw IOException("Android could not decode the QR image for printing.")

        if (!mimeType.startsWith("image/")) {
            throw IOException("Android printing only supports image data.")
        }

        val printableBitmap = createPrintSheet(bitmap, title)

        val result = AtomicReference<Result<String>>()
        val latch = CountDownLatch(1)

        activity.runOnUiThread {
            try {
                val printHelper = PrintHelper(activity).apply {
                    scaleMode = PrintHelper.SCALE_MODE_FIT
                    colorMode = PrintHelper.COLOR_MODE_COLOR
                    orientation = PrintHelper.ORIENTATION_PORTRAIT
                }
                printHelper.printBitmap(title.ifBlank { "QR Code" }, printableBitmap)
                result.set(Result.success("Opened Android print dialog."))
            } catch (e: Exception) {
                result.set(Result.failure(e))
            } finally {
                latch.countDown()
            }
        }

        if (!latch.await(10, TimeUnit.SECONDS)) {
            throw IOException("Android print dialog timed out before opening.")
        }

        return result.get().getOrElse { throw IOException("Android printing failed: ${it.message}", it) }
    }

    private fun createPrintSheet(qrBitmap: Bitmap, title: String): Bitmap {
        val pageWidth = maxOf(1800, qrBitmap.width * 3)
        val pageHeight = (pageWidth * 1.4142f).toInt()
        val outerMargin = (pageWidth * 0.1f).toInt()
        val titleText = title.ifBlank { "QR Code" }

        val sheet = Bitmap.createBitmap(pageWidth, pageHeight, Bitmap.Config.ARGB_8888)
        val canvas = Canvas(sheet)
        canvas.drawColor(Color.WHITE)

        val titlePaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = Color.parseColor("#111827")
            textAlign = Paint.Align.CENTER
            typeface = Typeface.create(Typeface.DEFAULT_BOLD, Typeface.BOLD)
            textSize = pageWidth * 0.05f
        }

        val subtitlePaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = Color.parseColor("#6B7280")
            textAlign = Paint.Align.CENTER
            textSize = pageWidth * 0.022f
        }

        val titleBounds = Rect()
        titlePaint.getTextBounds(titleText, 0, titleText.length, titleBounds)
        val titleBaseline = outerMargin + titleBounds.height().toFloat()
        canvas.drawText(titleText, pageWidth / 2f, titleBaseline, titlePaint)

        val subtitle = "Generated by QR Studio Ultra"
        val subtitleY = titleBaseline + (pageWidth * 0.04f)
        canvas.drawText(subtitle, pageWidth / 2f, subtitleY, subtitlePaint)

        val availableWidth = pageWidth - (outerMargin * 2)
        val availableHeight = pageHeight - subtitleY.toInt() - outerMargin - (pageWidth * 0.06f).toInt()
        val qrSize = minOf((availableWidth * 0.78f).toInt(), (availableHeight * 0.88f).toInt())
        val qrLeft = (pageWidth - qrSize) / 2
        val qrTop = subtitleY.toInt() + ((availableHeight - qrSize) / 2)
        val qrDest = Rect(qrLeft, qrTop, qrLeft + qrSize, qrTop + qrSize)

        canvas.drawBitmap(qrBitmap, null, qrDest, Paint(Paint.ANTI_ALIAS_FLAG or Paint.FILTER_BITMAP_FLAG))

        return sheet
    }
}
