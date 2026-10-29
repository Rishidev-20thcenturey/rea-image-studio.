<!-- Written by .github/workflows/studio-smoke.yml. Do not edit by hand. -->
# Studio app: emulator smoke test

Run [40910643612](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40910643612), commit `d0147b0ab34a6772ad15616f449f6c69d469e328`, 2026-10-29T16:27:05Z.
x86_64 build of the same code the arm64 APK ships.

```
(the smoke test did not reach the install step)
```

## Device log (filtered)

```
10-29 16:26:54.107  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.108  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.109  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.110  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.110  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.111  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.480  2934  2934 I crash_dump64: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:26:54.567  1796  1796 I /system/bin/tombstoned: found intercept fd 512 for pid 1712 and type kDebuggerdNativeBacktrace
10-29 16:26:54.758  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.759  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.760  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.762  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.762  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.763  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.763  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.905  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.906  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.907  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.908  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.909  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:54.910  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:55.225  1947  1970 E system_server: libdebuggerd_client: timeout expired
10-29 16:26:55.252  1947  1970 I system_server: libdebuggerd_client: started dumping process 1697
10-29 16:26:55.259  1796  1796 I /system/bin/tombstoned: registered intercept for pid 1697 and type kDebuggerdNativeBacktrace
10-29 16:26:55.262  1697  1697 I libc    : Requested dump for tid 1697 (healthd)
10-29 16:26:54.911  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:55.671  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:56.297  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:56.298  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:56.300  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:56.301  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:56.302  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:56.303  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.085  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.086  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.088  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.243  2951  2951 I crash_dump64: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:26:57.317  1796  1796 I /system/bin/tombstoned: found intercept fd 512 for pid 1697 and type kDebuggerdNativeBacktrace
10-29 16:26:57.318  1947  1970 E system_server: libdebuggerd_client: timeout expired
10-29 16:26:57.347  1947  1970 I system_server: libdebuggerd_client: started dumping process 1698
10-29 16:26:57.349  1796  1796 I /system/bin/tombstoned: registered intercept for pid 1698 and type kDebuggerdNativeBacktrace
10-29 16:26:57.359  1698  1698 I libc    : Requested dump for tid 1698 (audio@2.0-servi)
10-29 16:26:57.692  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.859  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.860  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.861  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.861  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.862  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.862  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.863  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:57.863  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:58.513  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:58.691  2959  2959 I crash_dump32: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:26:58.706  1796  1796 I /system/bin/tombstoned: found intercept fd 512 for pid 1698 and type kDebuggerdNativeBacktrace
10-29 16:26:58.787  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:58.788  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:58.789  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:58.789  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:58.793  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:26:59.356  1947  1970 E system_server: libdebuggerd_client: timeout expired
10-29 16:26:59.399  1947  1970 I system_server: libdebuggerd_client: started dumping process 1700
10-29 16:26:59.401  1796  1796 I /system/bin/tombstoned: registered intercept for pid 1700 and type kDebuggerdNativeBacktrace
10-29 16:26:59.403  1700  1700 I libc    : Requested dump for tid 1700 (provider@2.4-se)
10-29 16:26:59.917  2969  2969 I crash_dump32: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:26:59.919  1796  1796 I /system/bin/tombstoned: found intercept fd 512 for pid 1700 and type kDebuggerdNativeBacktrace
10-29 16:27:00.405  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:27:00.752  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:27:01.047  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:27:01.225  1947  1970 I system_server: libdebuggerd_client: done dumping process 1700
10-29 16:27:01.317  1947  1970 I system_server: libdebuggerd_client: started dumping process 1717
10-29 16:27:01.478  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:27:01.488  1796  1796 I /system/bin/tombstoned: registered intercept for pid 1717 and type kDebuggerdNativeBacktrace
10-29 16:27:01.506  1717  1717 I libc    : Requested dump for tid 1717 (sensors@1.0-ser)
10-29 16:27:02.955  1947  1972 W android.os.Debug: failed to get memory consumption info: -1
10-29 16:27:03.119  2982  2982 I crash_dump64: obtaining output fd from tombstoned, type: kDebuggerdNativeBacktrace
10-29 16:27:03.200  1796  1796 I /system/bin/tombstoned: found intercept fd 512 for pid 1717 and type kDebuggerdNativeBacktrace
10-29 16:27:03.324  1947  1970 E system_server: libdebuggerd_client: timeout expired
10-29 16:27:03.347  1947  1970 I system_server: libdebuggerd_client: started dumping process 1784
10-29 16:27:03.349  1796  1796 I /system/bin/tombstoned: registered intercept for pid 1784 and type kDebuggerdNativeBacktrace
10-29 16:27:04.276  1796  1796 I /system/bin/tombstoned: found intercept fd 512 for pid 1784 and type kDebuggerdNativeBacktrace
```
