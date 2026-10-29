<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [40913904274](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40913904274), commit `48709ba31e4821fb749ba4bebea89e4d31072012`, 2026-10-29T16:57:14Z.
x86_64 build of the same code the arm64 APK ships.

```
== device ==
List of devices attached
emulator-5554	device

booted after 1 checks
sdk: 29
abi: x86_64
size: Physical size: 320x640
page size: 4096
apk: 3139126 bytes

== install ==
Performing Streamed Install
Success
package:/data/app/com.rea.studio-qzmCEtxSfHFPhIRaKaZBww==/base.apk
installed

== launch ==
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Status: timeout
LaunchState: UNKNOWN (-1)
Activity: com.rea.studio/.MainActivity
WaitTime: 16168
Complete

== first frame ==
screenshot 01-launch: 28196 bytes

== draw three strokes ==
screenshot 02-painted: 28433 bytes

== undo, then the menu ==
screenshot 03-menu: 28332 bytes

== the app's own boot log ==
REA Image Studio 0.1.0
onCreate
native library loaded: REA Image Studio core 0.1.0 (original implementation)
immersive mode set
building the canvas
interface laid out
document created
cat: files/diag/crash.txt: No such file or directory

== process ==
u0_a146       3648  1691 4937188 140488 0                   0 S com.rea.studio
the app process is running

== activity state ==
    * TaskRecord{c8db87f #6 A=com.rea.studio U=0 StackId=1 sz=1}
      affinity=com.rea.studio
      intent={flg=0x10000000 cmp=com.rea.studio/.MainActivity}
      mActivityComponent=com.rea.studio/.MainActivity
      Activities=[ActivityRecord{bbec708 u0 com.rea.studio/.MainActivity t6}]
      mRootProcess=ProcessRecord{f4f5d9b 3648:com.rea.studio/u0a146}
      * Hist #0: ActivityRecord{bbec708 u0 com.rea.studio/.MainActivity t6}
          packageName=com.rea.studio processName=com.rea.studio
          app=ProcessRecord{f4f5d9b 3648:com.rea.studio/u0a146}
          Intent { flg=0x10000000 cmp=com.rea.studio/.MainActivity }

== crash, ANR and JNI errors in logcat ==
10-29 16:49:48.400  3648  3648 W RenderThread: type=1400 audit(0.0:24): avc: denied { write } for name="property_service" dev="tmpfs" ino=231 scontext=u:r:untrusted_app:s0:c146,c256,c512,c768 tcontext=u:object_r:property_socket:s0 tclass=sock_file permissive=0 app=com.rea.studio
10-29 16:49:40.137  1954  2920 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-29 16:49:42.131  1954  1984 I ActivityManager: Start proc 3648:com.rea.studio/u0a146 for activity {com.rea.studio/com.rea.studio.MainActivity}
10-29 16:49:42.465  3648  3648 I com.rea.studio: Late-enabling -Xcheck:jni
10-29 16:49:44.095  3648  3648 E com.rea.studio: Unknown bits set in runtime_flags: 0x8000
10-29 16:49:44.269  3648  3648 W com.rea.studio: Unexpected CPU variant for X86 using defaults: x86_64
10-29 16:49:48.807  3648  3648 W com.rea.studio: Verification of void com.rea.studio.MainActivity.refreshLayers() took 106.302ms
10-29 16:49:52.256  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 16:49:52.256  1954  1983 E ActivityManager:  +0% 3648/com.rea.studio: 0% user + 0% kernel
10-29 16:49:53.573  3648  3676 I com.rea.studio: Waiting for a blocking GC ProfileSaver
10-29 16:49:54.647  3648  3659 I com.rea.studio: NativeAlloc concurrent copying GC freed 3544(400KB) AllocSpace objects, 0(0B) LOS objects, 49% free, 2154KB/4308KB, paused 107us total 2.584s
10-29 16:49:54.647  3648  3676 I com.rea.studio: WaitForGcToComplete blocked ProfileSaver on ClassLinker for 1.074s
10-29 16:50:35.681  1954  1983 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 16:50:49.470  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 16:50:55.111  1954  1983 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 16:50:58.448  3885  3885 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:50:59.223  3885  3885 I AndroidRuntime: Using default boot image
10-29 16:50:59.224  3885  3885 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:51:25.966  1954  1983 E ActivityManager: ANR in com.google.android.apps.messaging
10-29 16:51:34.830  3885  3885 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:51:35.170  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 16:51:36.184  3885  3885 D AndroidRuntime: Shutting down VM
10-29 16:51:41.007  1954  1983 E ActivityManager: ANR in com.android.dialer
10-29 16:51:54.529  4362  4362 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:51:55.090  4362  4362 I AndroidRuntime: Using default boot image
10-29 16:51:55.090  4362  4362 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:51:55.590  1954  1983 E ActivityManager: ANR in android.process.media
10-29 16:52:06.607  1954  1983 E ActivityManager: ANR in com.google.android.apps.docs
10-29 16:52:13.195  1954  1983 E ActivityManager: ANR in com.google.android.apps.messaging
10-29 16:52:15.686  4362  4362 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:52:16.769  4362  4362 D AndroidRuntime: Shutting down VM
10-29 16:52:19.305  1954  1983 E ActivityManager: ANR in com.google.android.ims
10-29 16:52:23.488  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 16:52:25.322  1954  3695 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.packageinstaller/com.android.packageinstaller.PackageInstalledReceiver
10-29 16:52:25.326  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.android.musicfx/.Compatibility$Receiver
10-29 16:52:25.327  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.apps.enterprise.dmagent/.AppUpdateReceiver
10-29 16:52:25.327  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.games.chimera.GamesSystemBroadcastReceiverProxy
10-29 16:52:25.327  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.gass.chimera.PackageChangeBroadcastReceiver
10-29 16:52:25.328  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.chimera.GmsIntentOperationService$PersistentTrustedReceiver
10-29 16:52:25.328  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.googlequicksearchbox/com.google.android.apps.gsa.googlequicksearchbox.GelStubAppWatcher
10-29 16:52:25.329  1954  1983 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.ims/.receivers.RcsAutoStartReceiver
10-29 16:52:35.004  4658  4658 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:52:35.596  4658  4658 I AndroidRuntime: Using default boot image
10-29 16:52:35.599  4658  4658 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:52:44.438  1954  1983 E ActivityManager: ANR in android.process.media
10-29 16:52:51.655  4658  4658 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:52:52.795  4658  4658 D AndroidRuntime: Shutting down VM
10-29 16:53:16.250  1954  1983 E ActivityManager: ANR in com.google.android.gms
10-29 16:53:38.976  5041  5041 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:53:39.523  5041  5041 I AndroidRuntime: Using default boot image
10-29 16:53:39.524  5041  5041 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:53:48.431  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 16:53:53.387  5041  5041 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:53:54.163  5041  5041 D AndroidRuntime: Shutting down VM
10-29 16:53:55.801  1954  1983 E ActivityManager: ANR in com.google.android.gms
10-29 16:54:14.761  5221  5221 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:54:14.949  5221  5221 I AndroidRuntime: Using default boot image
10-29 16:54:14.950  5221  5221 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:54:16.003  1954  1982 I ActivityTaskManager: Displayed com.rea.studio/.MainActivity: +17s333ms
10-29 16:54:27.665  5221  5221 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:54:29.464  5221  5221 D AndroidRuntime: Shutting down VM
10-29 16:54:49.860  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 16:55:13.170  5458  5458 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:55:13.541  5458  5458 I AndroidRuntime: Using default boot image
10-29 16:55:13.698  5458  5458 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:55:28.814  5458  5458 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:55:29.510  5458  5458 D AndroidRuntime: Shutting down VM
10-29 16:55:29.715  1954  1983 E ActivityManager: ANR in com.google.android.gms.persistent

== result ==
ok 1793292591411
RESULT: the app reached a drawn interface
evidence: /home/runner/work/_temp/smoke.txt
```

## Device log (filtered)

```
10-29 16:54:47.618  5185  5185 I libc    : Requested dump for tid 5185 (.gms.persistent)
10-29 16:54:49.614  1954  1983 E system_server: libdebuggerd_client: failed to read status response from tombstoned: Try again
10-29 16:54:49.619  1788  1788 I /system/bin/tombstoned: intercept for pid 5185 and type kDebuggerdNativeBacktrace terminated: due to input
10-29 16:54:50.171  5432  5432 E DEBUG   : failed to open directory /proc/5185/fd: Permission denied
10-29 16:54:52.075  4939  5400 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:54:52.186  1954  1954 W system_server: Long monitor contention with owner android.bg (1982) at android.system.StructStat libcore.io.Linux.fstat(java.io.FileDescriptor)(Linux.java:-2) waiters=0 in void com.android.server.appwidget.AppWidgetServiceImpl.onPackageBroadcastReceived(android.content.Intent, int) for 1.371s
10-29 16:54:56.557  4939  5400 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:55:07.491  5436  5436 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:55:13.170  5458  5458 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 16:55:13.540  5458  5458 E libc    : Access denied finding property "persist.device_config.runtime_native_boot.enable_apex_image"
10-29 16:55:13.541  5458  5458 I AndroidRuntime: Using default boot image
10-29 16:55:13.698  5458  5458 E libc    : Access denied finding property "persist.device_config.runtime_native_boot.disable_lock_profiling"
10-29 16:55:13.698  5458  5458 I AndroidRuntime: Leaving lock profiling enabled
10-29 16:55:13.700  5458  5458 E libc    : Access denied finding property "persist.device_config.runtime_native_boot.enable_generational_cc"
10-29 16:55:18.338  4762  4847 I FA      : To enable debug logging run: adb shell setprop log.tag.FA VERBOSE
10-29 16:55:18.339  4762  4847 I FA      : To enable faster debug mode event logging run:
10-29 16:55:18.339  4762  4847 I FA      :   adb shell setprop debug.firebase.analytics.app com.google.android.apps.messaging
10-29 16:55:22.179  1954  1983 I system_server: libdebuggerd_client: started dumping process 5436
10-29 16:55:22.243  1788  1788 I /system/bin/tombstoned: registered intercept for pid 5436 and type kDebuggerdJavaBacktrace
10-29 16:55:24.409  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:25.486  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:26.351  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:28.003  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:28.505  4748  5562 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:55:28.814  5458  5458 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 16:55:29.106  1788  1788 I /system/bin/tombstoned: found intercept fd 512 for pid 5436 and type kDebuggerdJavaBacktrace
10-29 16:55:29.124  1954  1983 I system_server: libdebuggerd_client: done dumping process 5436
10-29 16:55:29.510  5458  5458 D AndroidRuntime: Shutting down VM
10-29 16:55:29.934  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:31.177  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:33.613  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:35.912  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:37.729  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:41.487  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:42.680  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:44.733  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:45.607  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:45.643  5572  5572 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:55:46.820  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:47.449  4887  5017 W droid.apps.doc: Long monitor contention with owner DocumentFileManager (5261) at int libcore.io.Linux.writeBytes(java.io.FileDescriptor, java.lang.Object, int, int)(Linux.java:-2) waiters=0 in void android.app.SharedPreferencesImpl$2.run() for 806ms
10-29 16:55:48.012  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:48.286  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:49.012  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:49.164  5607  5607 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:55:50.264  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:50.699  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:52.413  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:53.818  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:55.663  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:56.353  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:56.733  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:57.126  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:57.583  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:58.989  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:59.573  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:55:59.813  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:02.648  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:03.550  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:04.175  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:04.468  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:04.932  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:05.744  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:06.123  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:06.268  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:07.128  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:08.195  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:09.251  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:10.087  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:11.076  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:12.746  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:13.116  1954  1982 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:38.783  1954  1983 I system_server: libdebuggerd_client: started dumping process 5572
10-29 16:56:38.806  1788  1788 I /system/bin/tombstoned: registered intercept for pid 5572 and type kDebuggerdJavaBacktrace
10-29 16:56:42.109  1788  1788 I /system/bin/tombstoned: found intercept fd 512 for pid 5572 and type kDebuggerdJavaBacktrace
10-29 16:56:42.161  1954  1983 I system_server: libdebuggerd_client: done dumping process 5572
10-29 16:56:45.864  1954  3784 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:56:46.136  1954  3784 W system_server: Long monitor contention with owner Binder:1954_1E (4661) at void com.android.server.am.ActivityManagerService.attachApplication(android.app.IApplicationThread, long)(ActivityManagerService.java:5176) waiters=5 in android.os.Debug$MemoryInfo[] com.android.server.am.ActivityManagerService.getProcessMemoryInfo(int[]) for 272ms
10-29 16:56:53.946  5767  5767 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:57:00.485  4939  5810 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 16:57:01.269  4939  5810 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
```
