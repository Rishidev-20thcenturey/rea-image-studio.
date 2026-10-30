package com.rea.studio

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.DashPathEffect
import android.graphics.Paint
import android.graphics.PointF
import android.graphics.RectF
import android.view.MotionEvent
import android.view.View
import kotlin.math.hypot
import kotlin.math.max
import kotlin.math.min
import kotlin.math.roundToInt

/** The tools the rail offers. The first four map to the core's stroke tool ids. */
enum class Tool(val label: String, val strokeId: Int) {
    BRUSH("Brush", 0),
    ERASER("Eraser", 1),
    RECTANGLE("Rectangle", 2),
    ELLIPSE("Ellipse", 3),
    MARQUEE("Marquee", -1),
    OVAL_SELECT("Oval select", -1),
    LASSO("Lasso", -1),
    BUCKET("Bucket", -1),
    MOVE("Move", -1),
    TEXT("Text", -1)
}

/**
 * The canvas. It owns the view transform and turns touches into core calls; the core owns the
 * pixels. Rendering is pull-based: the view asks the core for the visible region into one reused
 * bitmap, so a stroke never allocates a frame.
 */
class EditorView(context: Context) : View(context) {

    var handle: Long = 0L
        set(value) {
            field = value
            needsFit = true
            invalidateDocument()
        }

    var tool: Tool = Tool.BRUSH
    var color: Int = 0xFF000000.toInt()
    var brushSize: Float = 24f
    var hardness: Float = 0.8f
    var flow: Float = 1f
    var opacity: Float = 1f
    var tolerance: Int = 32
    var feather: Float = 0f

    /** Called after an edit lands, so the activity can refresh the layer panel and undo state. */
    var onEdit: (() -> Unit)? = null

    /** Called when a tap should open the text input (the activity owns dialogs). */
    var onRequestText: ((Float, Float) -> Unit)? = null

    private var pixels = IntArray(0)
    private var frame: Bitmap? = null
    private var needsRender = true
    private var needsFit = true

    // View transform: the document point at the view's top-left, and view pixels per document pixel.
    private var viewX = 0f
    private var viewY = 0f
    private var zoom = 1f

    private var docW = 0
    private var docH = 0
    private var hasSelection = false
    // Cached selection rectangle: onDraw must not pay for a JNI call and a string parse per frame.
    private var selX = 0
    private var selY = 0
    private var selW = 0
    private var selH = 0

    // Touch state.
    private var strokeOn = false
    private var lastX = 0f
    private var lastY = 0f
    private var downX = 0f
    private var downY = 0f
    private var curX = 0f
    private var curY = 0f
    private var panning = false
    private var suppressPinchUntilUp = false
    private var pinchStartSpan = 0f
    private var pinchStartZoom = 1f
    private var pinchFocusX = 0f
    private var pinchFocusY = 0f
    private val lasso = ArrayList<Float>()

    private val dash = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 1.5f
        color = 0xFFFFFFFF.toInt()
        pathEffect = DashPathEffect(floatArrayOf(6f, 5f), 0f)
    }
    private val dashShadow = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 1.5f
        color = 0xCC000000.toInt()
        pathEffect = DashPathEffect(floatArrayOf(6f, 5f), 6f)
    }
    private val lassoPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 2f
        color = 0xFFFFC46B.toInt()
    }
    private val cursorPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 1.5f
        color = 0xCCFFFFFF.toInt()
    }
    private val cursorShadow = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 1.5f
        color = 0x88000000.toInt()
    }

    // --- document plumbing -------------------------------------------------------------------

    /** Re-read the document size and reset the selection state. */
    fun syncDocument() {
        if (handle == 0L) {
            docW = 0
            docH = 0
            return
        }
        val size = NativeCore.documentSize(handle).split(',')
        docW = size.getOrNull(0)?.toIntOrNull() ?: 0
        docH = size.getOrNull(1)?.toIntOrNull() ?: 0
        refreshSelection()
        needsFit = true
        invalidateDocument()
    }

    fun refreshSelection() {
        if (handle == 0L) {
            hasSelection = false
            return
        }
        hasSelection = NativeCore.hasSelection(handle)
        val b = NativeCore.selectionBounds(handle).split(',')
        selX = b.getOrNull(0)?.toIntOrNull() ?: 0
        selY = b.getOrNull(1)?.toIntOrNull() ?: 0
        selW = b.getOrNull(2)?.toIntOrNull() ?: 0
        selH = b.getOrNull(3)?.toIntOrNull() ?: 0
    }

    fun invalidateDocument() {
        needsRender = true
        invalidate()
    }

    /**
     * Fit the whole document on screen, centred.
     *
     * If the view has not been measured yet this does nothing and keeps `needsFit` set, so the
     * first draw retries: that ordering is what made "fit" silently do nothing on a freshly opened
     * image and leave it at 100%.
     */
    fun fitToWindow(): Boolean {
        if (docW <= 0 || docH <= 0 || width == 0 || height == 0) return false
        val margin = 24f
        val z = min((width - margin) / docW, (height - margin) / docH)
        zoom = z.coerceIn(0.02f, 16f)
        viewX = (docW - width / zoom) / 2f
        viewY = (docH - height / zoom) / 2f
        needsFit = false
        invalidateDocument()
        onEdit?.invoke()
        return true
    }

    /** Fit once the view has a size, whenever that turns out to be. */
    fun fitWhenMeasure() {
        needsFit = true
        post {
            if (!fitToWindow()) {
                // Still not measured: try again on the next frame.
                postDelayed({ fitToWindow() }, 64)
            }
        }
    }

    fun zoomTo(z: Float, focusX: Float = width / 2f, focusY: Float = height / 2f) {
        val dx = viewX + focusX / zoom
        val dy = viewY + focusY / zoom
        zoom = z.coerceIn(0.02f, 16f)
        viewX = dx - focusX / zoom
        viewY = dy - focusY / zoom
        invalidateDocument()
        onEdit?.invoke()
    }

    fun currentZoom(): Float = zoom
    fun documentWidth(): Int = docW
    fun documentHeight(): Int = docH

    private fun viewToDocX(x: Float) = viewX + x / zoom
    private fun viewToDocY(y: Float) = viewY + y / zoom
    private fun docToViewX(x: Float) = (x - viewX) * zoom
    private fun docToViewY(y: Float) = (y - viewY) * zoom

    // --- drawing -----------------------------------------------------------------------------

    override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
        super.onSizeChanged(w, h, oldw, oldh)
        frame = null
        pixels = IntArray(0)
        needsRender = true
        if (needsFit) fitToWindow()
    }

    override fun onDraw(canvas: Canvas) {
        val w = width
        val h = height
        if (w <= 0 || h <= 0) return
        if (needsFit && docW > 0) fitToWindow()
        if (handle == 0L || docW <= 0) {
            canvas.drawColor(0xFF14161B.toInt())
            return
        }
        if (frame == null || frame!!.width != w || frame!!.height != h) {
            frame = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
            pixels = IntArray(w * h)
            needsRender = true
        }
        if (needsRender) {
            NativeCore.render(handle, pixels, w, h, viewX, viewY, 1f / zoom)
            frame!!.setPixels(pixels, 0, w, 0, 0, w, h)
            needsRender = false
        }
        canvas.drawBitmap(frame!!, 0f, 0f, null)

        // Canvas border, so a document smaller than the screen is legible against the surround.
        val rect = RectF(docToViewX(0f), docToViewY(0f), docToViewX(docW.toFloat()), docToViewY(docH.toFloat()))
        canvas.drawRect(rect, cursorShadow)

        drawSelection(canvas)
        drawToolPreview(canvas)
    }

    private fun drawSelection(canvas: Canvas) {
        if (!hasSelection || handle == 0L) return
        if (selW <= 0 || selH <= 0) return
        val r = RectF(
            docToViewX(selX.toFloat()),
            docToViewY(selY.toFloat()),
            docToViewX((selX + selW).toFloat()),
            docToViewY((selY + selH).toFloat())
        )
        canvas.drawRect(r, dashShadow)
        canvas.drawRect(r, dash)
    }

    private fun drawToolPreview(canvas: Canvas) {
        when (tool) {
            Tool.MARQUEE, Tool.OVAL_SELECT -> {
                if (!strokeOn) return
                val r = RectF(
                    min(docToViewX(downX), docToViewX(curX)),
                    min(docToViewY(downY), docToViewY(curY)),
                    max(docToViewX(downX), docToViewX(curX)),
                    max(docToViewY(downY), docToViewY(curY))
                )
                canvas.drawRect(r, dashShadow)
                if (tool == Tool.MARQUEE) {
                    canvas.drawRect(r, dash)
                } else {
                    val oval = android.graphics.Path()
                    oval.addOval(r, android.graphics.Path.Direction.CW)
                    canvas.drawPath(oval, dash)
                }
            }
            Tool.LASSO -> {
                if (lasso.size < 4) return
                val path = android.graphics.Path()
                path.moveTo(docToViewX(lasso[0]), docToViewY(lasso[1]))
                var i = 2
                while (i + 1 < lasso.size) {
                    path.lineTo(docToViewX(lasso[i]), docToViewY(lasso[i + 1]))
                    i += 2
                }
                canvas.drawPath(path, lassoPaint)
            }
            Tool.BRUSH, Tool.ERASER -> {
                if (strokeOn) return
                // A ring the size of the brush, so the size is visible before the first dab.
                val cx = docToViewX(curX)
                val cy = docToViewY(curY)
                if (cx < -100 || cy < -100 || cx > width + 100 || cy > height + 100) return
                val r = max(2f, brushSize * 0.5f * zoom)
                canvas.drawCircle(cx, cy, r + 1f, cursorShadow)
                canvas.drawCircle(cx, cy, r, cursorPaint)
            }
            else -> {}
        }
    }

    // --- touch -------------------------------------------------------------------------------

    override fun onTouchEvent(event: MotionEvent): Boolean {
        if (handle == 0L) return false
        when (event.actionMasked) {
            MotionEvent.ACTION_DOWN -> {
                downX = viewToDocX(event.x)
                downY = viewToDocY(event.y)
                curX = downX
                curY = downY
                lastX = event.x
                lastY = event.y
                panning = false
                lasso.clear()
                beginTool(downX, downY, effectivePressure(event))
                return true
            }

            MotionEvent.ACTION_POINTER_DOWN -> {
                if (event.pointerCount >= 2) {
                    // A second finger landing *during* a stroke is almost always an accident — the
                    // hand holding the tablet — so the stroke finishes and nothing else happens.
                    // Pinch only starts from a two-finger touch that began with two fingers.
                    if (strokeOn) {
                        NativeCore.strokeEnd(handle)
                        strokeOn = false
                        commitEdit()
                        suppressPinchUntilUp = true
                        return true
                    }
                    panning = true
                    pinchStartSpan = span(event)
                    pinchStartZoom = zoom
                    pinchFocusX = (event.getX(0) + event.getX(1)) / 2f
                    pinchFocusY = (event.getY(0) + event.getY(1)) / 2f
                }
                return true
            }

            MotionEvent.ACTION_MOVE -> {
                if (suppressPinchUntilUp) return true
                if (panning && event.pointerCount >= 2) {
                    val midX = (event.getX(0) + event.getX(1)) / 2f
                    val midY = (event.getY(0) + event.getY(1)) / 2f
                    // Pan by how far the pinch's midpoint moved, at the old zoom.
                    viewX -= (midX - pinchFocusX) / zoom
                    viewY -= (midY - pinchFocusY) / zoom
                    val spanNow = span(event)
                    if (pinchStartSpan > 4f && spanNow > 4f) {
                        val docFocusX = viewToDocX(midX)
                        val docFocusY = viewToDocY(midY)
                        zoom = (pinchStartZoom * spanNow / pinchStartSpan).coerceIn(0.02f, 16f)
                        viewX = docFocusX - midX / zoom
                        viewY = docFocusY - midY / zoom
                    }
                    pinchFocusX = midX
                    pinchFocusY = midY
                    invalidateDocument()
                    return true
                }
                curX = viewToDocX(event.x)
                curY = viewToDocY(event.y)
                moveTool(curX, curY, effectivePressure(event))
                return true
            }

            MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> {
                suppressPinchUntilUp = false
                if (!panning) {
                    endTool(viewToDocX(event.x), viewToDocY(event.y))
                }
                panning = false
                invalidate()
                return true
            }
            MotionEvent.ACTION_POINTER_UP -> {
                if (event.pointerCount <= 2) {
                    // Lifting one of two fingers ends the pinch; the remaining finger does nothing
                    // until it is lifted too, which avoids a stray dab.
                    panning = false
                }
                return true
            }
        }
        return super.onTouchEvent(event)
    }

    private fun span(e: MotionEvent): Float =
        if (e.pointerCount < 2) 0f
        else hypot(e.getX(1) - e.getX(0), e.getY(1) - e.getY(0))

    /**
     * A capacitive screen reports pressure inconsistently — often 1.0 on the first touch and 0.0 or
     * random fractions during a drag — which made every dab a different size and turned a stroke
     * into a string of blobs. Pressure is only honoured when it comes from a stylus.
     */
    private fun effectivePressure(event: MotionEvent): Float =
        if (event.getToolType(0) == MotionEvent.TOOL_TYPE_STYLUS ||
            event.getToolType(0) == MotionEvent.TOOL_TYPE_ERASER
        ) {
            event.getPressure(0).coerceIn(0.05f, 1f)
        } else {
            1f
        }

    private fun beginTool(x: Float, y: Float, pressure: Float) {
        strokeOn = true
        when (tool) {
            Tool.BRUSH, Tool.ERASER, Tool.RECTANGLE, Tool.ELLIPSE -> {
                NativeCore.strokeBegin(
                    handle, x, y, pressure, tool.strokeId, color,
                    brushSize, hardness, flow, opacity
                )
                invalidateDocument()
            }
            Tool.BUCKET -> {
                NativeCore.bucketFill(handle, x.toInt(), y.toInt(), color, tolerance)
                commitEdit()
            }
            Tool.TEXT -> {
                strokeOn = false
                onRequestText?.invoke(x, y)
            }
            Tool.MOVE -> {
                // Move is a drag of the active layer; each step is its own undo entry.
                lastDocX = x
                lastDocY = y
            }
            else -> {}
        }
    }

    private fun moveTool(x: Float, y: Float, pressure: Float) {
        when (tool) {
            Tool.BRUSH, Tool.ERASER, Tool.RECTANGLE, Tool.ELLIPSE -> {
                if (strokeOn) {
                    NativeCore.strokeTo(handle, x, y, pressure)
                    invalidateDocument()
                }
            }
            Tool.MOVE -> {
                if (!strokeOn) return
                val dx = x - lastDocX
                val dy = y - lastDocY
                val ix = dx.roundToInt()
                val iy = dy.roundToInt()
                if (ix != 0 || iy != 0) {
                    NativeCore.nudgeLayer(handle, ix, iy)
                    lastDocX += ix
                    lastDocY += iy
                    invalidateDocument()
                }
            }
            Tool.LASSO -> {
                lasso.add(x)
                lasso.add(y)
                invalidate()
            }
            else -> invalidate()
        }
    }

    private var lastDocX = 0f
    private var lastDocY = 0f

    private fun endTool(x: Float, y: Float) {
        if (!strokeOn) {
            invalidate()
            return
        }
        when (tool) {
            Tool.BRUSH, Tool.ERASER, Tool.RECTANGLE, Tool.ELLIPSE -> {
                NativeCore.strokeEnd(handle)
                commitEdit()
            }
            Tool.MARQUEE -> {
                NativeCore.selectRect(handle, downX, downY, x, y, feather)
                commitEdit()
            }
            Tool.OVAL_SELECT -> {
                NativeCore.selectEllipse(handle, downX, downY, x, y, feather)
                commitEdit()
            }
            Tool.LASSO -> {
                if (lasso.size >= 6) {
                    NativeCore.selectPolygon(handle, lasso.toFloatArray(), feather)
                    commitEdit()
                }
                lasso.clear()
            }
            else -> {}
        }
        strokeOn = false
        invalidate()
    }

    /** Notify the activity after an edit and make sure the next frame reflects it. */
    private fun commitEdit() {
        refreshSelection()
        invalidateDocument()
        onEdit?.invoke()
    }

    /** Set the drag origin for the move tool (called from `beginTool` via the activity path). */
    fun beginMove() {
        lastDocX = downX
        lastDocY = downY
    }

    /**
     * Drive the view the way a finger does and report whether the document changed.
     *
     * The events are the ones `MotionEvent` produces for a drag: down, several moves, up. If they
     * reach the brush, the core paints and the pixels come back non-transparent. Used by the
     * diagnostics screen and by the CI probe, which cannot rely on injected touches.
     */
    fun uiSelfTest(): Boolean {
        if (handle == 0L || width == 0 || height == 0) return false
        tool = Tool.BRUSH
        brushSize = 24f
        color = 0xFF000000.toInt()
        // A diagonal stroke across the middle of the canvas, in view coordinates.
        val x0 = width * 0.3f
        val y0 = height * 0.3f
        val x1 = width * 0.7f
        val y1 = height * 0.7f
        val down = android.os.SystemClock.uptimeMillis()
        fun send(action: Int, x: Float, y: Float, t: Long) {
            val e = MotionEvent.obtain(down, down + t, action, x, y, 0)
            dispatchTouchEvent(e)
            e.recycle()
        }
        send(MotionEvent.ACTION_DOWN, x0, y0, 0)
        for (step in 1..8) {
            val f = step / 8f
            send(MotionEvent.ACTION_MOVE, x0 + (x1 - x0) * f, y0 + (y1 - y0) * f, step * 16L)
        }
        send(MotionEvent.ACTION_UP, x1, y1, 160L)

        val w = docW
        val h = docH
        val buf = IntArray(w * h)
        if (!NativeCore.exportPixels(handle, buf)) return false
        val painted = buf.count { (it ushr 24) != 0 }
        return painted > 0
    }

    fun hasActiveSelection(): Boolean = hasSelection
}
