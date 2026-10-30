<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [41003761840](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/41003761840), commit `8538b38ce84c7a8ffaeb610a4cee65d795737d77`, 2026-10-30T05:04:54Z.
x86_64 build of the same code the arm64 APK ships.

```
== device ==
List of devices attached
emulator-5554	device

system services ready after 1 checks
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

first install attempt failed; waiting and retrying once
system services ready after 7 checks
Performing Streamed Install
adb: failed to install /home/runner/work/_temp/rea-studio-x86_64.apk: 
Exception occurred while executing 'install':
java.lang.NullPointerException: Attempt to invoke virtual method 'void android.content.pm.PackageManagerInternal.freeStorage(java.lang.String, long, int)' on a null object reference
	at com.android.server.StorageManagerService.allocateBytes(StorageManagerService.java:3901)
	at android.os.storage.StorageManager.allocateBytes(StorageManager.java:2278)
	at android.os.storage.StorageManager.allocateBytes(StorageManager.java:2335)
	at com.android.server.pm.PackageInstallerSession.doWriteInternal(PackageInstallerSession.java:987)
	at com.android.server.pm.PackageInstallerSession.write(PackageInstallerSession.java:940)
	at android.content.pm.PackageInstaller$Session.write(PackageInstaller.java:1041)
	at com.android.server.pm.PackageManagerShellCommand.doWriteSplit(PackageManagerShellCommand.java:3269)
	at com.android.server.pm.PackageManagerShellCommand.doWriteSplits(PackageManagerShellCommand.java:3231)
	at com.android.server.pm.PackageManagerShellCommand.doRunInstall(PackageManagerShellCommand.java:1296)
	at com.android.server.pm.PackageManagerShellCommand.runInstall(PackageManagerShellCommand.java:1248)
	at com.android.server.pm.PackageManagerShellCommand.onCommand(PackageManagerShellCommand.java:184)
	at android.os.BasicShellCommandHandler.exec(BasicShellCommandHandler.java:98)
	at android.os.ShellCommand.exec(ShellCommand.java:44)
	at com.android.server.pm.PackageManagerService.onShellCommand(PackageManagerService.java:21646)
	at android.os.Binder.shellCommand(Binder.java:929)
	at android.os.Binder.onTransact(Binder.java:813)
	at android.content.pm.IPackageManager$Stub.onTransact(IPackageManager.java:4603)
	at com.android.server.pm.PackageManagerService.onTransact(PackageManagerService.java:4305)
	at android.os.Binder.execTransactInternal(Binder.java:1159)
	at android.os.Binder.execTransact(Binder.java:1123)

NOT installed

== launch ==
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Error type 3
Error: Activity class {com.rea.studio/com.rea.studio.MainActivity} does not exist.

== first frame ==
screenshot 01-launch: 2195 bytes
Broadcasting: Intent { act=android.intent.action.CLOSE_SYSTEM_DIALOGS flg=0x400000 }
Broadcast completed: result=0
focus:   mCurrentFocus=Window{a3acd74 u0 com.android.settings/com.android.settings.FallbackHome}
the app did not have focus; dismissing dialogs and restarting it
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Error type 3
Error: Activity class {com.rea.studio/com.rea.studio.MainActivity} does not exist.
focus now:   mCurrentFocus=Window{5aa9f88 u0 Application Not Responding: com.android.systemui}

== draw three strokes ==
screenshot 02-painted: 69780 bytes

== switch to the eraser from the rail, then undo ==
screenshot 03-tool: 69781 bytes

== open the layer panel ==
screenshot 04-layers: 69944 bytes

== open the menu ==
screenshot 05-menu: 69812 bytes

== the interface's own end-to-end check ==
Starting: Intent { cmp=com.rea.studio/.MainActivity (has extras) }
Error type 3
Error: Activity class {com.rea.studio/com.rea.studio.MainActivity} does not exist.
run-as: unknown package: com.rea.studio
screenshot 06-selftest: 69729 bytes

== the app's own boot log ===
run-as: unknown package: com.rea.studio
run-as: unknown package: com.rea.studio

== process ==
the app process is NOT running

== activity state ==
    mResumedActivity: ActivityRecord{3a063f1 u0 com.google.android.apps.nexuslauncher/.NexusLauncherActivity t6}

== crash, ANR and JNI errors in logcat ==
10-30 04:46:54.346  2355  2370 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-30 04:47:00.139  2639  2639 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:47:00.426  2620  2620 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:47:23.956  2543  2543 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:47:34.963  2995  2995 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:34.993  2995  2995 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:35.200  2995  2995 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:35.222  2995  2995 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:35.235  2995  2995 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:35.261  2995  2995 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:35.299  2995  2995 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:35.327  2995  2995 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:35.407  2995  2995 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:35.464  2995  2995 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:35.507  2995  2995 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:35.552  2995  2995 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:35.627  2995  2995 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:35.661  2995  2995 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:35.693  2995  2995 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:35.746  2995  2995 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:35.760  2995  2995 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:35.817  2995  2995 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:40.738  3038  3038 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:47:41.186  3038  3038 I AndroidRuntime: Using default boot image
10-30 04:47:41.192  3038  3038 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:47:42.817  2995  2995 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:47:48.450  3038  3038 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:47:48.681  3038  3038 D AndroidRuntime: Shutting down VM
10-30 04:47:50.955  3115  3115 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:50.985  3115  3115 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:51.055  3115  3115 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:51.156  3115  3115 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:51.231  3115  3115 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:51.251  3115  3115 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:51.280  3115  3115 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:51.299  3115  3115 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:51.324  3115  3115 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:51.393  3115  3115 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:51.419  3115  3115 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:51.432  3115  3115 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:51.479  3115  3115 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:51.503  3115  3115 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:51.539  3115  3115 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:51.577  3115  3115 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:47:51.651  3115  3115 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:47:51.700  3115  3115 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:47:53.828  2355  2951 E ActivityManager: ANR in com.android.systemui
10-30 04:48:02.366  3115  3115 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:48:12.698  3259  3259 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:48:13.177  3259  3259 I AndroidRuntime: Using default boot image
10-30 04:48:13.224  3259  3259 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:48:26.796  3259  3259 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:48:31.867  2355  3465 E ActivityManager: ANR in com.google.android.inputmethod.latin
10-30 04:48:40.684  3259  3259 D AndroidRuntime: Shutting down VM
10-30 04:48:41.456  3303  3303 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:48:49.232  2355  3465 E ActivityManager: ANR in com.android.settings (com.android.settings/.FallbackHome)
10-30 04:48:50.508  2355  3465 E ActivityManager: ANR in com.google.android.gms.persistent
10-30 04:48:50.799  2355  3637 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-30 04:48:57.103  2355  3465 E ActivityManager: ANR in com.google.android.apps.messaging:rcs
10-30 04:49:16.924  3737  3737 E a2dp_vendor_aptx_encoder: A2DP_VendorLoadEncoderAptx: cannot open aptX encoder library libaptX_encoder.so: dlopen failed: library "libaptX_encoder.so" not found
10-30 04:49:17.019  3737  3737 E a2dp_vendor_aptx_hd_encoder: A2DP_VendorLoadEncoderAptxHd: cannot open aptX-HD encoder library libaptXHD_encoder.so: dlopen failed: library "libaptXHD_encoder.so" not found
10-30 04:49:17.622  3737  3737 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:49:18.746  3737  3737 E a2dp_vendor_ldac_decoder: A2DP_VendorLoadDecoderLdac: cannot open LDAC decoder library libldacBT_bco.so: dlopen failed: library "libldacBT_bco.so" not found
10-30 04:49:23.278  4028  4028 I AndroidRuntime: Using default boot image
10-30 04:49:23.279  4028  4028 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:49:32.270  3737  3737 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:49:45.687  4028  4028 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:49:46.663  4028  4028 D AndroidRuntime: Shutting down VM
10-30 04:49:50.441  2355  3465 E ActivityManager: ANR in system
10-30 04:49:55.088  2355  3465 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-30 04:49:57.880  2355  3465 E ActivityManager: ANR in com.google.android.gms
10-30 04:50:02.190  4334  4334 I AndroidRuntime: VM exiting with result code 0, cleanup skipped.
10-30 04:50:04.617  2355  4556 E ActivityManager: ANR in com.android.phone
10-30 04:50:04.838  4499  4499 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-30 04:50:05.117  4499  4499 I AndroidRuntime: Using default boot image
10-30 04:50:05.118  4499  4499 I AndroidRuntime: Leaving lock profiling enabled
10-30 04:50:11.357  4499  4499 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-30 04:50:12.020  4499  4499 D AndroidRuntime: Shutting down VM
10-30 04:50:15.965  2355  4556 E ActivityManager: ANR in com.google.android.cellbroadcastreceiver
10-30 04:50:26.207  2355  4747 E ActivityManager: ANR in com.google.android.ims

== result ==
run-as: unknown package: com.rea.studio
RESULT: no boot-complete marker - the app did not finish starting
evidence: /home/runner/work/_temp/smoke.txt
```

## Device log (filtered)

```
10-30 05:00:29.301  2355  8645 I system_server: libdebuggerd_client: done dumping process 8535
10-30 05:00:32.216  2355  2753 W ActivityManager: Scheduling restart of crashed service com.google.android.gms/.clearcut.debug.ClearcutDebugDumpService in 337780ms for start-requested
10-30 05:00:34.147  8659  8659 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:00:34.176  8659  8659 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:00:39.847  8368  8602 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:00:39.849  8368  8602 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:00:43.121  2355  2390 W ActivityManager: Scheduling restart of crashed service com.google.android.gms/.clearcut.debug.ClearcutDebugDumpService in 1351120ms for start-requested
10-30 05:00:45.339  8704  8704 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:00:45.373  8704  8704 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:00:59.899  8368  8494 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:00:59.900  8368  8494 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:01:17.999  8368  8494 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:01:18.000  8368  8494 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:01:26.568  2355  8890 I system_server: libdebuggerd_client: started dumping process 8816
10-30 05:01:26.664   276   276 I tombstoned: registered intercept for pid 8816 and type kDebuggerdJavaBacktrace
10-30 05:01:27.334   276   276 I tombstoned: found intercept fd 512 for pid 8816 and type kDebuggerdJavaBacktrace
10-30 05:01:27.368  2355  8890 I system_server: libdebuggerd_client: done dumping process 8816
10-30 05:01:34.849  2355  8939 I system_server: libdebuggerd_client: started dumping process 8704
10-30 05:01:34.862   276   276 I tombstoned: registered intercept for pid 8704 and type kDebuggerdJavaBacktrace
10-30 05:01:39.382   276   276 I tombstoned: found intercept fd 512 for pid 8704 and type kDebuggerdJavaBacktrace
10-30 05:01:39.439  2355  8939 I system_server: libdebuggerd_client: done dumping process 8704
10-30 05:01:42.890  2355  4287 W ActivityManager: Scheduling restart of crashed service com.google.android.gms/.clearcut.debug.ClearcutDebugDumpService in 5404480ms for start-requested
10-30 05:01:44.994  8974  8974 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:01:45.094  8974  8974 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:01:59.995  8184  8221 I BugleRcsEngine: [127] xly.c: Checking if using CarrierServices.apk is possible. Enabled: true, isAtLeastM: true, runningInsideBugle: true
10-30 05:02:02.234  8184  8221 E BugleRcsEngine: 	at libcore.io.IoBridge.open(IoBridge.java:492)
10-30 05:02:02.234  8184  8221 E BugleRcsEngine: 	at libcore.io.Linux.open(Native Method)
10-30 05:02:02.234  8184  8221 E BugleRcsEngine: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:166)
10-30 05:02:02.234  8184  8221 E BugleRcsEngine: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:254)
10-30 05:02:02.234  8184  8221 E BugleRcsEngine: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:166)
10-30 05:02:02.234  8184  8221 E BugleRcsEngine: 	at libcore.io.IoBridge.open(IoBridge.java:478)
10-30 05:02:07.334  8184  9077 I BugleRcsEngine: [190] xly.c: Checking if using CarrierServices.apk is possible. Enabled: true, isAtLeastM: true, runningInsideBugle: true
10-30 05:02:20.445  8368  9010 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:02:20.455  8368  9010 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:02:20.973  2355  9130 I system_server: libdebuggerd_client: started dumping process 8974
10-30 05:02:20.979   276   276 I tombstoned: registered intercept for pid 8974 and type kDebuggerdJavaBacktrace
10-30 05:02:21.940   276   276 I tombstoned: found intercept fd 512 for pid 8974 and type kDebuggerdJavaBacktrace
10-30 05:02:22.019  2355  9130 I system_server: libdebuggerd_client: done dumping process 8974
10-30 05:02:23.975  6083  9120 I PeopleInitIntentOp: Not initializing debuggability
10-30 05:02:24.686  2355  4557 W ActivityManager: Scheduling restart of crashed service com.google.android.gms/.clearcut.debug.ClearcutDebugDumpService in 21617920ms for start-requested
10-30 05:02:24.771  9138  9138 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:02:24.786  9138  9138 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:02:49.973  8368  8494 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:02:50.039  8368  8494 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:03:23.323  8790  8924 I FA      : To enable debug logging run: adb shell setprop log.tag.FA VERBOSE
10-30 05:03:23.323  8790  8924 I FA      : To enable faster debug mode event logging run:
10-30 05:03:23.323  8790  8924 I FA      :   adb shell setprop debug.firebase.analytics.app com.google.android.apps.messaging
10-30 05:03:52.565  8368  8494 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:03:52.593  8368  8494 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:04:07.107  2355  2482 W FileUtils: 	at libcore.io.Linux.chmod(Native Method)
10-30 05:04:07.107  2355  2482 W FileUtils: 	at libcore.io.ForwardingOs.chmod(ForwardingOs.java:85)
10-30 05:04:07.107  2355  2482 W FileUtils: 	at libcore.io.BlockGuardOs.chmod(BlockGuardOs.java:80)
10-30 05:04:11.021  6385  9418 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:04:11.237  6385  9418 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-30 05:04:11.340  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.access(BlockGuardOs.java:71)
10-30 05:04:11.340  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.access(ForwardingOs.java:72)
10-30 05:04:11.400  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.access(BlockGuardOs.java:71)
10-30 05:04:11.400  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.access(ForwardingOs.java:72)
10-30 05:04:11.473  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.access(BlockGuardOs.java:71)
10-30 05:04:11.473  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.access(ForwardingOs.java:72)
10-30 05:04:11.476  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:249)
10-30 05:04:11.476  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:166)
10-30 05:04:11.476  4386  4386 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:478)
10-30 05:04:11.477  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:252)
10-30 05:04:11.477  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:166)
10-30 05:04:11.477  4386  4386 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:478)
10-30 05:04:11.529  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.fstat(BlockGuardOs.java:174)
10-30 05:04:11.529  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.fstat(ForwardingOs.java:106)
10-30 05:04:11.529  4386  4386 D StrictMode: 	at libcore.io.IoBridge.open(IoBridge.java:481)
10-30 05:04:11.531  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.write(BlockGuardOs.java:417)
10-30 05:04:11.531  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.write(ForwardingOs.java:240)
10-30 05:04:11.531  4386  4386 D StrictMode: 	at libcore.io.IoBridge.write(IoBridge.java:535)
10-30 05:04:11.532  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.chmod(BlockGuardOs.java:78)
10-30 05:04:11.532  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.chmod(ForwardingOs.java:85)
10-30 05:04:11.549  4386  4386 D StrictMode: 	at libcore.io.BlockGuardOs.stat(BlockGuardOs.java:389)
10-30 05:04:11.549  4386  4386 D StrictMode: 	at libcore.io.ForwardingOs.stat(ForwardingOs.java:221)
10-30 05:04:16.613  8368  8385 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:04:16.620  8368  8385 E libc    : Access denied finding property "service.adb.tls.port"
10-30 05:04:35.018  8368  8385 E libc    : Access denied finding property "persist.adb.tls_server.enable"
10-30 05:04:35.020  8368  8385 E libc    : Access denied finding property "service.adb.tls.port"
```

![01-launch](01-launch.png)

![02-painted](02-painted.png)

![04-layers](04-layers.png)
