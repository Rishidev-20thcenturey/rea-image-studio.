<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [40923833620](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40923833620), commit `c8cf9626e047393d52ba963e11fcdcf5bb3f1f92`, 2026-10-29T18:05:19Z.
x86_64 build of the same code the arm64 APK ships.

```
== device ==
List of devices attached
emulator-5554	device

booted after 1 checks
sdk: 29
abi: x86_64
size: 640x800
page size: 4096
apk: 3139126 bytes

== install ==
Performing Streamed Install
Success
package:/data/app/com.rea.studio-i3BIv43vgyBlXaCf0MIqHA==/base.apk
installed

== launch ==
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Status: timeout
LaunchState: UNKNOWN (-1)
Activity: com.rea.studio/.MainActivity
WaitTime: 15890
Complete

== first frame ==
screenshot 01-launch: 46270 bytes
Broadcasting: Intent { act=android.intent.action.CLOSE_SYSTEM_DIALOGS flg=0x400000 }
Broadcast completed: result=0
focus:   mCurrentFocus=Window{a6f05dd u0 Application Not Responding: com.android.systemui}
the app did not have focus; dismissing dialogs and restarting it
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Warning: Activity not started, intent has been delivered to currently running top-most instance.
focus now:   mCurrentFocus=Window{a6f05dd u0 Application Not Responding: com.android.systemui}

== draw three strokes ==
screenshot 02-painted: 42878 bytes

== switch to the eraser from the rail, then undo ==
screenshot 03-tool: 41175 bytes

== open the layer panel ==
screenshot 04-layers: 53178 bytes

== open the menu ==
screenshot 05-menu: 53548 bytes

== the interface's own end-to-end check ==
Starting: Intent { cmp=com.rea.studio/.MainActivity (has extras) }
[ok] touch -> paint -> pixels
document: 320x320
canvas: 640x800
zoom: 1.925

--- core self-test ---
[ok] document 96x64, 1 layer(s)
[ok] brush 1439 pixels painted
[ok] render 4800/4800 pixels opaque
[ok] undo 2878 -> 0 non-zero bytes
[ok] selection 50x30
[ok] save/load Some((96, 64))
[ok] export 6144 pixels
all checks passed
screenshot 06-selftest: 44992 bytes

== the app's own boot log ===
REA Image Studio 0.1.0
onCreate
native library loaded: REA Image Studio core 0.1.0 (original implementation)
immersive mode set
building the canvas
interface laid out
document created
ui self-test finished
cat: files/diag/crash.txt: No such file or directory

== process ==
u0_a146       6657  1690 4943520 147388 0                   0 S com.rea.studio
the app process is running

== activity state ==
    * TaskRecord{ccd4ea0 #7 A=com.rea.studio U=0 StackId=2 sz=1}
      affinity=com.rea.studio
      intent={flg=0x10000000 cmp=com.rea.studio/.MainActivity}
      mActivityComponent=com.rea.studio/.MainActivity
      Activities=[ActivityRecord{968842b u0 com.rea.studio/.MainActivity t7}]
      mRootProcess=ProcessRecord{11f6a59 6657:com.rea.studio/u0a146}
      * Hist #0: ActivityRecord{968842b u0 com.rea.studio/.MainActivity t7}
          packageName=com.rea.studio processName=com.rea.studio
          app=ProcessRecord{11f6a59 6657:com.rea.studio/u0a146}
          Intent { flg=0x10000000 cmp=com.rea.studio/.MainActivity (has extras) }

== crash, ANR and JNI errors in logcat ==
10-29 17:50:44.901  1949  3131 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-29 17:50:46.018  1949  1978 E ActivityManager: ANR in com.google.android.ims
10-29 17:50:48.211  1949  1979 I ActivityManager: Start proc 3829:com.rea.studio/u0a146 for activity {com.rea.studio/com.rea.studio.MainActivity}
10-29 17:50:48.475  3829  3829 I com.rea.studio: Late-enabling -Xcheck:jni
10-29 17:50:49.393  3829  3829 E com.rea.studio: Unknown bits set in runtime_flags: 0x8000
10-29 17:50:49.586  3829  3829 W com.rea.studio: Unexpected CPU variant for X86 using defaults: x86_64
10-29 17:50:53.680  3829  3829 W RenderThread: type=1400 audit(0.0:29): avc: denied { write } for name="property_service" dev="tmpfs" ino=845 scontext=u:r:untrusted_app:s0:c146,c256,c512,c768 tcontext=u:object_r:property_socket:s0 tclass=sock_file permissive=0 app=com.rea.studio
10-29 17:50:53.798  3829  3829 W com.rea.studio: Verification of void com.rea.studio.MainActivity.buildLayersPanel(android.widget.FrameLayout) took 128.764ms
10-29 17:50:54.481  3829  3829 W com.rea.studio: Verification of void com.rea.studio.MainActivity.showDiagnostics$lambda$132(com.rea.studio.MainActivity, android.widget.TextView, android.content.DialogInterface, int) took 385.491ms
10-29 17:50:59.072  3829  3866 I com.rea.studio: Waiting for a blocking GC ProfileSaver
10-29 17:51:00.479  1949  1978 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 17:51:00.479  1949  1978 E ActivityManager:   42% 3829/com.rea.studio: 10% user + 32% kernel / faults: 8074 minor 7 major
10-29 17:51:00.831  3829  3853 I com.rea.studio: NativeAlloc concurrent copying GC freed 1206(111KB) AllocSpace objects, 0(0B) LOS objects, 49% free, 3330KB/6660KB, paused 75us total 2.043s
10-29 17:51:00.833  3829  3866 I com.rea.studio: WaitForGcToComplete blocked ProfileSaver on HeapTrim for 1.760s
10-29 17:51:14.444  1949  1978 E ActivityManager: ANR in android.process.media
10-29 17:51:54.175  1949  1978 E ActivityManager: ANR in android.process.media
10-29 17:52:15.660  1949  1978 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 17:52:21.364  1949  1978 E ActivityManager: ANR in com.google.android.apps.docs
10-29 17:52:28.415  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:52:32.036  1949  1978 E ActivityManager: ANR in android.process.media
10-29 17:53:50.840  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:54:08.849  1949  1977 I ActivityTaskManager: Displayed com.rea.studio/.MainActivity: +17s562ms
10-29 17:54:24.009  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:54:48.545  1949  3440 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.packageinstaller/com.android.packageinstaller.PackageInstalledReceiver
10-29 17:54:48.865  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.android.musicfx/.Compatibility$Receiver
10-29 17:54:48.873  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.apps.enterprise.dmagent/.AppUpdateReceiver
10-29 17:54:48.874  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.games.chimera.GamesSystemBroadcastReceiverProxy
10-29 17:54:48.875  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.gass.chimera.PackageChangeBroadcastReceiver
10-29 17:54:48.875  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.chimera.GmsIntentOperationService$PersistentTrustedReceiver
10-29 17:54:48.875  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.googlequicksearchbox/com.google.android.apps.gsa.googlequicksearchbox.GelStubAppWatcher
10-29 17:54:48.875  1949  1978 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.ims/.receivers.RcsAutoStartReceiver
10-29 17:55:11.591  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:55:25.462  1949  1978 E ActivityManager: ANR in com.google.android.ims
10-29 17:55:45.707  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:56:01.350  5239  5239 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:56:01.853  5239  5239 I AndroidRuntime: Using default boot image
10-29 17:56:01.854  5239  5239 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:56:12.960  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:56:25.948  5239  5239 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:56:28.575  1949  1978 E ActivityManager: ANR in com.google.android.apps.messaging
10-29 17:56:29.126  5239  5239 D AndroidRuntime: Shutting down VM
10-29 17:56:59.005  5517  5517 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:57:00.644  5517  5517 I AndroidRuntime: Using default boot image
10-29 17:57:00.645  5517  5517 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:57:08.282  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:57:19.163  1949  1978 E ActivityManager: ANR in com.google.android.gms
10-29 17:57:20.578  5517  5517 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:57:21.397  5517  5517 D AndroidRuntime: Shutting down VM
10-29 17:57:27.707  1949  3099 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-29 17:57:58.518  5776  5776 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:57:58.790  5776  5776 I AndroidRuntime: Using default boot image
10-29 17:57:58.790  5776  5776 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:58:09.157  5776  5776 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:58:10.159  5776  5776 D AndroidRuntime: Shutting down VM
10-29 17:58:15.629  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:58:27.709  5906  5906 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:58:28.109  5906  5906 I AndroidRuntime: Using default boot image
10-29 17:58:28.110  5906  5906 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:58:40.141  5906  5906 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:58:43.446  5906  5906 D AndroidRuntime: Shutting down VM
10-29 17:59:08.357  6015  6015 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:59:08.754  6015  6015 I AndroidRuntime: Using default boot image
10-29 17:59:08.755  6015  6015 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:59:18.124  6015  6015 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:59:20.927  6015  6015 D AndroidRuntime: Shutting down VM
10-29 17:59:41.608  1949  1978 E ActivityManager: ANR in com.google.android.gms.persistent
10-29 17:59:41.608  1949  1978 E ActivityManager:   0.3% 3829/com.rea.studio: 0% user + 0.2% kernel / faults: 2697 minor
10-29 17:59:50.590  6094  6094 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:59:50.659  6094  6094 I AndroidRuntime: Using default boot image
10-29 17:59:50.659  6094  6094 I AndroidRuntime: Leaving lock profiling enabled
10-29 18:00:02.116  6094  6094 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 18:00:02.711  6094  6094 D AndroidRuntime: Shutting down VM
10-29 18:00:32.778  6200  6200 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 18:00:32.990  6200  6200 I AndroidRuntime: Using default boot image
10-29 18:00:32.991  6200  6200 I AndroidRuntime: Leaving lock profiling enabled
10-29 18:00:43.200  6200  6200 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 18:00:43.784  6200  6200 D AndroidRuntime: Shutting down VM
10-29 18:00:44.206  3829  3866 I com.rea.studio: Waiting for a blocking GC ProfileSaver
10-29 18:00:46.445  3829  3853 I com.rea.studio: NativeAlloc concurrent copying GC freed 2261(177KB) AllocSpace objects, 2(32KB) LOS objects, 49% free, 3368KB/6736KB, paused 81us total 2.501s
10-29 18:00:46.446  3829  3866 I com.rea.studio: WaitForGcToComplete blocked ProfileSaver on ProfileSaver for 2.239s

== result ==
ok 1793297002714
RESULT: the app reached a drawn interface
evidence: /home/runner/work/_temp/smoke.txt
```

## Device log (filtered)

```
10-29 18:03:07.800  1949  3642 I ActivityManager: Force stopping com.rea.studio appid=10146 user=0: from pid 6615
10-29 18:03:07.802  1949  3642 I ActivityManager: Killing 3829:com.rea.studio/u0a146 (adj 0): stop com.rea.studio
10-29 18:03:07.806  1949  3642 W ActivityTaskManager: Force removing ActivityRecord{3be7844 u0 com.rea.studio/.MainActivity t6 f}: app died, no saved state
10-29 18:03:12.872  1949  3451 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity (has extras)} from uid 2000
10-29 18:03:15.208  6657  6657 I com.rea.studio: Late-enabling -Xcheck:jni
10-29 18:03:15.664  6657  6657 E com.rea.studio: Unknown bits set in runtime_flags: 0x8000
10-29 18:03:15.701  6657  6657 W com.rea.studio: Unexpected CPU variant for X86 using defaults: x86_64
10-29 18:03:17.166  1949  1979 I ActivityManager: Start proc 6657:com.rea.studio/u0a146 for activity {com.rea.studio/com.rea.studio.MainActivity}
10-29 18:03:17.214  1949  1978 I system_server: libdebuggerd_client: started dumping process 6474
10-29 18:03:17.223  1784  1784 I /system/bin/tombstoned: registered intercept for pid 6474 and type kDebuggerdJavaBacktrace
10-29 18:03:18.986  6657  6657 I com.rea.studio: Waiting for a blocking GC ClassLinker
10-29 18:03:19.082  6657  6657 I com.rea.studio: WaitForGcToComplete blocked ClassLinker on HeapTrim for 95.908ms
10-29 18:03:19.977  6657  6695 W libc    : Unable to set property "qemu.gles" to "1": connection failed; errno=13 (Permission denied)
10-29 18:03:19.970  6657  6657 W RenderThread: type=1400 audit(0.0:76): avc: denied { write } for name="property_service" dev="tmpfs" ino=845 scontext=u:r:untrusted_app:s0:c146,c256,c512,c768 tcontext=u:object_r:property_socket:s0 tclass=sock_file permissive=0 app=com.rea.studio
10-29 18:03:24.548  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:24.664  6657  6689 I com.rea.studio: Waiting for a blocking GC ProfileSaver
10-29 18:03:24.941  6657  6670 I com.rea.studio: NativeAlloc concurrent copying GC freed 3555(368KB) AllocSpace objects, 0(0B) LOS objects, 54% free, 1299KB/2835KB, paused 216us total 1.471s
10-29 18:03:24.941  6657  6689 I com.rea.studio: WaitForGcToComplete blocked ProfileSaver on ClassLinker for 309.382ms
10-29 18:03:24.958  1784  1784 I /system/bin/tombstoned: found intercept fd 512 for pid 6474 and type kDebuggerdJavaBacktrace
10-29 18:03:24.975  1949  1978 I system_server: libdebuggerd_client: done dumping process 6474
10-29 18:03:26.071  6657  6670 I com.rea.studio: Background young concurrent copying GC freed 3(30KB) AllocSpace objects, 0(0B) LOS objects, 0% free, 3323KB/3323KB, paused 75us total 494.437ms
10-29 18:03:26.143  1949  1978 E ActivityManager:  +0% 6657/com.rea.studio: 0% user + 0% kernel
10-29 18:03:26.314  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:28.255  6657  6670 I com.rea.studio: Background concurrent copying GC freed 287(67KB) AllocSpace objects, 0(0B) LOS objects, 49% free, 3336KB/6672KB, paused 77us total 1.898s
10-29 18:03:29.471  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:30.171  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:31.897  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:32.943  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:33.182  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:34.489  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:35.628  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:37.244  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:37.954  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:38.595  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:39.492  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:40.289  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:42.543  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:43.436  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:03:51.290  6763  6763 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 18:04:14.165  1949  1978 I system_server: libdebuggerd_client: started dumping process 6763
10-29 18:04:14.206  1784  1784 I /system/bin/tombstoned: registered intercept for pid 6763 and type kDebuggerdJavaBacktrace
10-29 18:04:16.260  1784  1784 I /system/bin/tombstoned: found intercept fd 512 for pid 6763 and type kDebuggerdJavaBacktrace
10-29 18:04:16.340  1949  1978 I system_server: libdebuggerd_client: done dumping process 6763
10-29 18:04:16.411  1949  1978 E ActivityManager:   4.7% 6657/com.rea.studio: 1.2% user + 3.4% kernel / faults: 1824 minor
10-29 18:04:22.765  4461  6979 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 18:04:22.887  4461  6979 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 18:04:26.776  1949  1949 V SettingsProvider: Notifying for 0: content://settings/global/angle_debug_package
10-29 18:04:29.663  7001  7001 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 18:04:42.143  1949  1978 I system_server: libdebuggerd_client: started dumping process 7001
10-29 18:04:42.155  1784  1784 I /system/bin/tombstoned: registered intercept for pid 7001 and type kDebuggerdJavaBacktrace
10-29 18:04:44.194  7125  7184 W libc    : Unable to set property "qemu.gles" to "1": connection failed; errno=13 (Permission denied)
10-29 18:04:44.381  1784  1784 I /system/bin/tombstoned: found intercept fd 512 for pid 7001 and type kDebuggerdJavaBacktrace
10-29 18:04:44.401  1949  1978 I system_server: libdebuggerd_client: done dumping process 7001
10-29 18:04:44.568  1949  1978 E ActivityManager:   0.1% 6657/com.rea.studio: 0% user + 0% kernel / faults: 24 minor
10-29 18:04:49.057  1949  1977 I ActivityTaskManager: Displayed com.rea.studio/.MainActivity: +15s519ms
10-29 18:04:50.159  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:50.275  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:50.546  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:50.595  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:50.691  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:51.211  7195  7195 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 18:04:51.213  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:51.536  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:51.830  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:52.253  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:52.535  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:53.134  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:54.206  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:54.331  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:54.351  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:55.203  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:55.454  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:55.723  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:04:58.704  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:05:15.091  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:05:15.568  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:05:16.617  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:05:17.013  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:05:17.728  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
10-29 18:05:18.477  1949  1977 W android.os.Debug: failed to get memory consumption info: -1
```

![01-launch](01-launch.png)

![02-painted](02-painted.png)

![04-layers](04-layers.png)
