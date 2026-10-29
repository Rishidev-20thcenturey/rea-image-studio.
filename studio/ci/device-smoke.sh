#!/bin/sh
# Drive REA Studio on a running emulator and record what happened.
#
# Run by .github/workflows/studio-smoke.yml inside ReactiveCircus/android-emulator-runner, which
# executes its `script:` with /bin/sh (dash on the runner). So: POSIX only, no pipefail, no strict
# mode, and every command is allowed to fail - a failure in one probe must not lose the rest of the
# evidence. `sh -n` on this file must stay clean.
#
# Everything is written to $RUNNER_TEMP/smoke.txt as it happens, so a run that dies halfway still
# leaves a trail. Screenshots go to $RUNNER_TEMP/*.png.

set +e

TEMP="${RUNNER_TEMP:-/tmp}"
OUT="$TEMP/smoke.txt"
PKG="com.rea.studio"
APK="$TEMP/rea-studio-x86_64.apk"

: > "$OUT"
say() {
    echo "$*" >> "$OUT"
    echo "$*"
}

shot() {
    # shot <name> - never fails the script
    adb exec-out screencap -p > "$TEMP/$1.png" 2>/dev/null
    say "screenshot $1: $(stat -c%s "$TEMP/$1.png" 2>/dev/null || echo 0) bytes"
}

say "== device =="
adb devices >> "$OUT" 2>&1
for i in 1 2 3 4 5 6 7 8 9 10 11 12; do
    if adb shell getprop sys.boot_completed 2>/dev/null | grep -q 1; then
        say "booted after $i checks"
        break
    fi
    sleep 5
done
say "sdk: $(adb shell getprop ro.build.version.sdk 2>&1 | tr -d '\r')"
say "abi: $(adb shell getprop ro.product.cpu.abi 2>&1 | tr -d '\r')"
say "size: $(adb shell wm size 2>&1 | tr -d '\r')"
say "page size: $(adb shell getconf PAGE_SIZE 2>&1 | tr -d '\r')"
say "apk: $(stat -c%s "$APK" 2>/dev/null || echo missing) bytes"

say ""
say "== install =="
adb install -r "$APK" >> "$OUT" 2>&1
if adb shell pm path "$PKG" >> "$OUT" 2>&1; then
    say "installed"
else
    say "NOT installed"
fi

say ""
say "== launch =="
adb logcat -c >> "$OUT" 2>&1
adb shell am start -W -n "$PKG/.MainActivity" >> "$OUT" 2>&1
sleep 15

say ""
say "== first frame =="
shot 01-launch

# Real touch events on the canvas. The tool rail sits on the left edge, so keep clear of it.
say ""
say "== draw three strokes =="
adb shell input swipe 500 900 1300 1300 600 >> "$OUT" 2>&1
adb shell input swipe 1300 1300 700 1700 600 >> "$OUT" 2>&1
adb shell input swipe 900 700 1100 1900 600 >> "$OUT" 2>&1
sleep 3
shot 02-painted

say ""
say "== undo, then the menu =="
adb shell input tap 660 120 >> "$OUT" 2>&1
sleep 2
adb shell input tap 40 40 >> "$OUT" 2>&1
sleep 4
shot 03-menu
adb shell input keyevent 4 >> "$OUT" 2>&1
sleep 2

say ""
say "== the app's own boot log =="
adb shell run-as "$PKG" cat files/diag/boot.log >> "$OUT" 2>&1
if adb shell run-as "$PKG" cat files/diag/crash.txt >> "$OUT" 2>&1; then
    say "(a recorded exception is printed above)"
fi

say ""
say "== process =="
if adb shell ps -A 2>/dev/null | grep -qi "$PKG"; then
    adb shell ps -A 2>/dev/null | grep -i "$PKG" >> "$OUT" 2>&1
    say "the app process is running"
else
    say "the app process is NOT running"
fi

say ""
say "== activity state =="
adb shell dumpsys activity activities 2>/dev/null | grep -iE "$PKG|mResumedActivity" | head -10 >> "$OUT" 2>&1

say ""
say "== crash, ANR and JNI errors in logcat =="
adb logcat -d 2>/dev/null \
    | grep -iE "FATAL EXCEPTION|AndroidRuntime|ANR in|UnsatisfiedLink|dlopen|$PKG" \
    | head -80 >> "$OUT" 2>&1
if [ "$(wc -l < "$OUT")" -lt 3 ]; then
    say "(no matching log lines)"
fi

adb logcat -d 2>/dev/null | tail -n 500 > "$TEMP/logcat.txt"

say ""
say "== result =="
# The app writes files/diag/boot-ok once the interface is up. Its absence means the launch failed,
# and boot.log above says which step it stopped at.
if adb shell run-as "$PKG" cat files/diag/boot-ok >> "$OUT" 2>&1; then
    say "RESULT: the app reached a drawn interface"
else
    say "RESULT: no boot-complete marker - the app did not finish starting"
fi
say "evidence: $OUT"
