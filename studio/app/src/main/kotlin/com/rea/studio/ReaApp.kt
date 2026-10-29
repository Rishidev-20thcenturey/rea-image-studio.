package com.rea.studio

import android.app.Application

/**
 * Records why the process died, so the next launch can show it.
 *
 * Without this, a crash on a device the developer cannot attach to is invisible: Android shows
 * "the app keeps stopping" and nothing else. The handler writes the stack trace to internal storage
 * and then lets the platform do what it would have done anyway.
 */
class ReaApp : Application() {

    override fun onCreate() {
        super.onCreate()
        Diag.init(this)
        val previous = Thread.getDefaultUncaughtExceptionHandler()
        Thread.setDefaultUncaughtExceptionHandler { thread, error ->
            Diag.recordThrowable(error)
            previous?.uncaughtException(thread, error)
        }
    }
}
