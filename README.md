# win2d_fail
failure to use Win2D.uwp nuget package for winrt-rs

```
C:\Projects\Rust\win2d_fail>cargo winrt run
    Fetching Win2D.uwp
    Finished in 2.82s
   Compiling proc-macro2 v1.0.18
   Compiling unicode-xid v0.2.1
   Compiling serde v1.0.114
   Compiling syn v1.0.34
   Compiling ryu v1.0.5
   Compiling serde_json v1.0.56
   Compiling itoa v0.4.6
   Compiling sha1 v0.6.0
   Compiling quote v1.0.7
   Compiling winrt_gen_macros v0.7.1
   Compiling winrt_gen v0.7.1
   Compiling winrt_macros v0.7.1
   Compiling winrt v0.7.1
   Compiling win2d_fail v0.1.0 (C:\Projects\Rust\win2d_fail)
    Finished dev [unoptimized + debuginfo] target(s) in 12.05s
     Running `target\debug\win2d_fail.exe`
Error: Error { code: 0x80040154, message: "Class not registered" }
error: process didn't exit successfully: `target\debug\win2d_fail.exe` (exit code: 1)
```
