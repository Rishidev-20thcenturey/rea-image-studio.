<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [40920093516](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40920093516), commit `35918a7749fc7d661dcd0aab4a8ad0d3f5f04201`, 2026-10-29T17:39:34Z.
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
package:/data/app/com.rea.studio-iBNvTAjCIf9vLBOUaR182Q==/base.apk
installed

== launch ==
failed to clear the 'main' log
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Status: ok
LaunchState: COLD
Activity: com.rea.studio/.MainActivity
TotalTime: 15545
WaitTime: 15763
Complete

== first frame ==
screenshot 01-launch: 46495 bytes
focus:   mCurrentFocus=Window{b2c8082 u0 Application Not Responding: com.android.systemui}
the app did not have focus; dismissing dialogs and restarting it
Starting: Intent { cmp=com.rea.studio/.MainActivity }
Warning: Activity not started, intent has been delivered to currently running top-most instance.
focus now:   mCurrentFocus=Window{b2c8082 u0 Application Not Responding: com.android.systemui}

== draw three strokes ==
screenshot 02-painted: 42881 bytes

== switch to the eraser from the rail, then undo ==
screenshot 03-tool: 41175 bytes

== open the layer panel ==
screenshot 04-layers: 53428 bytes

== open the menu ==
screenshot 05-menu: 53589 bytes

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
u0_a146       4038  1691 4998568 158112 0                   0 S com.rea.studio
the app process is running

== activity state ==
    * TaskRecord{50f94b6 #6 A=com.rea.studio U=0 StackId=1 sz=1}
      affinity=com.rea.studio
      intent={flg=0x10000000 cmp=com.rea.studio/.MainActivity}
      mActivityComponent=com.rea.studio/.MainActivity
      Activities=[ActivityRecord{79da97f u0 com.rea.studio/.MainActivity t6}]
      mRootProcess=ProcessRecord{1b5f5b7 4038:com.rea.studio/u0a146}
      * Hist #0: ActivityRecord{79da97f u0 com.rea.studio/.MainActivity t6}
          packageName=com.rea.studio processName=com.rea.studio
          app=ProcessRecord{1b5f5b7 4038:com.rea.studio/u0a146}
          Intent { flg=0x10000000 cmp=com.rea.studio/.MainActivity }

== crash, ANR and JNI errors in logcat ==
10-29 17:28:05.823  1955  1984 E ActivityManager: ANR in android.process.media
10-29 17:28:11.960  1955  1974 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-29 17:28:14.962  1955  1985 I ActivityManager: Start proc 4038:com.rea.studio/u0a146 for activity {com.rea.studio/com.rea.studio.MainActivity}
10-29 17:28:15.155  4038  4038 I com.rea.studio: Late-enabling -Xcheck:jni
10-29 17:28:16.479  4038  4038 E com.rea.studio: Unknown bits set in runtime_flags: 0x8000
10-29 17:28:16.483  4038  4038 W com.rea.studio: Unexpected CPU variant for X86 using defaults: x86_64
10-29 17:28:17.709  4038  4038 W com.rea.studio: Verification of void com.android.org.conscrypt.TrustedCertificateStore$PreloadHolder.<clinit>() took 193.666ms
10-29 17:28:19.919  4038  4038 I com.rea.studio: Waiting for a blocking GC ClassLinker
10-29 17:28:19.978  4038  4038 I com.rea.studio: WaitForGcToComplete blocked ClassLinker on HeapTrim for 59.392ms
10-29 17:28:20.640  4038  4038 W RenderThread: type=1400 audit(0.0:30): avc: denied { write } for name="property_service" dev="tmpfs" ino=1241 scontext=u:r:untrusted_app:s0:c146,c256,c512,c768 tcontext=u:object_r:property_socket:s0 tclass=sock_file permissive=0 app=com.rea.studio
10-29 17:28:25.436  4038  4078 I com.rea.studio: Waiting for a blocking GC ProfileSaver
10-29 17:28:26.565  4038  4060 I com.rea.studio: NativeAlloc concurrent copying GC freed 3532(403KB) AllocSpace objects, 0(0B) LOS objects, 49% free, 3369KB/6738KB, paused 88us total 2.834s
10-29 17:28:26.565  4038  4078 I com.rea.studio: WaitForGcToComplete blocked ProfileSaver on ClassLinker for 1.129s
10-29 17:29:13.602  1955  1984 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 17:29:32.112  4336  4336 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:29:32.585  4336  4336 I AndroidRuntime: Using default boot image
10-29 17:29:32.591  4336  4336 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:29:44.284  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.packageinstaller/com.android.packageinstaller.PackageInstalledReceiver
10-29 17:29:44.284  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.android.musicfx/.Compatibility$Receiver
10-29 17:29:44.285  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.apps.enterprise.dmagent/.AppUpdateReceiver
10-29 17:29:44.285  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.googlequicksearchbox/com.google.android.apps.gsa.googlequicksearchbox.GelStubAppWatcher
10-29 17:29:44.290  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.ims/.receivers.RcsAutoStartReceiver
10-29 17:29:44.669  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.games.chimera.GamesSystemBroadcastReceiverProxy
10-29 17:29:44.670  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.gass.chimera.PackageChangeBroadcastReceiver
10-29 17:29:44.670  1955  1984 W BroadcastQueue: Background execution not allowed: receiving Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 (has extras) } to com.google.android.gms/.chimera.GmsIntentOperationService$PersistentTrustedReceiver
10-29 17:29:55.281  4336  4336 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:29:55.800  4336  4336 D AndroidRuntime: Shutting down VM
10-29 17:30:02.088  1955  3359 I ActivityTaskManager: START u0 {flg=0x10000000 cmp=com.rea.studio/.MainActivity} from uid 2000
10-29 17:30:31.359  1955  1984 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 17:30:33.743  4645  4645 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:30:33.898  4645  4645 I AndroidRuntime: Using default boot image
10-29 17:30:33.917  4645  4645 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:30:36.473  2687  4664 W GCM     : Unexpected forwarded intent: Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 cmp=com.google.android.gms/.chimera.PersistentIntentOperationService (has extras) }
10-29 17:30:37.721  1955  1984 E ActivityManager: ANR in com.google.android.apps.messaging
10-29 17:30:38.368  2687  4502 W GCM     : Unexpected forwarded intent: Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 cmp=com.google.android.gms/.chimera.PersistentIntentOperationService (has extras) }
10-29 17:30:41.980  2604  4703 I ChromeSync: [Sync,SyncIntentOperation] Handling the intent: Intent { act=android.intent.action.PACKAGE_ADDED dat=package:com.rea.studio flg=0x4000010 cmp=com.google.android.gms/.chimera.GmsIntentOperationService (has extras) }.
10-29 17:30:42.033  2604  4702 I FontLog : Package com.rea.studio has no metadata [CONTEXT service_id=132 ]
10-29 17:30:48.182  2604  4703 I ChromeSync: [Persistence,AffiliationManager] One of affiliation or grouping data for android://aSxPsqqkS6R6kS2MqAEiQ2htYGZ3eXew6daKztRQ4QTtJ2j86qPSp5rlJGISC88gY4iFnwrDk_6FjDzm-HvuZw==@com.rea.studio/ was not found. Marking affiliation data as stale...
10-29 17:30:49.803  4645  4645 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:30:50.980  4645  4645 D AndroidRuntime: Shutting down VM
10-29 17:31:06.184  1955  1984 E ActivityManager: ANR in com.google.android.youtube
10-29 17:31:15.381  4802  4802 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:31:15.820  4802  4802 I AndroidRuntime: Using default boot image
10-29 17:31:15.820  4802  4802 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:31:32.821  4802  4802 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:31:36.569  4802  4802 D AndroidRuntime: Shutting down VM
10-29 17:31:56.715  1955  1983 I ActivityTaskManager: Displayed com.rea.studio/.MainActivity: +15s545ms
10-29 17:32:02.851  4990  4990 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:32:02.932  4990  4990 I AndroidRuntime: Using default boot image
10-29 17:32:02.933  4990  4990 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:32:19.629  4990  4990 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:32:21.668  4990  4990 D AndroidRuntime: Shutting down VM
10-29 17:32:22.458  4038  4060 I com.rea.studio: NativeAlloc concurrent copying GC freed 1135(135KB) AllocSpace objects, 0(0B) LOS objects, 49% free, 3300KB/6601KB, paused 81us total 887.006ms
10-29 17:33:00.460  5251  5251 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:33:00.870  5251  5251 I AndroidRuntime: Using default boot image
10-29 17:33:00.871  5251  5251 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:33:17.757  5251  5251 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:33:19.674  5251  5251 D AndroidRuntime: Shutting down VM
10-29 17:33:32.168  1955  1984 E ActivityManager: ANR in com.google.android.googlequicksearchbox:search
10-29 17:33:32.168  1955  1984 E ActivityManager:   5.5% 4038/com.rea.studio: 1.3% user + 4.2% kernel / faults: 201 minor 2 major
10-29 17:33:43.682  1955  1984 E ActivityManager: ANR in com.android.cellbroadcastreceiver
10-29 17:34:12.039  5502  5502 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:34:12.567  5502  5502 I AndroidRuntime: Using default boot image
10-29 17:34:12.587  5502  5502 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:34:42.417  5502  5502 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:34:43.094  5502  5502 D AndroidRuntime: Shutting down VM
10-29 17:34:48.518  1955  1984 E ActivityManager: ANR in com.google.android.gms.unstable
10-29 17:34:48.518  1955  1984 E ActivityManager:   2.2% 4038/com.rea.studio: 0.4% user + 1.8% kernel / faults: 4139 minor 3 major
10-29 17:35:03.859  5726  5726 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:35:04.170  5726  5726 I AndroidRuntime: Using default boot image
10-29 17:35:04.171  5726  5726 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:35:22.987  5726  5726 D AndroidRuntime: Calling main entry com.android.commands.input.Input
10-29 17:35:23.622  4038  4078 I com.rea.studio: Waiting for a blocking GC ProfileSaver
10-29 17:35:23.648  5726  5726 D AndroidRuntime: Shutting down VM
10-29 17:35:25.318  4038  4060 I com.rea.studio: NativeAlloc concurrent copying GC freed 1456(102KB) AllocSpace objects, 2(32KB) LOS objects, 49% free, 3446KB/6892KB, paused 10.545ms total 1.709s
10-29 17:35:25.318  4038  4078 I com.rea.studio: WaitForGcToComplete blocked ProfileSaver on ProfileSaver for 1.696s
10-29 17:36:04.085  5968  5968 D AndroidRuntime: >>>>>> START com.android.internal.os.RuntimeInit uid 2000 <<<<<<
10-29 17:36:04.411  5968  5968 I AndroidRuntime: Using default boot image
10-29 17:36:04.412  5968  5968 I AndroidRuntime: Leaving lock profiling enabled
10-29 17:36:15.159  5968  5968 D AndroidRuntime: Calling main entry com.android.commands.input.Input

== result ==
ok 1793294902843
RESULT: the app reached a drawn interface
evidence: /home/runner/work/_temp/smoke.txt
```

## Device log (filtered)

```
10-29 17:37:25.839  4750  5256 W RcsProvisioning: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:252)
10-29 17:37:25.839  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:25.839  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:482)
10-29 17:37:27.735  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:496)
10-29 17:37:27.735  4750  5256 W RcsProvisioning: 	at libcore.io.Linux.open(Native Method)
10-29 17:37:27.735  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:27.735  4750  5256 W RcsProvisioning: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:252)
10-29 17:37:27.735  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:27.735  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:482)
10-29 17:37:28.107  4750  5256 E BugleRcsEngine: 	libcore.io.IoBridge.open(IoBridge.java:496)
10-29 17:37:28.884  4750  5256 I BugleRcsEngine: [141] omj.shouldUseCarrierServicesApkForV1Apis: Checking if using CarrierServices.apk is possible. Enabled: true, isAtLeastM: true, runningInsideBugle: true
10-29 17:37:33.834  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:496)
10-29 17:37:33.834  4750  5256 W RcsProvisioning: 	at libcore.io.Linux.open(Native Method)
10-29 17:37:33.834  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:33.834  4750  5256 W RcsProvisioning: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:252)
10-29 17:37:33.834  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:33.834  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:482)
10-29 17:37:35.675  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:496)
10-29 17:37:35.675  4750  5256 W RcsProvisioning: 	at libcore.io.Linux.open(Native Method)
10-29 17:37:35.675  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:35.675  4750  5256 W RcsProvisioning: 	at libcore.io.BlockGuardOs.open(BlockGuardOs.java:252)
10-29 17:37:35.675  4750  5256 W RcsProvisioning: 	at libcore.io.ForwardingOs.open(ForwardingOs.java:167)
10-29 17:37:35.675  4750  5256 W RcsProvisioning: 	at libcore.io.IoBridge.open(IoBridge.java:482)
10-29 17:37:36.981  1955  1984 I system_server: libdebuggerd_client: started dumping process 5463
10-29 17:37:37.064  1795  1795 I /system/bin/tombstoned: registered intercept for pid 5463 and type kDebuggerdJavaBacktrace
10-29 17:37:40.409  1795  1795 I /system/bin/tombstoned: found intercept fd 512 for pid 5463 and type kDebuggerdJavaBacktrace
10-29 17:37:40.415  1955  1984 I system_server: libdebuggerd_client: done dumping process 5463
10-29 17:37:42.843  5743  5801 W libc    : Unable to set property "qemu.gles" to "1": connection failed; errno=13 (Permission denied)
10-29 17:37:57.715  6441  6441 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 17:38:08.625  1955  1984 I system_server: libdebuggerd_client: started dumping process 6441
10-29 17:38:08.660  1795  1795 I /system/bin/tombstoned: registered intercept for pid 6441 and type kDebuggerdJavaBacktrace
10-29 17:38:11.223  1795  1795 I /system/bin/tombstoned: found intercept fd 512 for pid 6441 and type kDebuggerdJavaBacktrace
10-29 17:38:11.229  1955  1984 I system_server: libdebuggerd_client: done dumping process 6441
10-29 17:38:18.109  1955  1984 I system_server: libdebuggerd_client: started dumping process 2604
10-29 17:38:18.122  1795  1795 I /system/bin/tombstoned: registered intercept for pid 2604 and type kDebuggerdJavaBacktrace
10-29 17:38:20.648  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:21.093  1795  1795 I /system/bin/tombstoned: found intercept fd 512 for pid 2604 and type kDebuggerdJavaBacktrace
10-29 17:38:21.331  1955  1984 I system_server: libdebuggerd_client: done dumping process 2604
10-29 17:38:21.872  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:23.781  6522  6522 I GAv4    : Google Analytics 17.4.49 is starting up. To enable debug logging on a device run:
10-29 17:38:23.781  6522  6522 I GAv4    :   adb shell setprop log.tag.GAv4 DEBUG
10-29 17:38:25.846  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:27.102  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:28.914  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:29.804  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:32.179  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:32.709  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:35.383  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:36.879  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:37.977  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:38.377  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:38.584  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:39.662  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:41.812  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:42.230  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:42.419  6595  6595 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 17:38:43.119  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:43.713  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:44.464  4140  6643 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 17:38:44.710  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:45.442  4140  6643 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 17:38:46.190  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:46.545  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:47.020  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:47.549  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:47.973  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:38:59.900  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:00.893  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:01.467  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:03.590  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:04.150  1955  1984 I system_server: libdebuggerd_client: started dumping process 6595
10-29 17:39:04.239  1795  1795 I /system/bin/tombstoned: registered intercept for pid 6595 and type kDebuggerdJavaBacktrace
10-29 17:39:04.249  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:04.303  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:04.942  1955  1983 W android.os.Debug: failed to get memory consumption info: -1
10-29 17:39:09.321  1795  1795 I /system/bin/tombstoned: found intercept fd 512 for pid 6595 and type kDebuggerdJavaBacktrace
10-29 17:39:09.381  1955  1984 I system_server: libdebuggerd_client: done dumping process 6595
10-29 17:39:20.464  1955  2091 E SupplicantStaIfaceHal: ISupplicantStaIface.setPowerSave failed: {.code = FAILURE_UNKNOWN, .debugMessage = }
10-29 17:39:21.450  6788  6788 D NetworkSecurityConfig: Using Network Security Config from resource network_security_config debugBuild: false
10-29 17:39:21.914  1955  2091 E SupplicantStaIfaceHal: ISupplicantStaIface.setPowerSave failed: {.code = FAILURE_UNKNOWN, .debugMessage = }
```

![01-launch](01-launch.png)

![02-painted](02-painted.png)

![04-layers](04-layers.png)
