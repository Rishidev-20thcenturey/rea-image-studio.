package com.rea.studio

/**
 * The JNI bindings to `librea_core.so` (studio/core). One-to-one with the Rust entry points in
 * `jni_bridge.rs`: the names here must match `Java_com_rea_studio_NativeCore_<name>` exactly.
 *
 * Every call takes an opaque document handle. A `Long` return of `0` means the call failed, and
 * [lastError] then carries the reason to show the user.
 */
object NativeCore {

    /**
     * Why the library could not be loaded, or `null` when it loaded.
     *
     * Loading is explicit rather than an `init` block: if `librea_core.so` is missing from the APK
     * or was built for a different ABI, the failure must become a message the user can send me, not
     * an `UnsatisfiedLinkError` thrown from a static initialiser on a screen that never appears.
     */
    var loadError: String? = null
        private set

    fun load(): Boolean {
        if (loadError != null) return false
        return try {
            System.loadLibrary("rea_core")
            true
        } catch (t: Throwable) {
            loadError = t.toString()
            false
        }
    }

    external fun selfTest(scratch: String): String

    external fun nativeVersion(): String
    external fun lastError(): String

    // --- documents ---
    external fun createDocument(w: Int, h: Int): Long
    external fun openDocument(path: String): Long
    external fun saveDocument(handle: Long, path: String): Boolean
    external fun closeDocument(handle: Long): Boolean
    external fun documentSize(handle: Long): String
    external fun selectionBounds(handle: Long): String
    external fun hasSelection(handle: Long): Boolean

    // --- rendering ---
    external fun render(
        handle: Long, out: IntArray, outW: Int, outH: Int,
        vx: Float, vy: Float, scale: Float
    ): Boolean

    external fun renderSelection(
        handle: Long, out: IntArray, outW: Int, outH: Int,
        vx: Float, vy: Float, scale: Float
    ): Boolean

    external fun layerThumb(handle: Long, index: Int, out: IntArray, outW: Int, outH: Int): Boolean
    external fun exportPixels(handle: Long, out: IntArray): Boolean

    // --- layers ---
    external fun layers(handle: Long): String
    external fun activeLayer(handle: Long): Int
    external fun selectLayer(handle: Long, index: Int): Boolean
    external fun addLayer(handle: Long, name: String): Boolean
    external fun addLayerFromPixels(handle: Long, name: String, w: Int, h: Int, pixels: IntArray): Boolean
    external fun deleteLayer(handle: Long, index: Int): Boolean
    external fun duplicateLayer(handle: Long, index: Int): Boolean
    external fun mergeDown(handle: Long, index: Int): Boolean
    external fun moveLayer(handle: Long, from: Int, to: Int): Boolean
    external fun setLayerVisible(handle: Long, index: Int, visible: Boolean): Boolean
    external fun setLayerOpacity(handle: Long, index: Int, opacity: Int): Boolean
    external fun setLayerBlend(handle: Long, index: Int, mode: Int): Boolean
    external fun setLayerName(handle: Long, index: Int, name: String): Boolean
    external fun addLayerMask(handle: Long, index: Int): Boolean
    external fun selectionToMask(handle: Long): Boolean
    external fun blendModes(): String

    // --- painting ---
    external fun strokeBegin(
        handle: Long, x: Float, y: Float, pressure: Float, tool: Int,
        color: Int, size: Float, hardness: Float, flow: Float, opacity: Float
    ): Boolean

    external fun strokeTo(handle: Long, x: Float, y: Float, pressure: Float): Boolean
    external fun strokeEnd(handle: Long): Boolean
    external fun bucketFill(handle: Long, x: Int, y: Int, color: Int, tolerance: Int): Boolean
    external fun fillSelection(handle: Long, color: Int): Boolean
    external fun clearSelection(handle: Long): Boolean

    // --- selection ---
    external fun selectRect(
        handle: Long, x0: Float, y0: Float, x1: Float, y1: Float, feather: Float
    ): Boolean

    external fun selectEllipse(
        handle: Long, x0: Float, y0: Float, x1: Float, y1: Float, feather: Float
    ): Boolean

    external fun selectPolygon(handle: Long, points: FloatArray, feather: Float): Boolean
    external fun selectAll(handle: Long): Boolean
    external fun selectNone(handle: Long): Boolean
    external fun invertSelection(handle: Long): Boolean
    external fun featherSelection(handle: Long, radius: Float): Boolean

    // --- adjustments, filters, transforms ---
    external fun adjust(handle: Long, kind: Int, params: FloatArray): Boolean
    external fun filter(handle: Long, kind: Int, params: FloatArray): Boolean
    external fun flip(handle: Long, horizontal: Boolean): Boolean
    external fun rotate90(handle: Long): Boolean
    external fun nudgeLayer(handle: Long, dx: Int, dy: Int): Boolean
    external fun cropToSelection(handle: Long): Boolean
    external fun resizeCanvas(handle: Long, w: Int, h: Int): Boolean

    // --- history ---
    external fun undo(handle: Long): Boolean
    external fun redo(handle: Long): Boolean
    external fun canUndo(handle: Long): Boolean
    external fun canRedo(handle: Long): Boolean
    external fun undoName(handle: Long): String
    external fun redoName(handle: Long): String
    external fun history(handle: Long): String
}
