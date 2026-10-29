<!-- Written by .github/workflows/studio.yml. Do not edit by hand. -->
# Studio app: last build evidence

Commit `f8fbd3f3401415ec358b57901a25cbf677d9816e`, run [40929121174](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40929121174), 2026-10-29T18:18:20Z.

```
== badging ==
package: name='com.rea.studio' versionCode='1' versionName='0.1.0' platformBuildVersionName='15' platformBuildVersionCode='35' compileSdkVersion='35' compileSdkVersionCodename='15'
targetSdkVersion:'35'
application-label:'REA Studio'
launchable-activity: name='com.rea.studio.MainActivity'  label='' icon=''
native-code: 'arm64-v8a'

== archive ==
  2335104  1981-01-01 01:01   classes.dex
     1812  1981-01-01 01:01   classes2.dex
   149140  1981-01-01 01:01   classes3.dex
   569520  1981-01-01 01:01   lib/arm64-v8a/librea_core.so
     1440  1981-01-01 01:01   resources.arsc

== native exports ==
JNI entry points: 57
Java_com_rea_studio_NativeCore_activeLayer
Java_com_rea_studio_NativeCore_addLayer
Java_com_rea_studio_NativeCore_addLayerFromPixels
Java_com_rea_studio_NativeCore_addLayerMask
Java_com_rea_studio_NativeCore_adjust
Java_com_rea_studio_NativeCore_blendModes
Java_com_rea_studio_NativeCore_bucketFill
Java_com_rea_studio_NativeCore_canRedo
Java_com_rea_studio_NativeCore_canUndo
Java_com_rea_studio_NativeCore_clearSelection
Java_com_rea_studio_NativeCore_closeDocument
Java_com_rea_studio_NativeCore_createDocument
Java_com_rea_studio_NativeCore_cropToSelection
Java_com_rea_studio_NativeCore_deleteLayer
Java_com_rea_studio_NativeCore_documentSize
Java_com_rea_studio_NativeCore_duplicateLayer
Java_com_rea_studio_NativeCore_exportPixels
Java_com_rea_studio_NativeCore_featherSelection
Java_com_rea_studio_NativeCore_fillSelection
Java_com_rea_studio_NativeCore_filter
Java_com_rea_studio_NativeCore_flip
Java_com_rea_studio_NativeCore_hasSelection
Java_com_rea_studio_NativeCore_history
Java_com_rea_studio_NativeCore_invertSelection
Java_com_rea_studio_NativeCore_lastError
Java_com_rea_studio_NativeCore_layerThumb
Java_com_rea_studio_NativeCore_layers
Java_com_rea_studio_NativeCore_mergeDown
Java_com_rea_studio_NativeCore_moveLayer
Java_com_rea_studio_NativeCore_nativeVersion
Java_com_rea_studio_NativeCore_nudgeLayer
Java_com_rea_studio_NativeCore_openDocument
Java_com_rea_studio_NativeCore_redo
Java_com_rea_studio_NativeCore_redoName
Java_com_rea_studio_NativeCore_render
Java_com_rea_studio_NativeCore_renderSelection
Java_com_rea_studio_NativeCore_resizeCanvas
Java_com_rea_studio_NativeCore_rotate
Java_com_rea_studio_NativeCore_saveDocument
Java_com_rea_studio_NativeCore_selectAll
Java_com_rea_studio_NativeCore_selectEllipse
Java_com_rea_studio_NativeCore_selectLayer
Java_com_rea_studio_NativeCore_selectNone
Java_com_rea_studio_NativeCore_selectPolygon
Java_com_rea_studio_NativeCore_selectRect
Java_com_rea_studio_NativeCore_selectionBounds
Java_com_rea_studio_NativeCore_selectionToMask
Java_com_rea_studio_NativeCore_selfTest
Java_com_rea_studio_NativeCore_setLayerBlend
Java_com_rea_studio_NativeCore_setLayerName
Java_com_rea_studio_NativeCore_setLayerOpacity
Java_com_rea_studio_NativeCore_setLayerVisible
Java_com_rea_studio_NativeCore_strokeBegin
Java_com_rea_studio_NativeCore_strokeEnd
Java_com_rea_studio_NativeCore_strokeTo
Java_com_rea_studio_NativeCore_undo
Java_com_rea_studio_NativeCore_undoName
library size: 556 KiB

== ELF LOAD alignment (must be 0x4000 for 16 KiB pages) ==
  LOAD           0x000000 0x0000000000000000 0x0000000000000000 0x0215e8 0x0215e8 R   0x4000
  LOAD           0x0215e8 0x00000000000255e8 0x00000000000255e8 0x065138 0x065138 R E 0x4000
  LOAD           0x086720 0x000000000008e720 0x000000000008e720 0x003680 0x0038e0 RW  0x4000
  LOAD           0x089da0 0x0000000000095da0 0x0000000000095da0 0x000ae0 0x0013d0 RW  0x4000

== shared objects required ==
  0x0000000000000001 (NEEDED)       Shared library: [libdl.so]
  0x0000000000000001 (NEEDED)       Shared library: [libm.so]
  0x0000000000000001 (NEEDED)       Shared library: [libc.so]

== how the library is packaged ==
compression: unknown
offset of local header: 2486324
(compression method not recognised; alignment not asserted)
        A: http://schemas.android.com/apk/res/android:extractNativeLibs(0x010104ea)=true
== dex ==
dex files: 3
classes.dex: 2335104 bytes
classes2.dex: 1812 bytes
classes3.dex: 149140 bytes
found com/rea/studio/MainActivity
found com/rea/studio/NativeCore
found com/rea/studio/EditorView
found com/rea/studio/ColorPickerView
found com/rea/studio/SliderRow

APK size: 2 MiB
APK sha256: 72f31643b7a82d12b988d3284532d6f3aaddd44f7a6ac03ef841bf48287cd4ab
```
