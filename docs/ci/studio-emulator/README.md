<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [40911856328](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40911856328), commit `bee6087bfcc0df05d60dbd031b09674346b22989`, 2026-10-29T16:34:45Z.
x86_64 build of the same code the arm64 APK ships.

```
(the smoke test did not reach the install step)
```

## Device log (filtered)

```
10-29 16:33:25.092  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:27.327  2606  2642 W libc    : Unable to set property "qemu.gles" to "1": error code: 0x18
10-29 16:33:27.853  2148  2632 I NativeLibraryUtils: Extraction to /data/user_de/0/com.google.android.gms/app_extracted_libs/x86/libconscrypt_gmscore_jni.so true
10-29 16:33:33.072  2428  2474 W libc    : Unable to set property "qemu.gles" to "1": error code: 0x18
10-29 16:33:35.761  2148  2632 I NativeLibraryUtils: Extraction to /data/user_de/0/com.google.android.gms/app_extracted_libs/x86_64/libconscrypt_gmscore_jni.so true
10-29 16:33:38.888  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:40.790  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:42.474  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:44.110  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:45.297  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:47.056  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:47.712  2747  2747 I GAv4    : Google Analytics 11.0.11 is starting up. To enable debug logging on a device run:
10-29 16:33:47.712  2747  2747 I GAv4    :   adb shell setprop log.tag.GAv4 DEBUG
10-29 16:33:48.433  1955  2092 E SupplicantStaIfaceHal: ISupplicantStaIface.setPowerSave failed: {.code = FAILURE_UNKNOWN, .debugMessage = }
10-29 16:33:52.456  1955  2092 E SupplicantStaIfaceHal: ISupplicantStaIface.setPowerSave failed: {.code = FAILURE_UNKNOWN, .debugMessage = }
10-29 16:33:53.575  1955  1984 I system_server: libdebuggerd_client: started dumping process 2659
10-29 16:33:53.718  1786  1786 I /system/bin/tombstoned: registered intercept for pid 2659 and type kDebuggerdJavaBacktrace
10-29 16:33:55.772  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:33:57.368  2730  2730 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:33:57.974  2730  2730 E libc    : Access denied finding property "persist.device_config.runtime_native_boot.enable_apex_image"
10-29 16:33:57.975  2730  2730 I AndroidRuntime: Using default boot image
10-29 16:33:57.975  2730  2730 E libc    : Access denied finding property "persist.device_config.runtime_native_boot.disable_lock_profiling"
10-29 16:33:57.975  2730  2730 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:33:58.091  2730  2730 E libc    : Access denied finding property "persist.device_config.runtime_native_boot.enable_generational_cc"
10-29 16:33:58.630  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:00.586  1786  1786 I /system/bin/tombstoned: found intercept fd 512 for pid 2659 and type kDebuggerdJavaBacktrace
10-29 16:34:00.665  1955  1984 I system_server: libdebuggerd_client: done dumping process 2659
10-29 16:34:01.562  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:02.088  2487  2487 D TelephonyDebugService: TelephonyDebugService()
10-29 16:34:02.591  2487  2487 D CarrierConfigLoader: Build fingerprint changed. old: null new: google/sdk_gphone_x86_64/generic_x86_64:10/QSR1.211112.011/13135432:userdebug/dev-keys
10-29 16:34:06.708  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:10.049  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:10.796  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:11.902  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:14.545  1955  1981 I system_server: libdebuggerd_client: started dumping process 1955
10-29 16:34:14.701  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1955 and type kDebuggerdJavaBacktrace
10-29 16:34:17.156  2730  2730 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:34:18.306  2730  2730 D AndroidRuntime: Shutting down VM
10-29 16:34:20.262  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:21.424  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:22.512  1786  1786 I /system/bin/tombstoned: found intercept fd 512 for pid 1955 and type kDebuggerdJavaBacktrace
10-29 16:34:22.605  1955  1981 I system_server: libdebuggerd_client: done dumping process 1955
10-29 16:34:22.673  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:22.917  1955  1981 I system_server: libdebuggerd_client: started dumping process 1697
10-29 16:34:22.921  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1697 and type kDebuggerdNativeBacktrace
10-29 16:34:22.934  1697  1697 I libc    : Requested dump for tid 1697 (healthd)
10-29 16:34:23.890  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:34:24.983  1955  1981 E system_server: libdebuggerd_client: failed to read status response from tombstoned: Try again
10-29 16:34:24.985  1955  1981 I system_server: libdebuggerd_client: started dumping process 1713
10-29 16:34:24.988  1786  1786 I /system/bin/tombstoned: intercept for pid 1697 and type kDebuggerdNativeBacktrace terminated: due to input
10-29 16:34:24.988  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1713 and type kDebuggerdNativeBacktrace
10-29 16:34:25.033  1713  1713 I libc    : Requested dump for tid 1713 (health@2.0-serv)
10-29 16:34:25.774  2926  2926 I crash_dump64: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:34:27.045  1955  1981 E system_server: libdebuggerd_client: failed to read status response from tombstoned: Try again
10-29 16:34:27.047  1786  1786 I /system/bin/tombstoned: intercept for pid 1713 and type kDebuggerdNativeBacktrace terminated: due to input
10-29 16:34:27.055  1955  1981 I system_server: libdebuggerd_client: started dumping process 1698
10-29 16:34:27.060  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1698 and type kDebuggerdNativeBacktrace
10-29 16:34:27.267  1698  1698 I libc    : Requested dump for tid 1698 (audio@2.0-servi)
10-29 16:34:28.639  2943  2943 I crash_dump64: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:34:29.120  1955  1981 E system_server: libdebuggerd_client: failed to read status response from tombstoned: Try again
10-29 16:34:29.141  1955  1981 I system_server: libdebuggerd_client: started dumping process 1700
10-29 16:34:29.145  1786  1786 I /system/bin/tombstoned: intercept for pid 1698 and type kDebuggerdNativeBacktrace terminated: due to input
10-29 16:34:29.146  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1700 and type kDebuggerdNativeBacktrace
10-29 16:34:29.151  1700  1700 I libc    : Requested dump for tid 1700 (provider@2.4-se)
10-29 16:34:31.202  1955  1981 E system_server: libdebuggerd_client: failed to read status response from tombstoned: Try again
10-29 16:34:31.204  1786  1786 I /system/bin/tombstoned: intercept for pid 1700 and type kDebuggerdNativeBacktrace terminated: due to input
10-29 16:34:31.215  1955  1981 I system_server: libdebuggerd_client: started dumping process 1781
10-29 16:34:31.261  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1781 and type kDebuggerdNativeBacktrace
10-29 16:34:31.274  1786  1786 I /system/bin/tombstoned: found intercept fd 512 for pid 1781 and type kDebuggerdNativeBacktrace
10-29 16:34:31.588  2971  2971 I crash_dump32: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:34:33.218  1955  1981 E system_server: libdebuggerd_client: timeout expired
10-29 16:34:33.267  1955  1981 I system_server: libdebuggerd_client: started dumping process 1717
10-29 16:34:33.273  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1717 and type kDebuggerdNativeBacktrace
10-29 16:34:33.278  1717  1717 I libc    : Requested dump for tid 1717 (sensors@1.0-ser)
10-29 16:34:33.357  2978  2978 I crash_dump32: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:34:34.320  1955  1981 E system_server: libdebuggerd_client: failed to read status response from tombstoned: Try again
10-29 16:34:34.346  1955  1981 I system_server: libdebuggerd_client: started dumping process 1783
10-29 16:34:34.353  1786  1786 I /system/bin/tombstoned: intercept for pid 1717 and type kDebuggerdNativeBacktrace terminated: due to input
10-29 16:34:34.437  1786  1786 I /system/bin/tombstoned: registered intercept for pid 1783 and type kDebuggerdNativeBacktrace
10-29 16:34:35.058  1955  2291 W system_server: Long monitor contention with owner Binder:1955_8 (2405) at void com.android.server.am.ActivityManagerService.finishReceiver(android.os.IBinder, int, java.lang.String, android.os.Bundle, boolean, int)(ActivityManagerService.java:15663) waiters=0 in void com.android.server.am.ActivityManagerService.setDumpHeapDebugLimit(java.lang.String, int, long, java.lang.String) for 280ms
```
