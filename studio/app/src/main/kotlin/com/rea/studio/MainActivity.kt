package com.rea.studio

import android.app.Activity
import android.app.AlertDialog
import android.app.Dialog
import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.drawable.GradientDrawable
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.text.InputType
import android.view.Gravity
import android.view.View
import android.view.ViewGroup
import android.view.Window
import android.view.WindowInsets
import android.view.WindowInsetsController
import android.view.WindowManager
import android.widget.EditText
import android.widget.FrameLayout
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.SeekBar
import android.widget.Spinner
import android.widget.ArrayAdapter
import android.widget.TextView
import android.widget.Toast
import java.io.File
import java.io.InputStream
import java.util.Locale

/**
 * REA Image Studio. The whole interface is built in code: an immersive full-bleed canvas with a
 * floating tool rail, a top bar, a contextual options strip, a layers panel and a colour picker.
 * Every control is backed by a real operation in the core; nothing here is decorative.
 */
class MainActivity : Activity() {

    private lateinit var editor: EditorView
    private var handle = 0L
    private var docName = "Untitled"
    private var documentUri: Uri? = null

    private lateinit var titleView: TextView
    private lateinit var statusView: TextView
    private lateinit var optionsHost: LinearLayout
    private lateinit var rail: LinearLayout
    private lateinit var layersCard: LinearLayout
    private lateinit var layersList: LinearLayout
    private lateinit var blendSpinner: Spinner
    private lateinit var layerOpacity: SeekBar
    private lateinit var layerOpacityLabel: TextView
    private lateinit var swatch: SwatchView
    private lateinit var undoBtn: IconView
    private lateinit var redoBtn: IconView
    private lateinit var layersBtn: IconView

    private val railButtons = ArrayList<Pair<Tool, IconView>>()
    private var updatingPanel = false

    private val reqOpenImage = 1001
    private val reqOpenDoc = 1002
    private val reqSaveDoc = 1003
    private val reqExportPng = 1004
    private val reqExportJpeg = 1005

    private val dp: Int get() = resources.displayMetrics.density.toInt()

    private fun d(v: Float): Int = (v * resources.displayMetrics.density).toInt()

    // --- lifecycle ---------------------------------------------------------------------------

    override fun onCreate(state: Bundle?) {
        super.onCreate(state)
        // Every startup step is recorded, so a crash on a device leaves a trail the next launch can
        // show. This runs first because everything after it is what can fail.
        Diag.beginBoot()
        // And if the interface cannot be built at all, the failure becomes a readable screen rather
        // than a "keeps stopping" dialog with nothing behind it.
        try {
            buildInterface()
        } catch (t: Throwable) {
            Diag.recordThrowable(t)
            showFatal(t)
        }
    }

    private fun buildInterface() {
        Diag.step("onCreate")
        if (!NativeCore.load()) {
            Diag.step("native library failed: ${NativeCore.loadError}")
        } else {
            Diag.step("native library loaded: ${runCatching { NativeCore.nativeVersion() }.getOrDefault("?")}")
        }
        goImmersive()
        Diag.step("immersive mode set")
        val root = FrameLayout(this)
        root.setBackgroundColor(0xFF0E1014.toInt())

        Diag.step("building the canvas")
        editor = EditorView(this)
        editor.onEdit = { refreshAll() }
        editor.onRequestText = { x, y -> askForText(x, y) }
        root.addView(editor, FrameLayout.LayoutParams(-1, -1))

        buildTopBar(root)
        buildToolRail(root)
        buildOptionsBar(root)
        buildLayersPanel(root)
        buildSwatch(root)
        buildStatus(root)

        setContentView(root)
        Diag.step("interface laid out")
        // Start on the brush with its rail button lit, so the first stroke does what the screen says.
        selectTool(Tool.BRUSH)
        if (NativeCore.loadError != null) {
            // Nothing that needs pixels can run, so show why instead of throwing from every call.
            Diag.step("no document: the native library is unavailable")
            Diag.markBootComplete()
            showDiagnostics()
            return
        }
        newDocument(2048, 1536)
        Diag.step("document created")
        Diag.markBootComplete()
        // The previous launch never finished: say why, now that there is a screen to say it on.
        if (Diag.previousBootFailed()) showDiagnostics()
        // `am start -n com.rea.studio/.MainActivity --ez rea.selftest true` runs the interface's own
        // end-to-end check and writes the result to files/diag/ui-selftest.txt. It exists because a
        // CI emulator cannot be trusted to deliver injected touches to the right window.
        if (intent?.getBooleanExtra("rea.selftest", false) == true) runUiSelfTest()
    }

    /** Edge to edge, system bars hidden with a swipe to reveal: the canvas owns the screen. */
    @Suppress("DEPRECATION")
    private fun goImmersive() {
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            window.attributes = window.attributes.apply {
                layoutInDisplayCutoutMode =
                    WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
            }
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            window.setDecorFitsSystemWindows(false)
            window.insetsController?.let {
                it.hide(WindowInsets.Type.systemBars())
                it.systemBarsBehavior =
                    WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
            }
        } else {
            window.decorView.systemUiVisibility = (
                View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                    or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                    or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                    or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                    or View.SYSTEM_UI_FLAG_FULLSCREEN
                    or View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                )
        }
    }

    /**
     * Shown when building the interface threw. It says what happened, in text the user can copy —
     * this is the difference between a report I can act on and a screenshot of "keeps stopping".
     */
    private fun showFatal(t: Throwable) {
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(16f), d(8f), d(16f), d(8f))
        }
        val text = buildString {
            append(t.toString()).append("\n\n")
            for (frame in t.stackTrace.take(24)) append("    at ").append(frame).append('\n')
            append("\n").append(Diag.previousBootLog())
        }
        body.addView(TextView(this).apply {
            this.text = text
            setTextColor(0xFFD6DBE3.toInt())
            textSize = 10.5f
            typeface = android.graphics.Typeface.MONOSPACE
            setTextIsSelectable(true)
        })
        AlertDialog.Builder(this)
            .setTitle("This build could not start")
            .setMessage("REA Studio failed while building its interface. The details below are what I need to fix it.")
            .setView(ScrollView(this).apply { addView(body) })
            .setPositiveButton("Close") { _, _ -> finish() }
            .setNeutralButton("Copy") { _, _ ->
                val clipboard = getSystemService(CLIPBOARD_SERVICE) as android.content.ClipboardManager
                clipboard.setPrimaryClip(android.content.ClipData.newPlainText("REA failure", text))
                toast("Copied")
            }
            .show()
    }

    // --- chrome ------------------------------------------------------------------------------

    private fun barBackground(alpha: Int = 0xF2): GradientDrawable = GradientDrawable().apply {
        cornerRadius = d(16f).toFloat()
        setColor(Color.argb(alpha, 22, 25, 31))
        setStroke(d(1f), 0xFF2A2F3A.toInt())
    }

    private fun buildTopBar(root: FrameLayout) {
        val bar = LinearLayout(this)
        bar.orientation = LinearLayout.HORIZONTAL
        bar.gravity = Gravity.CENTER_VERTICAL
        bar.background = barBackground()
        bar.setPadding(d(6f), d(4f), d(6f), d(4f))

        val menu = IconView(this, "menu").also {
            it.setOnClickListener { showMenu() }
            bar.addView(it, LinearLayout.LayoutParams(d(44f), d(44f)))
        }

        titleView = TextView(this).apply {
            text = docName
            setTextColor(0xFFE6E9EF.toInt())
            textSize = 14f
            setPadding(d(10f), 0, d(10f), 0)
            isSingleLine = true
        }
        bar.addView(titleView, LinearLayout.LayoutParams(0, -2, 1f))

        undoBtn = IconView(this, "undo").also {
            it.setOnClickListener { doUndo() }
            bar.addView(it, LinearLayout.LayoutParams(d(44f), d(44f)))
        }
        redoBtn = IconView(this, "redo").also {
            it.setOnClickListener { doRedo() }
            bar.addView(it, LinearLayout.LayoutParams(d(44f), d(44f)))
        }
        layersBtn = IconView(this, "layers").also {
            it.setOnClickListener { toggleLayers() }
            bar.addView(it, LinearLayout.LayoutParams(d(44f), d(44f)))
        }
        IconView(this, "more").also {
            it.setOnClickListener { showMenu() }
            bar.addView(it, LinearLayout.LayoutParams(d(44f), d(44f)))
        }

        val lp = FrameLayout.LayoutParams(-1, -2)
        lp.setMargins(d(8f), d(8f), d(8f), 0)
        root.addView(bar, lp)
    }

    private fun buildToolRail(root: FrameLayout) {
        rail = LinearLayout(this)
        rail.orientation = LinearLayout.VERTICAL
        rail.gravity = Gravity.CENTER
        rail.background = barBackground(0xE6)
        rail.setPadding(d(4f), d(6f), d(4f), d(6f))

        for (tool in Tool.entries) {
            val icon = when (tool) {
                Tool.BRUSH -> "brush"
                Tool.ERASER -> "eraser"
                Tool.RECTANGLE -> "rect"
                Tool.ELLIPSE -> "oval"
                Tool.MARQUEE -> "marquee"
                Tool.OVAL_SELECT -> "ovalselect"
                Tool.LASSO -> "lasso"
                Tool.BUCKET -> "bucket"
                Tool.MOVE -> "move"
                Tool.TEXT -> "text"
            }
            val button = IconView(this, icon)
            button.contentDescription = tool.label
            button.setOnClickListener { selectTool(tool) }
            val lp = LinearLayout.LayoutParams(d(44f), d(44f))
            lp.bottomMargin = d(2f)
            rail.addView(button, lp)
            railButtons.add(tool to button)
        }

        val lp = FrameLayout.LayoutParams(-2, -2, Gravity.START or Gravity.CENTER_VERTICAL)
        lp.leftMargin = d(8f)
        root.addView(rail, lp)
    }

    private fun buildOptionsBar(root: FrameLayout) {
        optionsHost = LinearLayout(this)
        optionsHost.orientation = LinearLayout.HORIZONTAL
        optionsHost.gravity = Gravity.CENTER_VERTICAL
        optionsHost.background = barBackground()
        optionsHost.setPadding(d(10f), d(2f), d(10f), d(2f))

        // Full width minus the rail, and the sliders inside share what is left: fixed widths
        // overflowed a 320dp display and pushed the second control off the edge.
        val lp = FrameLayout.LayoutParams(-1, d(52f), Gravity.BOTTOM)
        lp.setMargins(d(62f), 0, d(8f), d(10f))
        root.addView(optionsHost, lp)
        rebuildOptions()
    }

    private fun buildStatus(root: FrameLayout) {
        statusView = TextView(this).apply {
            setTextColor(0xFF9AA6B8.toInt())
            textSize = 11.5f
            setPadding(d(8f), d(3f), d(8f), d(3f))
            background = barBackground(0xE6)
        }
        val lp = FrameLayout.LayoutParams(-2, -2, Gravity.BOTTOM or Gravity.START)
        lp.setMargins(d(64f), 0, 0, d(10f))
        root.addView(statusView, lp)
    }

    private fun buildSwatch(root: FrameLayout) {
        swatch = SwatchView(this).apply {
            color = editor.color
            setOnClickListener { pickColor() }
        }
        val lp = FrameLayout.LayoutParams(d(46f), d(46f), Gravity.BOTTOM or Gravity.END)
        lp.setMargins(0, 0, d(10f), d(72f))
        root.addView(swatch, lp)
    }

    private fun buildLayersPanel(root: FrameLayout) {
        layersCard = LinearLayout(this)
        layersCard.orientation = LinearLayout.VERTICAL
        layersCard.background = barBackground()
        layersCard.setPadding(d(10f), d(10f), d(10f), d(8f))
        layersCard.visibility = View.GONE

        val header = LinearLayout(this)
        header.orientation = LinearLayout.HORIZONTAL
        header.gravity = Gravity.CENTER_VERTICAL
        val title = TextView(this).apply {
            text = "Layers"
            setTextColor(0xFFE6E9EF.toInt())
            textSize = 14f
        }
        header.addView(title, LinearLayout.LayoutParams(0, -2, 1f))
        val close = IconView(this, "close")
        close.setOnClickListener { toggleLayers(false) }
        header.addView(close, LinearLayout.LayoutParams(d(36f), d(36f)))
        layersCard.addView(header)

        blendSpinner = Spinner(this)
        val modes = NativeCore.blendModes().split('|')
        blendSpinner.adapter = ArrayAdapter(this, android.R.layout.simple_spinner_dropdown_item, modes)
        blendSpinner.onItemSelectedListener = object : android.widget.AdapterView.OnItemSelectedListener {
            override fun onItemSelected(parent: android.widget.AdapterView<*>?, view: View?, position: Int, id: Long) {
                if (updatingPanel) return
                NativeCore.setLayerBlend(handle, editorLayerIndex(), position)
                editor.invalidateDocument()
                refreshAll()
            }

            override fun onNothingSelected(parent: android.widget.AdapterView<*>?) {}
        }
        layersCard.addView(blendSpinner, LinearLayout.LayoutParams(-1, d(44f)))

        layerOpacityLabel = TextView(this).apply {
            setTextColor(0xFFB9C2D0.toInt())
            textSize = 12f
            text = "Opacity 100%"
        }
        layersCard.addView(layerOpacityLabel)
        layerOpacity = SeekBar(this)
        layerOpacity.max = 255
        layerOpacity.progress = 255
        layerOpacity.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                layerOpacityLabel.text = "Opacity ${(progress * 100) / 255}%"
                if (!fromUser || updatingPanel) return
                NativeCore.setLayerOpacity(handle, editorLayerIndex(), progress)
                editor.invalidateDocument()
            }

            override fun onStartTrackingTouch(sb: SeekBar?) {}

            override fun onStopTrackingTouch(sb: SeekBar?) {
                refreshAll()
            }
        })
        layersCard.addView(layerOpacity)

        val scroll = ScrollView(this)
        layersList = LinearLayout(this)
        layersList.orientation = LinearLayout.VERTICAL
        scroll.addView(layersList)
        layersCard.addView(scroll, LinearLayout.LayoutParams(-1, 0, 1f))

        val buttons = LinearLayout(this)
        buttons.orientation = LinearLayout.HORIZONTAL
        fun actionButton(icon: String, tip: String, action: () -> Unit) {
            val b = IconView(this, icon)
            b.contentDescription = tip
            b.setOnClickListener { action() }
            buttons.addView(b, LinearLayout.LayoutParams(d(36f), d(36f)))
        }
        // Index 0 is the bottom of the stack, so "up" on screen is a higher index in the core.
        actionButton("up", "Move layer up") {
            val count = layerCount()
            val i = editorLayerIndex()
            if (count > 0 && i < count - 1) {
                NativeCore.moveLayer(handle, i, i + 1)
                editor.invalidateDocument()
                refreshAll()
            }
        }
        actionButton("down", "Move layer down") {
            val i = editorLayerIndex()
            if (i > 0) {
                NativeCore.moveLayer(handle, i, i - 1)
                editor.invalidateDocument()
                refreshAll()
            }
        }
        actionButton("add", "Add layer") {
            NativeCore.addLayer(handle, "Layer")
            refreshAll()
        }
        actionButton("layers", "Duplicate layer") {
            NativeCore.duplicateLayer(handle, editorLayerIndex())
            refreshAll()
        }
        actionButton("mask", "Mask from selection") {
            NativeCore.selectionToMask(handle)
            refreshAll()
        }
        actionButton("check", "Merge down") {
            NativeCore.mergeDown(handle, editorLayerIndex())
            refreshAll()
        }
        actionButton("close", "Delete layer") {
            NativeCore.deleteLayer(handle, editorLayerIndex())
            refreshAll()
        }
        layersCard.addView(buttons)

        val lp = FrameLayout.LayoutParams(d(280f), -1, Gravity.END)
        lp.setMargins(0, d(64f), d(8f), d(72f))
        root.addView(layersCard, lp)
    }

    // --- state -------------------------------------------------------------------------------

    private fun selectTool(tool: Tool) {
        editor.tool = tool
        for ((t, button) in railButtons) button.active = t == tool
        rebuildOptions()
        editor.invalidate()
    }

    private fun rebuildOptions() {
        optionsHost.removeAllViews()
        fun put(row: View, weight: Float) {
            val lp = LinearLayout.LayoutParams(0, -2, weight)
            lp.rightMargin = d(6f)
            optionsHost.addView(row, lp)
        }

        fun putFixed(row: View) {
            val lp = LinearLayout.LayoutParams(d(40f), d(40f))
            lp.rightMargin = d(4f)
            optionsHost.addView(row, lp)
        }

        when (editor.tool) {
            Tool.BRUSH, Tool.ERASER -> {
                val size = SliderRow(this, "Size", 1f, 300f, editor.brushSize, " px")
                size.bar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                    override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                        editor.brushSize = size.value()
                    }

                    override fun onStartTrackingTouch(sb: SeekBar?) {}
                    override fun onStopTrackingTouch(sb: SeekBar?) {}
                })
                put(size, 1.4f)

                val opacity = SliderRow(this, "Opacity", 0f, 1f, editor.opacity)
                opacity.bar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                    override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                        editor.opacity = opacity.value()
                    }

                    override fun onStartTrackingTouch(sb: SeekBar?) {}
                    override fun onStopTrackingTouch(sb: SeekBar?) {}
                })
                put(opacity, 1f)

                val more = IconView(this, "more")
                more.contentDescription = "Brush settings"
                more.setOnClickListener { brushSettings() }
                putFixed(more)
            }

            Tool.MARQUEE, Tool.OVAL_SELECT, Tool.LASSO -> {
                val feather = SliderRow(this, "Feather", 0f, 60f, editor.feather, " px")
                feather.bar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                    override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                        editor.feather = feather.value()
                    }

                    override fun onStartTrackingTouch(sb: SeekBar?) {}
                    override fun onStopTrackingTouch(sb: SeekBar?) {}
                })
                put(feather, 1f)

                val none = TextView(this).apply {
                    text = "Deselect"
                    setTextColor(0xFF8FBBFF.toInt())
                    textSize = 13f
                    setPadding(d(10f), d(8f), d(10f), d(8f))
                    setOnClickListener {
                        NativeCore.selectNone(handle)
                        editor.invalidateDocument()
                        refreshAll()
                    }
                }
                put(none, 0.5f)
            }

            Tool.BUCKET -> {
                val tolerance = SliderRow(this, "Tolerance", 0f, 255f, editor.tolerance.toFloat())
                tolerance.bar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                    override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                        editor.tolerance = tolerance.value().toInt()
                    }

                    override fun onStartTrackingTouch(sb: SeekBar?) {}
                    override fun onStopTrackingTouch(sb: SeekBar?) {}
                })
                put(tolerance, 1f)
            }

            else -> {
                val hint = TextView(this).apply {
                    text = when (editor.tool) {
                        Tool.MOVE -> "Drag to move the active layer"
                        Tool.TEXT -> "Tap where the text should go"
                        else -> ""
                    }
                    setTextColor(0xFF9AA6B8.toInt())
                    textSize = 12.5f
                    setPadding(d(8f), 0, d(8f), 0)
                }
                put(hint, 1f)
            }
        }
    }

    private fun brushSettings() {
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(16f), d(8f), d(16f), d(8f))
        }
        val hard = SliderRow(this, "Hardness", 0f, 1f, editor.hardness)
        val flow = SliderRow(this, "Flow", 0f, 1f, editor.flow)
        val opacity = SliderRow(this, "Opacity", 0f, 1f, editor.opacity)
        body.addView(hard)
        body.addView(flow)
        body.addView(opacity)
        AlertDialog.Builder(this)
            .setTitle("Brush")
            .setView(body)
            .setPositiveButton("Done") { _, _ ->
                editor.hardness = hard.value()
                editor.flow = flow.value()
                editor.opacity = opacity.value()
                rebuildOptions()
            }
            .setNegativeButton("Cancel", null)
            .show()
    }

    private fun pickColor() {
        val dialog = Dialog(this)
        dialog.requestWindowFeature(Window.FEATURE_NO_TITLE)
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(16f), d(16f), d(16f), d(8f))
            background = barBackground()
        }
        val picker = ColorPickerView(this)
        picker.color = editor.color
        body.addView(picker, LinearLayout.LayoutParams(-1, d(280f)))
        val preview = TextView(this).apply {
            text = " "
            textSize = 15f
            height = d(36f)
            setPadding(d(8f), d(6f), d(8f), d(6f))
            setOnClickListener { }
        }
        picker.onColor = { c ->
            preview.setBackgroundColor(c)
            preview.text = String.format(Locale.US, "#%08X", c)
        }
        preview.setBackgroundColor(editor.color)
        body.addView(preview, LinearLayout.LayoutParams(-1, -2))
        val buttons = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            gravity = Gravity.END
        }
        val cancel = TextView(this).apply {
            text = "Cancel"
            textSize = 14f
            setTextColor(0xFF9AA6B8.toInt())
            setPadding(d(14f), d(12f), d(14f), d(12f))
            setOnClickListener { dialog.dismiss() }
        }
        val ok = TextView(this).apply {
            text = "Use colour"
            textSize = 14f
            setTextColor(0xFF8FBBFF.toInt())
            setPadding(d(14f), d(12f), d(14f), d(12f))
            setOnClickListener {
                editor.color = picker.color
                swatch.color = picker.color
                dialog.dismiss()
            }
        }
        buttons.addView(cancel)
        buttons.addView(ok)
        body.addView(buttons)
        dialog.setContentView(body)
        dialog.window?.setLayout(d(320f), -2)
        dialog.show()
    }

    // --- documents ---------------------------------------------------------------------------

    private fun newDocument(w: Int, h: Int) {
        if (NativeCore.loadError != null) return
        closeCurrent()
        handle = NativeCore.createDocument(w, h)
        if (handle == 0L) {
            toast("The core could not create a document: ${NativeCore.lastError()}")
            return
        }
        docName = "Untitled"
        documentUri = null
        editor.handle = handle
        editor.syncDocument()
        editor.fitToWindow()
        refreshAll()
    }

    private fun closeCurrent() {
        if (handle != 0L) {
            NativeCore.closeDocument(handle)
            handle = 0L
            editor.handle = 0L
        }
    }

    private fun refreshAll() {
        if (handle == 0L) return
        titleView.text = docName
        undoBtn.alpha = if (NativeCore.canUndo(handle)) 1f else 0.35f
        redoBtn.alpha = if (NativeCore.canRedo(handle)) 1f else 0.35f
        val zoom = (editor.currentZoom() * 100f).toInt()
        val sel = if (editor.hasActiveSelection()) " · selection" else ""
        statusView.text = String.format(
            Locale.US, "%d × %d · %d%%%s",
            editor.documentWidth(), editor.documentHeight(), zoom, sel
        )
        if (layersCard.visibility == View.VISIBLE) refreshLayers()
    }

    private fun editorLayerIndex(): Int = NativeCore.activeLayer(handle).coerceAtLeast(0)

    private fun layerCount(): Int =
        NativeCore.layers(handle).trim().split('\n').count { it.isNotBlank() }

    private fun toggleLayers() {
        toggleLayers(layersCard.visibility != View.VISIBLE)
    }

    private fun toggleLayers(show: Boolean) {
        layersCard.visibility = if (show) View.VISIBLE else View.GONE
        layersBtn.active = show
        if (show) refreshLayers()
    }

    private fun refreshLayers() {
        if (handle == 0L) return
        updatingPanel = true
        layersList.removeAllViews()
        val active = NativeCore.activeLayer(handle)
        val lines = NativeCore.layers(handle).trim().split('\n').filter { it.isNotBlank() }
        // Top of the stack first, the way a layer panel reads.
        for (line in lines.reversed()) {
            val f = line.split('|')
            if (f.size < 7) continue
            val index = f[0].toIntOrNull() ?: continue
            val name = f[2]
            val visible = f[3] == "1"
            val opacity = f[4].toIntOrNull() ?: 255
            val hasMask = f[6] == "1"

            val row = LinearLayout(this)
            row.orientation = LinearLayout.HORIZONTAL
            row.gravity = Gravity.CENTER_VERTICAL
            row.setPadding(d(4f), d(3f), d(4f), d(3f))
            if (index == active) {
                val sel = GradientDrawable().apply {
                    cornerRadius = d(8f).toFloat()
                    setColor(0xFF243449.toInt())
                }
                row.background = sel
            }

            val eye = IconView(this, if (visible) "check" else "close")
            eye.alpha = if (visible) 1f else 0.5f
            eye.contentDescription = "Toggle visibility"
            eye.setOnClickListener {
                NativeCore.setLayerVisible(handle, index, !visible)
                editor.invalidateDocument()
                refreshAll()
            }
            row.addView(eye, LinearLayout.LayoutParams(d(34f), d(34f)))

            val thumb = ImageView(this)
            val bmp = Bitmap.createBitmap(d(40f), d(40f), Bitmap.Config.ARGB_8888)
            val buf = IntArray(d(40f) * d(40f))
            NativeCore.layerThumb(handle, index, buf, d(40f), d(40f))
            bmp.setPixels(buf, 0, d(40f), 0, 0, d(40f), d(40f))
            thumb.setImageBitmap(bmp)
            thumb.setBackgroundColor(0xFF33383F.toInt())
            row.addView(thumb, LinearLayout.LayoutParams(d(40f), d(40f)))

            val label = TextView(this).apply {
                text = buildString {
                    append(name)
                    if (opacity < 255) append("  ${opacity * 100 / 255}%")
                    if (hasMask) append("  ·mask")
                }
                setTextColor(0xFFE6E9EF.toInt())
                textSize = 12.5f
                isSingleLine = true
                setPadding(d(8f), 0, d(4f), 0)
            }
            row.addView(label, LinearLayout.LayoutParams(0, -2, 1f))

            row.setOnClickListener {
                NativeCore.selectLayer(handle, index)
                refreshAll()
            }
            layersList.addView(row, LinearLayout.LayoutParams(-1, -2))
        }

        val activeIdx = active.coerceAtLeast(0)
        val line = lines.firstOrNull { it.startsWith("$activeIdx|") }
        if (line != null) {
            val f = line.split('|')
            val op = f.getOrNull(4)?.toIntOrNull() ?: 255
            val blend = f.getOrNull(5)?.toIntOrNull() ?: 0
            layerOpacity.progress = op
            layerOpacityLabel.text = "Opacity ${op * 100 / 255}%"
            if (blend in 0 until blendSpinner.count) blendSpinner.setSelection(blend)
        }
        updatingPanel = false
    }

    // --- menus -------------------------------------------------------------------------------

    private fun showMenu() {
        val dialog = Dialog(this)
        dialog.requestWindowFeature(Window.FEATURE_NO_TITLE)
        val scroll = ScrollView(this)
        val list = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(8f), d(8f), d(8f), d(16f))
            background = barBackground()
        }

        fun header(text: String) {
            val tv = TextView(this).apply {
                this.text = text.uppercase(Locale.US)
                setTextColor(0xFF7C8798.toInt())
                textSize = 10.5f
                letterSpacing = 0.12f
                setPadding(d(12f), d(12f), d(8f), d(4f))
            }
            list.addView(tv)
        }

        fun item(label: String, action: () -> Unit) {
            val tv = TextView(this).apply {
                text = label
                setTextColor(0xFFE6E9EF.toInt())
                textSize = 14.5f
                setPadding(d(12f), d(11f), d(12f), d(11f))
                setOnClickListener {
                    dialog.dismiss()
                    action()
                }
            }
            list.addView(tv, LinearLayout.LayoutParams(-1, -2))
        }

        header("File")
        item("New document…") { newDocumentDialog() }
        item("Open image…") { openImage() }
        item("Open project (.rea)…") { openDocument() }
        item("Save project") { saveDocument(saveAs = documentUri == null) }
        item("Save project as…") { saveDocument(saveAs = true) }
        item("Export PNG…") { exportImage(png = true) }
        item("Export JPEG…") { exportImage(png = false) }

        header("Edit")
        item("Undo ${NativeCore.undoName(handle).ifBlank { "" }}") { doUndo() }
        item("Redo ${NativeCore.redoName(handle).ifBlank { "" }}") { doRedo() }
        item("Clear selected area") {
            NativeCore.clearSelection(handle)
            editor.invalidateDocument()
            refreshAll()
        }
        item("Fill selection with colour") {
            NativeCore.fillSelection(handle, editor.color)
            editor.invalidateDocument()
            refreshAll()
        }

        header("Select")
        item("Select all") { NativeCore.selectAll(handle); editor.invalidateDocument(); refreshAll() }
        item("Deselect") { NativeCore.selectNone(handle); editor.invalidateDocument(); refreshAll() }
        item("Invert selection") { NativeCore.invertSelection(handle); editor.invalidateDocument(); refreshAll() }
        item("Feather selection…") { featherDialog() }
        item("Selection to layer mask") {
            NativeCore.selectionToMask(handle)
            editor.invalidateDocument()
            refreshAll()
        }

        header("Image")
        item("Image size…") { imageSizeDialog() }
        item("Crop to selection") {
            NativeCore.cropToSelection(handle)
            editor.syncDocument()
            editor.fitToWindow()
            refreshAll()
        }
        item("Flip horizontal") {
            NativeCore.flip(handle, true); editor.invalidateDocument(); refreshAll()
        }
        item("Flip vertical") {
            NativeCore.flip(handle, false); editor.invalidateDocument(); refreshAll()
        }
        item("Rotate 90°") {
            NativeCore.rotate90(handle); editor.invalidateDocument(); refreshAll()
        }

        header("Layer")
        item("New layer") { NativeCore.addLayer(handle, "Layer"); refreshAll() }
        item("Duplicate layer") { NativeCore.duplicateLayer(handle, editorLayerIndex()); refreshAll() }
        item("Merge down") { NativeCore.mergeDown(handle, editorLayerIndex()); refreshAll() }
        item("Delete layer") { NativeCore.deleteLayer(handle, editorLayerIndex()); refreshAll() }

        header("Adjust")
        item("Brightness / Contrast…") {
            sliderDialog(
                "Brightness / Contrast",
                listOf(
                    SliderSpec("Brightness", -100f, 100f, 0f),
                    SliderSpec("Contrast", -100f, 100f, 0f)
                )
            ) { v -> NativeCore.adjust(handle, 0, v) }
        }
        item("Levels…") {
            sliderDialog(
                "Levels",
                listOf(
                    SliderSpec("Black point", 0f, 254f, 0f),
                    SliderSpec("White point", 1f, 255f, 255f),
                    SliderSpec("Gamma", 0.1f, 3f, 1f),
                    SliderSpec("Output black", 0f, 255f, 0f),
                    SliderSpec("Output white", 0f, 255f, 255f)
                )
            ) { v -> NativeCore.adjust(handle, 1, v) }
        }
        item("Curves…") {
            sliderDialog(
                "Curves (gamma and lift)",
                listOf(
                    SliderSpec("Gamma", 0.1f, 3f, 1f),
                    SliderSpec("Lift", -100f, 100f, 0f)
                )
            ) { v -> NativeCore.adjust(handle, 2, v) }
        }
        item("Hue / Saturation…") {
            sliderDialog(
                "Hue / Saturation",
                listOf(
                    SliderSpec("Hue", -180f, 180f, 0f),
                    SliderSpec("Saturation", -100f, 100f, 0f),
                    SliderSpec("Lightness", -100f, 100f, 0f)
                )
            ) { v -> NativeCore.adjust(handle, 3, v) }
        }
        item("Threshold…") {
            sliderDialog("Threshold", listOf(SliderSpec("Level", 0f, 255f, 128f))) { v ->
                NativeCore.adjust(handle, 4, v)
            }
        }
        item("Auto levels") { NativeCore.adjust(handle, 7, FloatArray(0)); editor.invalidateDocument(); refreshAll() }
        item("Invert") { NativeCore.adjust(handle, 5, FloatArray(0)); editor.invalidateDocument(); refreshAll() }
        item("Desaturate") { NativeCore.adjust(handle, 6, FloatArray(0)); editor.invalidateDocument(); refreshAll() }
        item("Temperature…") {
            sliderDialog("Temperature", listOf(SliderSpec("Warmth", -100f, 100f, 0f))) { v ->
                NativeCore.adjust(handle, 8, v)
            }
        }

        header("Filter")
        item("Gaussian blur…") {
            sliderDialog("Blur", listOf(SliderSpec("Radius", 1f, 60f, 4f))) { v ->
                NativeCore.filter(handle, 0, v)
            }
        }
        item("Sharpen…") {
            sliderDialog("Sharpen", listOf(SliderSpec("Amount", 0f, 200f, 60f))) { v ->
                NativeCore.filter(handle, 1, v)
            }
        }
        item("Noise…") {
            sliderDialog("Noise", listOf(SliderSpec("Amount", 0f, 100f, 12f))) { v ->
                NativeCore.filter(handle, 2, floatArrayOf(v[0], (System.nanoTime() % 1000).toFloat()))
            }
        }
        item("Pixelate…") {
            sliderDialog("Pixelate", listOf(SliderSpec("Cell size", 2f, 64f, 12f))) { v ->
                NativeCore.filter(handle, 3, v)
            }
        }

        header("View")
        item("Fit to screen") {
            editor.fitToWindow(); editor.invalidateDocument(); refreshAll()
        }
        item("Actual size (100%)") {
            editor.zoomTo(1f); refreshAll()
        }
        item("Diagnostics…") { showDiagnostics() }
        item("About REA Image Studio") { showAbout() }

        scroll.addView(list)
        dialog.setContentView(scroll)
        dialog.window?.let {
            it.setGravity(Gravity.BOTTOM)
            it.setLayout(-1, (resources.displayMetrics.heightPixels * 0.78f).toInt())
        }
        dialog.show()
    }

    private fun showAbout() {
        AlertDialog.Builder(this)
            .setTitle("REA Image Studio")
            .setMessage(
                NativeCore.nativeVersion() + "\n\n" +
                    "An original image editor: layers with masks, brush and eraser, " +
                    "selections, adjustments, filters, undo/redo, and its own .rea document format.\n\n" +
                    "Not yet implemented: PSD import/export, smart objects, non-destructive " +
                    "adjustment layers, editable text layers (text is rasterized), colour management. " +
                    "The text has no undo history after a rasterize.\n\n" +
                    "Rendering is on the CPU: large documents are slower than a GPU editor would be."
            )
            .setPositiveButton("Close", null)
            .show()
    }

    /**
     * Everything the app knows about its own health, on one screen: whether the previous launch
     * died and where, any recorded exception, and the result of running the core's own end-to-end
     * check on this device. It exists so a failure is something the user can read out loud.
     */
    private fun showDiagnostics() {
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(16f), d(8f), d(16f), d(8f))
        }

        fun section(title: String, text: String) {
            if (text.isBlank()) return
            body.addView(TextView(this).apply {
                this.text = title
                setTextColor(0xFF8FBBFF.toInt())
                textSize = 12f
                setPadding(0, d(10f), 0, d(2f))
            })
            body.addView(TextView(this).apply {
                this.text = text.trim()
                setTextColor(0xFFD6DBE3.toInt())
                textSize = 10.5f
                typeface = android.graphics.Typeface.MONOSPACE
                setTextIsSelectable(true)
            })
        }

        section("Previous launch", Diag.previousBootLog())
        section("Recorded exception", Diag.crashText())

        val selfTestOut = TextView(this).apply {
            text = "running…"
            setTextColor(0xFFD6DBE3.toInt())
            textSize = 10.5f
            typeface = android.graphics.Typeface.MONOSPACE
            setTextIsSelectable(true)
        }
        body.addView(TextView(this).apply {
            text = "Core self-test on this device"
            setTextColor(0xFF8FBBFF.toInt())
            textSize = 12f
            setPadding(0, d(10f), 0, d(2f))
        })
        body.addView(selfTestOut)

        if (NativeCore.loadError != null) {
            section("Native library", "load failed: ${NativeCore.loadError}")
        } else {
            // Run it off the UI thread: it writes a file, and a slow device would drop frames.
            Thread {
                val result = runCatching { NativeCore.selfTest(cacheDir.absolutePath) }
                    .getOrElse { "self-test threw: $it" }
                runOnUiThread {
                    selfTestOut.text = result.trim()
                    selfTestOut.setTextColor(
                        if (result.contains("FAIL") || result.contains("threw"))
                            0xFFFF9C8A.toInt() else 0xFF9BE3A0.toInt()
                    )
                }
            }.start()
        }

        section("Diagnostics folder", Diag.directory())

        AlertDialog.Builder(this)
            .setTitle("Diagnostics")
            .setView(ScrollView(this).apply { addView(body) })
            .setPositiveButton("Close", null)
            .setNegativeButton("Clear") { _, _ ->
                Diag.clearReports()
                toast("Reports cleared")
            }
            .setNeutralButton("Copy all") { _, _ ->
                val all = buildString {
                    append("REA Image Studio diagnostics\n")
                    append("device: ").append(android.os.Build.MANUFACTURER).append(' ')
                    append(android.os.Build.MODEL).append(", Android ")
                    append(android.os.Build.VERSION.RELEASE).append(" (API ")
                    append(android.os.Build.VERSION.SDK_INT).append(")\n")
                    append("abis: ").append(android.os.Build.SUPPORTED_ABIS.joinToString()).append('\n')
                    append("native: ").append(NativeCore.loadError ?: "loaded").append('\n')
                    append("\n--- previous launch ---\n").append(Diag.previousBootLog())
                    append("\n--- exception ---\n").append(Diag.crashText())
                    append("\n--- self-test ---\n").append(selfTestOut.text)
                }
                val clipboard = getSystemService(CLIPBOARD_SERVICE) as android.content.ClipboardManager
                clipboard.setPrimaryClip(android.content.ClipData.newPlainText("REA diagnostics", all))
                toast("Diagnostics copied")
            }
            .show()
    }

    /**
     * Exercise the path a finger takes: synthesize the same MotionEvents a touchscreen produces,
     * dispatch them into the canvas, and then read the document's pixels back out of the core.
     *
     * This covers what a host unit test cannot — the view's coordinate mapping, its touch handling,
     * the JNI calls it makes — and it does not depend on the emulator's input routing, which is
     * exactly what kept failing in CI.
     */
    private fun runUiSelfTest() {
        val report = StringBuilder()
        editor.post {
            try {
                newDocument(320, 320)
                val drawn = editor.uiSelfTest()
                report.append(if (drawn) "[ok] touch -> paint -> pixels" else "[FAIL] touch -> paint -> pixels")
                report.append("\n")
                report.append("document: ").append(editor.documentWidth()).append('x')
                report.append(editor.documentHeight()).append("\n")
                report.append("canvas: ").append(editor.width).append('x').append(editor.height).append("\n")
                report.append("zoom: ").append(editor.currentZoom()).append("\n")
                if (NativeCore.loadError == null) {
                    report.append("\n--- core self-test ---\n")
                    report.append(NativeCore.selfTest(cacheDir.absolutePath))
                }
            } catch (t: Throwable) {
                report.append("[FAIL] ui self-test threw: ").append(t).append('\n')
            }
            Diag.writeReport("ui-selftest.txt", report.toString())
            Diag.step("ui self-test finished")
        }
    }

    // --- dialogs -----------------------------------------------------------------------------

    private fun newDocumentDialog() {
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(20f), d(8f), d(20f), d(8f))
        }
        val wField = EditText(this).apply {
            inputType = InputType.TYPE_CLASS_NUMBER
            setText("2048")
            hint = "Width"
        }
        val hField = EditText(this).apply {
            inputType = InputType.TYPE_CLASS_NUMBER
            setText("1536")
            hint = "Height"
        }
        body.addView(TextView(this).apply { text = "Width (px)"; setTextColor(0xFF9AA6B8.toInt()) })
        body.addView(wField)
        body.addView(TextView(this).apply { text = "Height (px)"; setTextColor(0xFF9AA6B8.toInt()) })
        body.addView(hField)
        AlertDialog.Builder(this)
            .setTitle("New document")
            .setView(body)
            .setPositiveButton("Create") { _, _ ->
                val w = wField.text.toString().toIntOrNull() ?: 2048
                val h = hField.text.toString().toIntOrNull() ?: 1536
                newDocument(w.coerceIn(16, 8192), h.coerceIn(16, 8192))
            }
            .setNegativeButton("Cancel", null)
            .show()
    }

    private fun imageSizeDialog() {
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(20f), d(8f), d(20f), d(8f))
        }
        val wField = EditText(this).apply {
            inputType = InputType.TYPE_CLASS_NUMBER
            setText(editor.documentWidth().toString())
        }
        val hField = EditText(this).apply {
            inputType = InputType.TYPE_CLASS_NUMBER
            setText(editor.documentHeight().toString())
        }
        body.addView(TextView(this).apply { text = "Width (px)"; setTextColor(0xFF9AA6B8.toInt()) })
        body.addView(wField)
        body.addView(TextView(this).apply { text = "Height (px)"; setTextColor(0xFF9AA6B8.toInt()) })
        body.addView(hField)
        body.addView(TextView(this).apply {
            text = "Resizing scales every layer and discards layer masks."
            setTextColor(0xFF9AA6B8.toInt())
            textSize = 11.5f
        })
        AlertDialog.Builder(this)
            .setTitle("Image size")
            .setView(body)
            .setPositiveButton("Resize") { _, _ ->
                val w = wField.text.toString().toIntOrNull() ?: return@setPositiveButton
                val h = hField.text.toString().toIntOrNull() ?: return@setPositiveButton
                NativeCore.resizeCanvas(handle, w.coerceIn(16, 8192), h.coerceIn(16, 8192))
                editor.syncDocument()
                editor.fitToWindow()
                refreshAll()
            }
            .setNegativeButton("Cancel", null)
            .show()
    }

    private fun featherDialog() {
        sliderDialog("Feather selection", listOf(SliderSpec("Radius", 0f, 60f, 4f))) { v ->
            NativeCore.featherSelection(handle, v[0])
        }
    }

    private fun sliderDialog(
        title: String,
        specs: List<SliderSpec>,
        apply: (FloatArray) -> Unit
    ) {
        val body = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(d(16f), d(8f), d(16f), d(8f))
        }
        val rows = ArrayList<SliderRow>()
        for (spec in specs) {
            val row = SliderRow(this, spec.label, spec.from, spec.to, spec.initial)
            rows.add(row)
            body.addView(row)
        }
        body.addView(TextView(this).apply {
            text = "Applies to the active layer, inside the selection if there is one."
            setTextColor(0xFF9AA6B8.toInt())
            textSize = 11.5f
            setPadding(0, d(6f), 0, 0)
        })
        AlertDialog.Builder(this)
            .setTitle(title)
            .setView(ScrollView(this).apply { addView(body) })
            .setPositiveButton("Apply") { _, _ ->
                val values = FloatArray(rows.size) { rows[it].value() }
                apply(values)
                editor.invalidateDocument()
                refreshAll()
            }
            .setNegativeButton("Cancel", null)
            .show()
    }

    private fun askForText(x: Float, y: Float) {
        val field = EditText(this).apply {
            hint = "Text"
            setSingleLine(false)
        }
        AlertDialog.Builder(this)
            .setTitle("Add text")
            .setView(field)
            .setPositiveButton("Add") { _, _ ->
                val text = field.text.toString()
                if (text.isNotBlank()) rasterizeText(text, x, y)
            }
            .setNegativeButton("Cancel", null)
            .show()
    }

    /**
     * Text is rasterized by the platform's text engine and added as a pixel layer. That is honest
     * about what it is: it draws real shaped text, and it is not editable afterwards.
     */
    private fun rasterizeText(text: String, x: Float, y: Float) {
        val w = editor.documentWidth()
        val h = editor.documentHeight()
        if (w <= 0 || h <= 0) return
        val bmp = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
        val canvas = Canvas(bmp)
        val paint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = editor.color
            textSize = (editor.brushSize * 3f).coerceAtLeast(24f)
            typeface = android.graphics.Typeface.DEFAULT
        }
        var lineY = y
        for (line in text.split('\n')) {
            canvas.drawText(line, x, lineY, paint)
            lineY += paint.textSize * 1.25f
        }
        val buf = IntArray(w * h)
        bmp.getPixels(buf, 0, w, 0, 0, w, h)
        NativeCore.addLayerFromPixels(handle, "Text", w, h, buf)
        bmp.recycle()
        editor.invalidateDocument()
        refreshAll()
    }

    // --- history -----------------------------------------------------------------------------

    private fun doUndo() {
        if (NativeCore.undo(handle)) {
            editor.syncDocument()
            refreshAll()
        }
    }

    private fun doRedo() {
        if (NativeCore.redo(handle)) {
            editor.syncDocument()
            refreshAll()
        }
    }

    // --- file i/o ----------------------------------------------------------------------------

    private fun openImage() {
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "image/*"
        }
        startActivityForResult(intent, reqOpenImage)
    }

    private fun openDocument() {
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "*/*"
        }
        startActivityForResult(intent, reqOpenDoc)
    }

    private fun saveDocument(saveAs: Boolean) {
        val existing = documentUri
        if (!saveAs && existing != null) {
            writeDocumentTo(existing)
            return
        }
        val intent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "application/octet-stream"
            putExtra(Intent.EXTRA_TITLE, "$docName.rea")
        }
        startActivityForResult(intent, reqSaveDoc)
    }

    private fun exportImage(png: Boolean) {
        val intent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = if (png) "image/png" else "image/jpeg"
            putExtra(Intent.EXTRA_TITLE, "$docName.${if (png) "png" else "jpg"}")
        }
        startActivityForResult(intent, if (png) reqExportPng else reqExportJpeg)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (resultCode != RESULT_OK) return
        val uri = data?.data ?: return
        when (requestCode) {
            reqOpenImage -> loadImage(uri)
            reqOpenDoc -> loadDocument(uri)
            reqSaveDoc -> {
                documentUri = uri
                docName = (uri.lastPathSegment ?: "Untitled").substringBeforeLast('.')
                writeDocumentTo(uri)
            }
            reqExportPng -> exportTo(uri, png = true)
            reqExportJpeg -> exportTo(uri, png = false)
        }
    }

    /**
     * A phone camera produces 12–50 megapixel files. Decoding one at full size would allocate
     * hundreds of megabytes and the process would be killed, so the image is subsampled until it
     * fits both the core's 8192 px limit and a sane pixel budget.
     */
    private fun decodeBounded(uri: Uri): Bitmap? {
        val bounds = BitmapFactory.Options().apply { inJustDecodeBounds = true }
        contentResolver.openInputStream(uri)?.use { BitmapFactory.decodeStream(it, null, bounds) }
        if (bounds.outWidth <= 0 || bounds.outHeight <= 0) return null
        var sample = 1
        while (bounds.outWidth / sample > 8192 ||
            bounds.outHeight / sample > 8192 ||
            (bounds.outWidth.toLong() / sample) * (bounds.outHeight / sample) > 24_000_000L
        ) {
            sample *= 2
        }
        val options = BitmapFactory.Options().apply { inSampleSize = sample }
        return contentResolver.openInputStream(uri)?.use { BitmapFactory.decodeStream(it, null, options) }
    }

    private fun loadImage(uri: Uri) {
        try {
            val bmp = decodeBounded(uri)
            if (bmp == null) {
                toast("That file could not be decoded as an image.")
                return
            }
            val w = bmp.width
            val h = bmp.height
            val buf = IntArray(w * h)
            bmp.getPixels(buf, 0, w, 0, 0, w, h)
            newDocument(w, h)
            NativeCore.addLayerFromPixels(handle, "Image", w, h, buf)
            // The empty background layer is redundant now.
            NativeCore.deleteLayer(handle, 0)
            docName = (uri.lastPathSegment ?: "Image").substringBeforeLast('.').ifBlank { "Image" }
            bmp.recycle()
            editor.syncDocument()
            editor.fitToWindow()
            refreshAll()
        } catch (e: Exception) {
            toast("Could not open the image: ${e.message}")
        }
    }

    private fun loadDocument(uri: Uri) {
        try {
            val tmp = File(cacheDir, "open.rea")
            contentResolver.openInputStream(uri)?.use { input: InputStream ->
                tmp.outputStream().use { input.copyTo(it) }
            }
            val h = NativeCore.openDocument(tmp.path)
            if (h == 0L) {
                toast("Could not open the project: ${NativeCore.lastError()}")
                return
            }
            closeCurrent()
            handle = h
            documentUri = uri
            docName = (uri.lastPathSegment ?: "Untitled").substringBeforeLast('.')
            editor.handle = handle
            editor.syncDocument()
            editor.fitToWindow()
            refreshAll()
        } catch (e: Exception) {
            toast("Could not open the project: ${e.message}")
        }
    }

    private fun writeDocumentTo(uri: Uri) {
        try {
            val tmp = File(cacheDir, "save.rea")
            if (!NativeCore.saveDocument(handle, tmp.path)) {
                toast("Save failed: ${NativeCore.lastError()}")
                return
            }
            contentResolver.openOutputStream(uri)?.use { out ->
                tmp.inputStream().use { it.copyTo(out) }
            }
            documentUri = uri
            refreshAll()
            toast("Saved $docName.rea")
        } catch (e: Exception) {
            toast("Save failed: ${e.message}")
        }
    }

    private fun exportTo(uri: Uri, png: Boolean) {
        try {
            val w = editor.documentWidth()
            val h = editor.documentHeight()
            if (w <= 0 || h <= 0) return
            // Export holds the whole document twice (core pixels plus a bitmap); refuse the sizes
            // that would be killed by the memory limit instead of dying silently.
            if (w.toLong() * h > 30_000_000L) {
                toast("This document is too large to export on this device ($w × $h).")
                return
            }
            val buf = IntArray(w * h)
            if (!NativeCore.exportPixels(handle, buf)) {
                toast("Export failed: ${NativeCore.lastError()}")
                return
            }
            val bmp = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
            bmp.setPixels(buf, 0, w, 0, 0, w, h)
            val fmt = if (png) Bitmap.CompressFormat.PNG else Bitmap.CompressFormat.JPEG
            contentResolver.openOutputStream(uri)?.use { out ->
                bmp.compress(fmt, 95, out)
            }
            bmp.recycle()
            toast("Exported")
        } catch (e: Exception) {
            toast("Export failed: ${e.message}")
        }
    }

    private fun toast(message: String) {
        Toast.makeText(this, message, Toast.LENGTH_LONG).show()
    }

    override fun onDestroy() {
        closeCurrent()
        super.onDestroy()
    }
}
