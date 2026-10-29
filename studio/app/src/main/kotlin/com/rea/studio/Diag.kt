package com.rea.studio

import android.content.Context
import java.io.File

/**
 * A tiny, dependency-free boot recorder.
 *
 * This app is developed on a machine that cannot run it, so a failure on a real device has to be
 * readable from the device itself. Every launch appends a line per startup step to `boot.log` and
 * drops a `boot-ok` marker once the interface is up. If the next launch finds the marker missing,
 * the previous run died — and the last line of its log says where.
 *
 * Nothing here may throw: it is called from the crash handler and from the very first line of
 * `onCreate`, where an exception would be the second failure.
 */
object Diag {

    private var dir: File? = null
    private var appVersion: String = "?"

    fun init(context: Context) {
        runCatching {
            dir = File(context.filesDir, "diag").apply { mkdirs() }
            appVersion = runCatching {
                context.packageManager.getPackageInfo(context.packageName, 0).versionName ?: "?"
            }.getOrDefault("?")
        }
    }

    private fun file(name: String): File? {
        val d = dir ?: return null
        return runCatching { File(d, name).also { d.mkdirs() } }.getOrNull()
    }

    private var lastBootFailed = false

    /**
     * Start a session. The verdict on the previous launch is taken *before* the log is moved aside,
     * because moving it is what destroys the evidence.
     */
    fun beginBoot() {
        runCatching {
            val ok = file("boot-ok")
            val cur = file("boot.log")
            lastBootFailed = (ok == null || !ok.exists()) && cur != null && cur.exists() && cur.length() > 0
            val prev = file("prev-boot.log")
            if (cur != null && cur.exists() && cur.length() > 0) {
                prev?.delete()
                cur.renameTo(prev ?: return@runCatching)
            }
            ok?.delete()
            append("boot.log", "REA Image Studio $appVersion")
        }
    }

    /** Record one startup step. Cheap: one line, flushed immediately. */
    fun step(name: String) {
        append("boot.log", name)
    }

    fun markBootComplete() {
        append("boot-ok", "ok ${System.currentTimeMillis()}")
    }

    /** True when the previous launch never reached a drawn interface. */
    fun previousBootFailed(): Boolean = lastBootFailed

    fun previousBootLog(): String = read("prev-boot.log")

    fun crashText(): String = read("crash.txt")

    /** Forget past failures, once the user has read them. */
    fun clearReports() {
        runCatching {
            file("prev-boot.log")?.delete()
            file("crash.txt")?.delete()
            lastBootFailed = false
        }
    }

    fun recordThrowable(t: Throwable) {
        runCatching {
            val text = buildString {
                append("thrown at ").append(System.currentTimeMillis()).append('\n')
                append(t.toString()).append('\n')
                for (frame in t.stackTrace) append("    at ").append(frame).append('\n')
                var cause = t.cause
                var depth = 0
                while (cause != null && depth < 5) {
                    append("caused by ").append(cause.toString()).append('\n')
                    for (frame in cause.stackTrace) append("    at ").append(frame).append('\n')
                    cause = cause.cause
                    depth++
                }
            }
            file("crash.txt")?.writeText(text)
        }
    }

    private fun append(name: String, line: String) {
        runCatching {
            val f = file(name) ?: return
            f.appendText(line + "\n")
        }
    }

    private fun read(name: String): String = runCatching {
        val f = file(name) ?: return@runCatching ""
        if (f.exists()) f.readText() else ""
    }.getOrDefault("")

    /** Where the app keeps its evidence, shown in the Diagnostics panel. */
    fun directory(): String = dir?.absolutePath ?: "(unavailable)"
}
