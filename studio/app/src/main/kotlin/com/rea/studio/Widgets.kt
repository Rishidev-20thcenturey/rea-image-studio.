package com.rea.studio

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.ComposeShader
import android.graphics.LinearGradient
import android.graphics.Paint
import android.graphics.Path
import android.graphics.PorterDuff
import android.graphics.RectF
import android.graphics.Shader
import android.graphics.Typeface
import android.view.MotionEvent
import android.view.View
import android.widget.LinearLayout
import android.widget.SeekBar
import android.widget.TextView

/** A button that draws its own icon, so the app ships without a pile of image assets. */
class IconView(context: Context, val kind: String) : View(context) {

    var tint: Int = 0xFFE6E9EF.toInt()
    var active: Boolean = false
        set(value) {
            field = value
            invalidate()
        }

    init {
        isClickable = true
        isFocusable = true
    }

    override fun onDraw(canvas: Canvas) {
        val p = Paint(Paint.ANTI_ALIAS_FLAG)
        val w = width.toFloat()
        val h = height.toFloat()
        val s = minOf(w, h)
        val cx = w / 2f
        val cy = h / 2f
        if (active) {
            val bg = Paint(Paint.ANTI_ALIAS_FLAG)
            bg.color = 0xFF2A3B57.toInt()
            canvas.drawRoundRect(RectF(4f, 4f, w - 4f, h - 4f), 10f, 10f, bg)
            val bar = Paint(Paint.ANTI_ALIAS_FLAG)
            bar.color = 0xFF6FA8FF.toInt()
            canvas.drawRoundRect(RectF(1.5f, h * 0.22f, 4.5f, h * 0.78f), 2f, 2f, bar)
        }
        p.color = if (active) 0xFF8FBBFF.toInt() else tint
        p.strokeWidth = s * 0.07f
        p.strokeCap = Paint.Cap.ROUND
        p.strokeJoin = Paint.Join.ROUND
        val r = s * 0.28f
        when (kind) {
            "menu" -> {
                p.strokeWidth = s * 0.075f
                for (i in -1..1) {
                    canvas.drawLine(cx - r, cy + i * r * 0.62f, cx + r, cy + i * r * 0.62f, p)
                }
            }
            "more" -> {
                p.style = Paint.Style.FILL
                for (i in -1..1) canvas.drawCircle(cx, cy + i * r * 0.62f, s * 0.045f, p)
            }
            "undo", "redo" -> {
                p.style = Paint.Style.STROKE
                val path = Path()
                val box = RectF(cx - r, cy - r * 0.75f, cx + r, cy + r * 0.85f)
                val flip = kind == "undo"
                path.addArc(box, if (flip) 200f else 340f, if (flip) -230f else 230f)
                canvas.drawPath(path, p)
                // Arrow head at the arc's start.
                val ax = if (flip) cx - r * 0.98f else cx + r * 0.98f
                val ay = cy + r * 0.05f
                p.strokeWidth = s * 0.06f
                canvas.drawLine(ax, ay, ax + (if (flip) s * 0.11f else -s * 0.11f), ay - s * 0.12f, p)
                canvas.drawLine(ax, ay, ax + (if (flip) s * 0.11f else -s * 0.11f), ay + s * 0.1f, p)
            }
            "layers" -> {
                p.style = Paint.Style.STROKE
                for (i in 0..2) {
                    val y = cy + (i - 1) * r * 0.75f
                    val path = Path()
                    path.moveTo(cx, y - r * 0.42f)
                    path.lineTo(cx + r, y)
                    path.lineTo(cx, y + r * 0.42f)
                    path.lineTo(cx - r, y)
                    path.close()
                    canvas.drawPath(path, p)
                }
            }
            "brush" -> {
                p.style = Paint.Style.STROKE
                canvas.drawLine(cx - r * 0.7f, cy + r * 0.7f, cx + r * 0.45f, cy - r * 0.7f, p)
                p.style = Paint.Style.FILL
                val tip = Path()
                tip.moveTo(cx - r * 0.85f, cy + r * 0.85f)
                tip.lineTo(cx - r * 0.95f, cy + r * 1.1f)
                tip.lineTo(cx - r * 0.55f, cy + r * 1.05f)
                tip.close()
                canvas.drawPath(tip, p)
            }
            "eraser" -> {
                p.style = Paint.Style.STROKE
                canvas.save()
                canvas.rotate(-40f, cx, cy)
                canvas.drawRoundRect(RectF(cx - r * 0.95f, cy - r * 0.55f, cx + r * 0.95f, cy + r * 0.55f), 3f, 3f, p)
                canvas.drawLine(cx - r * 0.3f, cy - r * 0.55f, cx - r * 0.3f, cy + r * 0.55f, p)
                canvas.restore()
            }
            "rect" -> {
                p.style = Paint.Style.STROKE
                canvas.drawRect(RectF(cx - r, cy - r * 0.8f, cx + r, cy + r * 0.8f), p)
            }
            "oval" -> {
                p.style = Paint.Style.STROKE
                canvas.drawOval(RectF(cx - r, cy - r * 0.85f, cx + r, cy + r * 0.85f), p)
            }
            "marquee" -> {
                p.style = Paint.Style.STROKE
                p.pathEffect = android.graphics.DashPathEffect(floatArrayOf(s * 0.09f, s * 0.07f), 0f)
                canvas.drawRect(RectF(cx - r, cy - r * 0.85f, cx + r, cy + r * 0.85f), p)
                p.pathEffect = null
            }
            "ovalselect" -> {
                p.style = Paint.Style.STROKE
                p.pathEffect = android.graphics.DashPathEffect(floatArrayOf(s * 0.09f, s * 0.07f), 0f)
                canvas.drawOval(RectF(cx - r, cy - r * 0.85f, cx + r, cy + r * 0.85f), p)
                p.pathEffect = null
            }
            "lasso" -> {
                p.style = Paint.Style.STROKE
                p.pathEffect = android.graphics.DashPathEffect(floatArrayOf(s * 0.08f, s * 0.06f), 0f)
                val path = Path()
                path.addOval(RectF(cx - r, cy - r * 0.95f, cx + r, cy + r * 0.6f), Path.Direction.CW)
                canvas.drawPath(path, p)
                p.pathEffect = null
                canvas.drawLine(cx - r * 0.85f, cy + r * 0.4f, cx - r * 0.55f, cy + r * 1.0f, p)
            }
            "bucket" -> {
                p.style = Paint.Style.FILL
                val tri = Path()
                tri.moveTo(cx - r * 0.9f, cy - r * 0.2f)
                tri.lineTo(cx + r * 0.3f, cy - r * 0.9f)
                tri.lineTo(cx + r * 0.3f, cy + r * 0.5f)
                tri.close()
                canvas.drawPath(tri, p)
                canvas.drawCircle(cx + r * 0.75f, cy + r * 0.75f, s * 0.075f, p)
            }
            "move" -> {
                p.strokeWidth = s * 0.06f
                canvas.drawLine(cx - r, cy, cx + r, cy, p)
                canvas.drawLine(cx, cy - r, cx, cy + r, p)
                val t = s * 0.09f
                canvas.drawLine(cx + r, cy, cx + r - t, cy - t, p)
                canvas.drawLine(cx + r, cy, cx + r - t, cy + t, p)
                canvas.drawLine(cx - r, cy, cx - r + t, cy - t, p)
                canvas.drawLine(cx - r, cy, cx - r + t, cy + t, p)
                canvas.drawLine(cx, cy - r, cx - t, cy - r + t, p)
                canvas.drawLine(cx, cy - r, cx + t, cy - r + t, p)
                canvas.drawLine(cx, cy + r, cx - t, cy + r - t, p)
                canvas.drawLine(cx, cy + r, cx + t, cy + r - t, p)
            }
            "text" -> {
                val tp = Paint(Paint.ANTI_ALIAS_FLAG)
                tp.color = p.color
                tp.textSize = s * 0.62f
                tp.typeface = Typeface.create(Typeface.SERIF, Typeface.BOLD)
                tp.textAlign = Paint.Align.CENTER
                canvas.drawText("T", cx, cy + s * 0.22f, tp)
            }
            "up" -> {
                canvas.drawLine(cx, cy - r * 0.6f, cx, cy + r * 0.7f, p)
                canvas.drawLine(cx, cy - r * 0.6f, cx - r * 0.55f, cy - r * 0.05f, p)
                canvas.drawLine(cx, cy - r * 0.6f, cx + r * 0.55f, cy - r * 0.05f, p)
            }
            "down" -> {
                canvas.drawLine(cx, cy + r * 0.6f, cx, cy - r * 0.7f, p)
                canvas.drawLine(cx, cy + r * 0.6f, cx - r * 0.55f, cy + r * 0.05f, p)
                canvas.drawLine(cx, cy + r * 0.6f, cx + r * 0.55f, cy + r * 0.05f, p)
            }
            "add" -> {
                canvas.drawLine(cx - r * 0.7f, cy, cx + r * 0.7f, cy, p)
                canvas.drawLine(cx, cy - r * 0.7f, cx, cy + r * 0.7f, p)
            }
            "close" -> {
                canvas.drawLine(cx - r * 0.65f, cy - r * 0.65f, cx + r * 0.65f, cy + r * 0.65f, p)
                canvas.drawLine(cx + r * 0.65f, cy - r * 0.65f, cx - r * 0.65f, cy + r * 0.65f, p)
            }
            "check" -> {
                val path = Path()
                path.moveTo(cx - r * 0.75f, cy)
                path.lineTo(cx - r * 0.15f, cy + r * 0.6f)
                path.lineTo(cx + r * 0.8f, cy - r * 0.6f)
                canvas.drawPath(path, p)
            }
            "mask" -> {
                p.style = Paint.Style.STROKE
                canvas.drawRect(RectF(cx - r, cy - r * 0.8f, cx + r, cy + r * 0.8f), p)
                p.style = Paint.Style.FILL
                canvas.drawRect(RectF(cx - r * 0.45f, cy - r * 0.35f, cx + r * 0.45f, cy + r * 0.35f), p)
            }
            else -> {
                p.style = Paint.Style.STROKE
                canvas.drawCircle(cx, cy, r, p)
            }
        }
    }
}

/** A colour swatch button: a filled circle with a checkerboard ring. */
class SwatchView(context: Context) : View(context) {
    var color: Int = 0xFF000000.toInt()
        set(value) {
            field = value
            invalidate()
        }

    override fun onDraw(canvas: Canvas) {
        val r = minOf(width, height) * 0.5f - 3f
        val cx = width / 2f
        val cy = height / 2f
        val ring = Paint(Paint.ANTI_ALIAS_FLAG)
        ring.style = Paint.Style.STROKE
        ring.strokeWidth = 2.5f
        ring.color = 0xFF9AA6B8.toInt()
        canvas.drawCircle(cx, cy, r, ring)
        val fill = Paint(Paint.ANTI_ALIAS_FLAG)
        fill.color = color
        canvas.drawCircle(cx, cy, r - 2f, fill)
    }
}

/** Saturation/value square plus a hue strip: the colour picker. */
class ColorPickerView(context: Context) : View(context) {

    var color: Int = 0xFF000000.toInt()
        set(value) {
            field = value
            val hsv = FloatArray(3)
            Color.colorToHSV(value, hsv)
            hue = hsv[0]
            sat = hsv[1]
            value2 = hsv[2]
            invalidate()
        }

    var onColor: ((Int) -> Unit)? = null

    private var hue = 0f
    private var sat = 0f
    private var value2 = 0f
    private val svPaint = Paint(Paint.ANTI_ALIAS_FLAG)
    private val huePaint = Paint(Paint.ANTI_ALIAS_FLAG)
    private val thumb = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 3f
        color = Color.WHITE
    }
    private val thumbDark = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        strokeWidth = 1f
        color = 0x88000000.toInt()
    }

    private fun svRect(): RectF = RectF(0f, 0f, width.toFloat(), height * 0.72f)
    private fun hueRect(): RectF = RectF(0f, height * 0.78f, width.toFloat(), height * 0.92f)

    override fun onDraw(canvas: Canvas) {
        val sv = svRect()
        val base = Color.HSVToColor(floatArrayOf(hue, 1f, 1f))
        svPaint.shader = ComposeShader(
            LinearGradient(sv.left, sv.top, sv.right, sv.top, Color.WHITE, base, Shader.TileMode.CLAMP),
            LinearGradient(sv.left, sv.top, sv.left, sv.bottom, 0x00000000, 0xFF000000.toInt(), Shader.TileMode.CLAMP),
            PorterDuff.Mode.MULTIPLY
        )
        canvas.drawRect(sv, svPaint)

        val hues = intArrayOf(
            0xFFFF0000.toInt(), 0xFFFFFF00.toInt(), 0xFF00FF00.toInt(),
            0xFF00FFFF.toInt(), 0xFF0000FF.toInt(), 0xFFFF00FF.toInt(), 0xFFFF0000.toInt()
        )
        huePaint.shader = LinearGradient(0f, 0f, width.toFloat(), 0f, hues, null, Shader.TileMode.CLAMP)
        canvas.drawRect(hueRect(), huePaint)

        val tx = sat * sv.width() + sv.left
        val ty = (1f - value2) * sv.height() + sv.top
        canvas.drawCircle(tx, ty, 12f, thumb)
        canvas.drawCircle(tx, ty, 13f, thumbDark)

        val hx = hue / 360f * width
        canvas.drawCircle(hx, hueRect().centerY(), 10f, thumb)
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        val sv = svRect()
        val hr = hueRect()
        when (event.actionMasked) {
            MotionEvent.ACTION_DOWN, MotionEvent.ACTION_MOVE -> {
                if (event.y <= sv.bottom) {
                    sat = ((event.x - sv.left) / sv.width()).coerceIn(0f, 1f)
                    value2 = (1f - (event.y - sv.top) / sv.height()).coerceIn(0f, 1f)
                } else if (event.y >= hr.top - 20f) {
                    hue = ((event.x / width).coerceIn(0f, 1f)) * 360f
                }
                color = Color.HSVToColor(floatArrayOf(hue, sat, value2))
                onColor?.invoke(color)
                invalidate()
                return true
            }
        }
        return super.onTouchEvent(event)
    }
}

/** A slider's definition: its label, its range and where it starts. */
data class SliderSpec(val label: String, val from: Float, val to: Float, val initial: Float)

/** One labelled slider row, used by the tool options and every adjustment dialog. */
class SliderRow(
    context: Context,
    label: String,
    private val from: Float,
    private val to: Float,
    initial: Float,
    private val unit: String = ""
) : LinearLayout(context) {

    val bar = SeekBar(context)
    private val name = TextView(context)
    private val readout = TextView(context)
    private val steps = 1000

    init {
        orientation = HORIZONTAL
        gravity = android.view.Gravity.CENTER_VERTICAL
        name.text = label
        name.setTextColor(0xFFB9C2D0.toInt())
        name.textSize = 12.5f
        name.width = (context.resources.displayMetrics.density * 74).toInt()
        readout.setTextColor(0xFFE6E9EF.toInt())
        readout.textSize = 12.5f
        readout.gravity = android.view.Gravity.END
        readout.width = (context.resources.displayMetrics.density * 56).toInt()
        bar.max = steps
        bar.progress = (((initial - from) / (to - from)) * steps).toInt().coerceIn(0, steps)
        bar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                readout.text = format(value())
            }
            override fun onStartTrackingTouch(sb: SeekBar?) {}
            override fun onStopTrackingTouch(sb: SeekBar?) {}
        })
        readout.text = format(value())
        addView(name)
        addView(bar, LayoutParams(0, LayoutParams.WRAP_CONTENT, 1f))
        addView(readout)
    }

    fun value(): Float = from + (to - from) * (bar.progress.toFloat() / steps)

    private fun format(v: Float): String =
        if (to - from <= 4f) String.format("%.2f", v) + unit else v.toInt().toString() + unit
}
