<!-- Written by .github/workflows/emulator-smoke.yml. Do not edit by hand. -->
# Emulator smoke test

Run [40879383419](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40879383419), commit `854eea62425f9fa1b39d56c0623cf9ed2f1eb132`, 2026-10-29T13:22:53Z.
x86_64 build on an Android API 35 emulator; the shipped APK is the same code built for arm64-v8a.

```
== environment ==
ANDROID_HOME=/home/runner/android-sdk
java: openjdk version "17.0.21" 2026-10-20
emulator: /home/runner/android-sdk/emulator/emulator
avdmanager: /home/runner/android-sdk/cmdline-tools/latest/bin/avdmanager
adb: /home/runner/android-sdk/platform-tools/adb
kvm: crw-rw---- 1 root kvm 10, 232 Oct 29 12:43 /dev/kvm

== create the AVD ==
Loading local repository...                                                     Warning: This version only understands SDK XML versions up to 3 but an SDK XML file of version 4 was encountered. This can happen if you use versions of Android Studio and the command-line tools that were released at different times.
[=========                              ] 25% Loading local repository...       [=========                              ] 25% Fetch remote repository...        [=======================================] 100% Fetch remote repository...       
Do you wish to create a custom hardware profile? [no] avdmanager exit: 0
$ avdmanager list avd
Available Android Virtual Devices:
    Name: rea-smoke
    Path: /home/runner/.config/.android/avd/rea-smoke.avd
  Target: Google APIs (Google Inc.)
          Based on: Android API 35 Tag/ABI: google_apis/x86_64
  Sdcard: 512 MB
(exit 0)
avd config: NOT FOUND

== start the emulator ==
emulator pid: 12522
$ adb devices
List of devices attached

(exit 0)
== the device never appeared: emulator log (tail) ==
INFO         | Android emulator version 37.2.12.0 (build_id 16428233) (CL:N/A)
INFO         | Graphics backend: gfxstream
ERROR        | Unknown AVD name [rea-smoke], use -list-avds to see valid list.
ERROR        | HOME is defined but there is no file rea-smoke.ini in $HOME/.android/avd
ERROR        | (Note: Directories are searched in the order $ANDROID_AVD_HOME, $ANDROID_SDK_HOME/avd and $HOME/.android/avd)
::error title=Emulator did not start::see docs/ci/emulator/last-smoke.txt
```
