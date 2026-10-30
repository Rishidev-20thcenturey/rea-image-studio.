<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [41001170164](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/41001170164), commit `66b24ecc8a49b6b6930ea4ca23e1cd47771d8aa8`, 2026-10-30T04:34:31Z.
x86_64 build of the same code the arm64 APK ships.

```
== device ==
List of devices attached
emulator-5554	device

booted after 1 checks
sdk: 30
abi: x86_64
cmd: Failure calling service window: Broken pipe (32)
cmd: Can't find service: window
size: 1080x1920
page size: 4096
apk: 2823558 bytes
cmd: Can't find service: settings

== install ==
Performing Streamed Install
adb: failed to install /home/runner/work/_temp/rea-studio-x86_64.apk: cmd: Can't find service: package

cmd: Can't find service: package
NOT installed

== launch ==
cmd: Can't find service: activity

== first frame ==
screenshot 01-launch: 2195 bytes
cmd: Can't find service: activity
java.lang.IllegalStateException: android.os.ServiceManager$ServiceNotFoundException: No service published for: input
	at android.hardware.input.InputManager.getInstance(InputManager.java:243)
	at com.android.commands.input.Input.injectKeyEvent(Input.java:355)
	at com.android.commands.input.Input.access$100(Input.java:41)
	at com.android.commands.input.Input$InputKeyEvent.sendKeyEvent(Input.java:203)
	at com.android.commands.input.Input$InputKeyEvent.run(Input.java:190)
	at com.android.commands.input.Input.onRun(Input.java:107)
	at com.android.internal.os.BaseCommand.run(BaseCommand.java:60)
	at com.android.commands.input.Input.main(Input.java:71)
	at com.android.internal.os.RuntimeInit.nativeFinishInit(Native Method)
	at com.android.internal.os.RuntimeInit.main(RuntimeInit.java:399)
Caused by: android.os.ServiceManager$ServiceNotFoundException: No service published for: input
	at android.os.ServiceManager.getServiceOrThrow(ServiceManager.java:153)
	at android.hardware.input.InputManager.getInstance(InputManager.java:241)
	... 9 more
focus: 
the app did not have focus; dismissing dialogs and restarting it
java.lang.IllegalStateException: android.os.ServiceManager$ServiceNotFoundException: No service published for: input
	at android.hardware.input.InputManager.getInstance(InputManager.java:243)
	at com.android.commands.input.Input.injectKeyEvent(Input.java:355)
	at com.android.commands.input.Input.access$100(Input.java:41)
	at com.android.commands.input.Input$InputKeyEvent.sendKeyEvent(Input.java:203)
	at com.android.commands.input.Input$InputKeyEvent.run(Input.java:190)
	at com.android.commands.input.Input.onRun(Input.java:107)
	at com.android.internal.os.BaseCommand.run(BaseCommand.java:60)
	at com.android.commands.input.Input.main(Input.java:71)
	at com.android.internal.os.RuntimeInit.nativeFinishInit(Native Method)
	at com.android.internal.os.RuntimeInit.main(RuntimeInit.java:399)
Caused by: android.os.ServiceManager$ServiceNotFoundException: No service published for: input
	at android.os.ServiceManager.getServiceOrThrow(ServiceManager.java:153)
	at android.hardware.input.InputManager.getInstance(InputManager.java:241)
	... 9 more
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Error type 3
Error: Activity class {com.rea.studio/com.rea.studio.MainActivity} does not exist.
focus now:   mCurrentFocus=null

== draw three strokes ==
screenshot 02-painted: 2195 bytes

== switch to the eraser from the rail, then undo ==
screenshot 03-tool: 5248 bytes

== open the layer panel ==
screenshot 04-layers: 45334 bytes

== open the menu ==
screenshot 05-menu: 70300 bytes

== the interface's own end-to-end check ==
Starting: Intent { cmp=com.rea.studio/.MainActivity (has extras) }
Error type 3
Error: Activity class {com.rea.studio/com.rea.studio.MainActivity} does not exist.
run-as: unknown package: com.rea.studio
screenshot 06-selftest: 70360 bytes

== the app's own boot log ===
run-as: unknown package: com.rea.studio
run-as: unknown package: com.rea.studio

== process ==
the app process is NOT running

== activity state ==
    mResumedActivity: ActivityRecord{3ae28a9 u0 com.google.android.apps.nexuslauncher/.NexusLauncherActivity t6}

== crash, ANR and JNI errors in logcat ==
10-30 04:20:08.717  2509  2509 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:20:08.944  2509  2509 I AndroidRuntime: Using default boot image
10-30 04:20:08.959  2509  2509 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:20:17.867  2509  2509 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:20:18.399  2509  2509 I AndroidRuntime: VM exiting with result code 1.
10-30 04:20:25.291  2551  2551 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:20:25.581  2551  2551 I AndroidRuntime: Using default boot image
10-30 04:20:25.588  2551  2551 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:20:31.747  2551  2551 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:20:31.939  2551  2551 I AndroidRuntime: VM exiting with result code 1.
10-30 04:20:36.161  2395  2414 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-30 04:20:49.791  2665  2665 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:20:49.889  2665  2665 I AndroidRuntime: Using default boot image
10-30 04:20:49.904  2665  2665 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:20:54.113  2665  2665 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:20:54.750  2665  2665 D AndroidRuntime: Shutting down VM
10-30 04:21:01.695  2789  2789 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:21:02.369  2814  2814 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:21:04.488  2744  2744 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:21:04.743  2744  2744 I AndroidRuntime: Using default boot image
10-30 04:21:04.744  2744  2744 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:21:10.980  2744  2744 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:21:11.643  2744  2744 D AndroidRuntime: Shutting down VM
10-30 04:21:23.435  2997  2997 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:21:23.553  2997  2997 I AndroidRuntime: Using default boot image
10-30 04:21:23.562  2997  2997 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:21:26.467  2997  2997 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:21:27.133  2997  2997 D AndroidRuntime: Shutting down VM
10-30 04:21:36.869  3210  3210 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:21:36.977  3210  3210 I AndroidRuntime: Using default boot image
10-30 04:21:36.977  3210  3210 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:21:40.212  2395  3047 E ActivityManager: ANR in com.android.systemui
10-30 04:21:47.430  3210  3210 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:21:47.852  3210  3210 D AndroidRuntime: Shutting down VM
10-30 04:21:49.156  2707  2707 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:21:49.163  2707  2707 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:21:49.327  2707  2707 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:21:49.333  2707  2707 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:21:49.360  2707  2707 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:21:49.412  2707  2707 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:21:49.439  2707  2707 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:21:49.450  2707  2707 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:21:49.472  2707  2707 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:21:49.489  2707  2707 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:21:49.510  2707  2707 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:21:49.513  2707  2707 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:21:49.554  2707  2707 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:21:49.557  2707  2707 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:21:49.572  2707  2707 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:21:49.615  2707  2707 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:21:49.634  2707  2707 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:21:49.656  2707  2707 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:22:05.122  3385  3385 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:22:05.513  3385  3385 I AndroidRuntime: Using default boot image
10-30 04:22:05.536  3385  3385 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:22:15.595  3385  3385 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:22:16.318  3385  3385 D AndroidRuntime: Shutting down VM
10-30 04:22:34.127  3613  3613 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:22:34.652  3613  3613 I AndroidRuntime: Using default boot image
10-30 04:22:34.655  3613  3613 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:22:44.227  3613  3613 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:22:45.612  3613  3613 D AndroidRuntime: Shutting down VM
10-30 04:23:00.136  3810  3810 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:23:00.252  3810  3810 I AndroidRuntime: Using default boot image
10-30 04:23:00.253  3810  3810 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:23:11.195  3810  3810 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:23:14.779  3810  3810 D AndroidRuntime: Shutting down VM
10-30 04:23:28.388  2395  4176 E ActivityManager: ANR in com.google.android.gms.persistent
10-30 04:23:30.581  4162  4162 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:23:30.648  4162  4162 I AndroidRuntime: Using default boot image
10-30 04:23:30.649  4162  4162 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:23:49.395  4162  4162 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:23:50.355  4162  4162 D AndroidRuntime: Shutting down VM
10-30 04:24:11.648  2395  4599 E ActivityManager: ANR in com.google.android.apps.youtube.music
10-30 04:24:29.413  2395  4599 E ActivityManager: ANR in com.google.android.youtube
10-30 04:24:46.452  2395  4599 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-30 04:24:51.747  4871  4871 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:24:52.155  4871  4871 I AndroidRuntime: Using default boot image
10-30 04:24:52.158  4871  4871 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:25:10.928  4871  4871 D AndroidRuntime: Calling main entry com.android.commands.input.Input

== result ==
run-as: unknown package: com.rea.studio
RESULT: no boot-complete marker - the app did not finish starting
evidence: /home/runner/work/_temp/smoke.txt
```

## Device log (filtered)

```
10-30 04:30:10.292  6216  6237 E libc    : Access denied finding property "service.adb.tls.port"
10-30 04:30:11.849   277   277 I tombstoned: found intercept fd 512 for pid 6264 and type kDebuggerdJavaBacktrace
10-30 04:30:11.905  2395  6418 I system_server: libdebuggerd_client: done dumping process 6264
10-30 04:30:13.813  2395  6418 I system_server: libdebuggerd_client: started dumping process 6113
10-30 04:30:13.852   277   277 I tombstoned: registered intercept for pid 6113 and type kDebuggerdJavaBacktrace
10-30 04:30:14.643   277   277 I tombstoned: found intercept fd 512 for pid 6113 and type kDebuggerdJavaBacktrace
10-30 04:30:14.661  2395  6418 I system_server: libdebuggerd_client: done dumping process 6113
10-30 04:30:21.074  6453  6453 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:30:21.131  6453  6453 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:30:21.188  6433  6433 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:30:21.232  6433  6433 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:30:26.964  6512  6512 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:30:27.000  6512  6512 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:30:35.484  6453  6539 E GEL_DELAYED_EVENT_DEBUG: Failed delayed event dispatch, no dispatchers.
10-30 04:30:41.263  6216  6290 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 04:30:41.264  6216  6290 E libc    : Access denied finding property "service.adb.tls.port"
10-30 04:30:54.055  6512  6551 D StrictMode: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:249)
10-30 04:30:54.055  6512  6551 D StrictMode: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:166)
10-30 04:30:54.055  6512  6551 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:478)
10-30 04:30:54.086  6512  6551 D StrictMode: 	at libcore.io.BlockGuardOs.fstat(BlockGuardOs.java:174)
10-30 04:30:54.086  6512  6551 D StrictMode: 	at libcore.io.ForwardingOs.fstat(ForwardingOs.java:106)
10-30 04:30:54.086  6512  6551 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:481)
10-30 04:30:54.122  6512  6551 D StrictMode: 	at libcore.io.BlockGuardOs.read(BlockGuardOs.java:304)
10-30 04:30:54.122  6512  6551 D StrictMode: 	at libcore.io.ForwardingOs.read(ForwardingOs.java:176)
10-30 04:30:54.122  6512  6551 D StrictMode: 	at libcore.io.IoBridge.read(IoBridge.java:509)
10-30 04:31:19.059  3419  6696 I PeopleInitIntentOp: Not initializing debuggability
10-30 04:31:33.808  2395  6758 I system_server: libdebuggerd_client: started dumping process 6433
10-30 04:31:33.840   277   277 I tombstoned: registered intercept for pid 6433 and type kDebuggerdJavaBacktrace
10-30 04:31:34.558   277   277 I tombstoned: found intercept fd 512 for pid 6433 and type kDebuggerdJavaBacktrace
10-30 04:31:34.655  2395  6758 I system_server: libdebuggerd_client: done dumping process 6433
10-30 04:31:41.832  6773  6773 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:31:41.901  6773  6773 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:31:48.599  6815  6815 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:31:48.613  6815  6815 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:32:04.968  6815  6859 D StrictMode: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:249)
10-30 04:32:04.968  6815  6859 D StrictMode: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:166)
10-30 04:32:04.968  6815  6859 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:478)
10-30 04:32:04.993  6815  6859 D StrictMode: 	at libcore.io.BlockGuardOs.fstat(BlockGuardOs.java:174)
10-30 04:32:04.993  6815  6859 D StrictMode: 	at libcore.io.ForwardingOs.fstat(ForwardingOs.java:106)
10-30 04:32:04.993  6815  6859 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:481)
10-30 04:32:05.073  6815  6859 D StrictMode: 	at libcore.io.BlockGuardOs.read(BlockGuardOs.java:304)
10-30 04:32:05.073  6815  6859 D StrictMode: 	at libcore.io.ForwardingOs.read(ForwardingOs.java:176)
10-30 04:32:05.073  6815  6859 D StrictMode: 	at libcore.io.IoBridge.read(IoBridge.java:509)
10-30 04:32:27.704  2395  6932 I system_server: libdebuggerd_client: started dumping process 6773
10-30 04:32:27.801   277   277 I tombstoned: registered intercept for pid 6773 and type kDebuggerdJavaBacktrace
10-30 04:32:28.667   277   277 I tombstoned: found intercept fd 512 for pid 6773 and type kDebuggerdJavaBacktrace
10-30 04:32:28.723  2395  6932 I system_server: libdebuggerd_client: done dumping process 6773
10-30 04:32:34.728  6946  6946 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:32:34.825  6946  6946 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:32:35.367  6453  6611 I FA      : To enable debug logging run: adb shell setprop log.tag.FA VERBOSE
10-30 04:32:35.368  6453  6611 I FA      : To enable faster debug mode event logging run:
10-30 04:32:35.368  6453  6611 I FA      :   adb shell setprop debug.firebase.analytics.app com.google.android.youtube
10-30 04:32:55.664  5021  7019 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:32:56.316  5021  7019 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:33:05.349  2395  7071 I system_server: libdebuggerd_client: started dumping process 6946
10-30 04:33:05.397   277   277 I tombstoned: registered intercept for pid 6946 and type kDebuggerdJavaBacktrace
10-30 04:33:07.054   277   277 I tombstoned: found intercept fd 512 for pid 6946 and type kDebuggerdJavaBacktrace
10-30 04:33:07.184  2395  7071 I system_server: libdebuggerd_client: done dumping process 6946
10-30 04:33:13.115  7116  7116 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:33:13.131  7116  7116 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:33:17.352  7150  7150 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:33:17.357  7150  7150 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:33:19.985  6216  7064 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 04:33:20.018  6216  7064 E libc    : Access denied finding property "service.adb.tls.port"
10-30 04:33:42.749  2395  7265 I system_server: libdebuggerd_client: started dumping process 7116
10-30 04:33:42.759   277   277 I tombstoned: registered intercept for pid 7116 and type kDebuggerdJavaBacktrace
10-30 04:33:43.999   277   277 I tombstoned: found intercept fd 512 for pid 7116 and type kDebuggerdJavaBacktrace
10-30 04:33:44.158  2395  7265 I system_server: libdebuggerd_client: done dumping process 7116
10-30 04:33:51.160  7278  7278 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:33:51.188  7278  7278 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:34:00.588  7150  7318 I ChmraDebugLogger: [39] null
10-30 04:34:04.361  5021  7311 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:34:04.927  5021  7311 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 04:34:09.389  7150  7317 I PeopleInitIntentOp: Not initializing debuggability
10-30 04:34:18.971  2395  7386 I system_server: libdebuggerd_client: started dumping process 7278
10-30 04:34:19.028   277   277 I tombstoned: registered intercept for pid 7278 and type kDebuggerdJavaBacktrace
10-30 04:34:19.931   277   277 I tombstoned: found intercept fd 512 for pid 7278 and type kDebuggerdJavaBacktrace
10-30 04:34:19.983  2395  7386 I system_server: libdebuggerd_client: done dumping process 7278
10-30 04:34:22.203  6216  6237 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 04:34:22.205  6216  6237 E libc    : Access denied finding property "service.adb.tls.port"
```

![01-launch](01-launch.png)

![02-painted](02-painted.png)

![04-layers](04-layers.png)
