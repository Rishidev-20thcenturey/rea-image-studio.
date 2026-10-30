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
# sys.boot_completed alone is not enough: on a device whose system server is still starting (or
# restarting) the property can already read 1 while the package, activity and window services are
# not published, and every later command fails with "Can't find service". Wait for the package
# manager to actually answer before touching anything.
wait_for_system() {
    n=0
    while [ "$n" -lt 90 ]; do
        n=$((n + 1))
        if adb shell getprop sys.boot_completed 2>/dev/null | grep -q 1; then
            if adb shell cmd package list packages 2>/dev/null | grep -q "package:"; then
                say "system services ready after $n checks"
                return 0
            fi
            # Booted, but the system server is not publishing services yet.
            if [ "$n" -gt 30 ]; then
                say "boot_completed is set but the package service never answered"
                return 1
            fi
        fi
        sleep 5
    done
    say "timed out waiting for the system to be ready"
    return 1
}
wait_for_system
say "sdk: $(adb shell getprop ro.build.version.sdk 2>&1 | tr -d '\r')"
say "abi: $(adb shell getprop ro.product.cpu.abi 2>&1 | tr -d '\r')"
# The stock AVD is 320x640, which is not a screen anyone edits a photo on, and touch coordinates
# have to be inside it. Ask for a tablet-sized display first, then read back what was granted.
adb shell wm size 1280x800 >> "$OUT" 2>&1
adb shell wm density 240 >> "$OUT" 2>&1
SIZE_LINE="$(adb shell wm size 2>&1 | tr -d '\r' | grep -o '[0-9]*x[0-9]*' | tail -1)"
SCREEN_W="${SIZE_LINE%x*}"
SCREEN_H="${SIZE_LINE#*x}"
case "$SCREEN_W" in ''|*[!0-9]*) SCREEN_W=1080 ;; esac
case "$SCREEN_H" in ''|*[!0-9]*) SCREEN_H=1920 ;; esac
say "size: ${SCREEN_W}x${SCREEN_H}"
say "page size: $(adb shell getconf PAGE_SIZE 2>&1 | tr -d '\r')"
say "apk: $(stat -c%s "$APK" 2>/dev/null || echo missing) bytes"

# A software-emulated device throws "System UI isn't responding" dialogs that swallow touch events.
# They are an artefact of the emulator, not of the app, but they made the previous run's strokes
# land on a dialog instead of the canvas.
adb shell settings put global hide_error_dialogs 1 >> "$OUT" 2>&1

say ""
say "== install =="
if ! adb install -r "$APK" >> "$OUT" 2>&1; then
    say "first install attempt failed; waiting and retrying once"
    wait_for_system
    adb install -r "$APK" >> "$OUT" 2>&1
fi
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

# Make sure the activity, not a system dialog, has focus before any touch is sent.
# A system dialog that holds focus swallows every injected touch. Close whatever is on top first.
adb shell am broadcast -a android.intent.action.CLOSE_SYSTEM_DIALOGS >> "$OUT" 2>&1
adb shell input keyevent 4 >> "$OUT" 2>&1
sleep 2
ensure_focus() {
    FOCUS=$(adb shell dumpsys window 2>/dev/null | grep -m1 mCurrentFocus)
    say "focus: $FOCUS"
    case "$FOCUS" in
        *"$PKG"*) return 0 ;;
    esac
    say "the app did not have focus; dismissing dialogs and restarting it"
    adb shell input keyevent 4 >> "$OUT" 2>&1
    adb shell am start -n "$PKG/.MainActivity" >> "$OUT" 2>&1
    sleep 10
    say "focus now: $(adb shell dumpsys window 2>/dev/null | grep -m1 mCurrentFocus)"
}
ensure_focus

# Real touch events on the canvas. The tool rail sits on the left edge, so keep clear of it.
say ""
say "== draw three strokes =="
# Fractions of the screen: the rail is at the far left and the top bar along the top, so the strokes
# stay on the canvas in the middle.
adb shell input swipe "$((SCREEN_W * 3 / 10))" "$((SCREEN_H * 4 / 10))" \
    "$((SCREEN_W * 7 / 10))" "$((SCREEN_H * 6 / 10))" 600 >> "$OUT" 2>&1
adb shell input swipe "$((SCREEN_W * 7 / 10))" "$((SCREEN_H * 6 / 10))" \
    "$((SCREEN_W * 4 / 10))" "$((SCREEN_H * 7 / 10))" 600 >> "$OUT" 2>&1
adb shell input swipe "$((SCREEN_W * 5 / 10))" "$((SCREEN_H * 3 / 10))" \
    "$((SCREEN_W * 6 / 10))" "$((SCREEN_H * 8 / 10))" 600 >> "$OUT" 2>&1
sleep 3
shot 02-painted

say ""
say "== switch to the eraser from the rail, then undo =="
adb shell input tap 30 "$((SCREEN_H / 2 + 60))" >> "$OUT" 2>&1
sleep 2
shot 03-tool
# Undo lives at the right end of the top bar.
adb shell input tap "$((SCREEN_W - 150))" 40 >> "$OUT" 2>&1
sleep 3

say ""
say "== open the layer panel =="
adb shell input tap "$((SCREEN_W - 60))" 40 >> "$OUT" 2>&1
sleep 4
shot 04-layers
adb shell input keyevent 4 >> "$OUT" 2>&1
sleep 2

say ""
say "== open the menu =="
adb shell input tap 40 40 >> "$OUT" 2>&1
sleep 4
shot 05-menu
adb shell input keyevent 4 >> "$OUT" 2>&1
sleep 2

say ""
say "== the interface's own end-to-end check =="
# Injected touches are unreliable here (the emulator's system UI holds focus), so the app is asked
# to drive its own view: real MotionEvents dispatched into the canvas, then pixels read back out of
# the core. The result is written to files/diag/ui-selftest.txt.
adb shell am force-stop "$PKG" >> "$OUT" 2>&1
adb shell am start -n "$PKG/.MainActivity" --ez rea.selftest true >> "$OUT" 2>&1
sleep 20
adb shell run-as "$PKG" cat files/diag/ui-selftest.txt >> "$OUT" 2>&1
shot 06-selftest

say ""
say "== the app's own boot log ==="
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
