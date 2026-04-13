# QR Studio Ultra 📱🚀

![Platform](https://img.shields.io/badge/Platform-Android-3DDC84?logo=android&logoColor=white)
![Framework](https://img.shields.io/badge/Framework-Tauri_2.0-FFC131?logo=tauri&logoColor=white)
![Frontend](https://img.shields.io/badge/Frontend-Svelte-FF3E00?logo=svelte&logoColor=white)
![Backend](https://img.shields.io/badge/Backend-Rust-000000?logo=rust&logoColor=white)

**QR Studio Ultra** is a premium, privacy-first, offline QR code generator built with a truly hybrid architecture. Designed to replace generic, ad-filled utility apps, it features a custom-built pixel rendering engine in Rust and a lightning-fast Svelte frontend.

> **Download the latest Android APK from the [Releases Tab](../../releases).**

---

## ✨ The "Ultra" Difference

Most hybrid QR generators rely on generic JavaScript libraries that paint a basic black-and-white grid to an HTML canvas. **QR Studio Ultra does not.** By bridging the DOM to a compiled ARM64 Rust backend, this app handles the heavy mathematical lifting natively, enabling commercial-grade aesthetic customization without sacrificing speed or memory safety.

### 🦀 Custom Rust Rendering Engine
* **Pixel-Perfect Math:** Calculates radial boundaries to dynamically carve out circular, rounded, or diamond Finder Patterns (eyes) and data modules.
* **Native Image Processing:** Injects and composites center logos natively in Rust using the `image` crate with Lanczos3 filtering.
* **Memory Safe:** Implements strict bounding-box enforcement using `.saturating_sub()` and `.min()` to absolutely prevent integer underflows and out-of-bounds panics during logo overlay generation.

### 🖼️ Smart Auto-Cropper (Canvas Optimized)
Uploading a 12-megapixel photo to a mobile backend creates massive Base64 bottlenecks. QR Studio Ultra solves this with an interactive frontend crop modal.
* **Zero Double-Decoding:** Uses HTML5 Canvas to synchronously pan, zoom, and crop the image *before* it leaves the frontend.
* **Payload Reduction:** Shrinks user logos to a strict 200x200 optimized PNG, reducing the data payload sent to Rust by over 99% for instantaneous generation.

### 🇮🇪 Localized Legal Compliance
Features specialized UX flows, including an "Irish Dog Tag" generator that validates the 15-digit microchip requirement under Irish law, utilizing custom warning modals without hard-blocking user intent.

### 🔒 100% Privacy First
Zero external tracking. Zero analytics. QR codes are generated entirely locally on-device. On Android, generated codes save into a normal shared folder at `Downloads/QR Studio Ultra` so users can find them without digging through app-private storage.

---

## 🛠️ Tech Stack

* **Frontend:** Svelte, HTML5 Canvas, TypeScript
* **Backend:** Rust (`fast_qr`, `image` crates)
* **Bridge & Build:** Tauri 2.0 (Mobile)
* **Native Integrations:** `@tauri-apps/plugin-barcode-scanner`

---

## 🚀 Build Instructions

If you want to clone this repository and build it yourself, ensure you have the Rust toolchain, Node.js, and the Android Studio SDK installed.

```bash
# 1. Clone the repository
git clone https://github.com/Cypher-Shadowbourne/qr-studio-ultra.git
cd qr-studio-ultra

# 2. Install frontend dependencies
npm install

# 3. Build the Android APK (requires connected device or emulator)
npx tauri android build --target aarch64
```

## 🛠️ Build Patches & Troubleshooting

This project includes specific patches in the `src-tauri/gen/android` directory to resolve common Tauri 2.0 + Kotlin compatibility issues:

### 1. Missing Kotlin Plugin in Dependencies
Certain registry-based dependencies (like `tauri-android` and `tauri-plugin-shell`) may fail to apply the Kotlin Android plugin. We've patched the root `build.gradle.kts` to inject the plugin into all library modules automatically:
```kotlin
project.plugins.withId("com.android.library") {
    if (!project.plugins.hasPlugin("org.jetbrains.kotlin.android")) {
        project.plugins.apply("org.jetbrains.kotlin.android")
    }
}
```

### 2. JVM Target Compatibility
To resolve "Inconsistent JVM-target compatibility" errors, the build is forced to use `JVM 1.8` across all Kotlin tasks and the application module:
```kotlin
kotlinOptions {
    jvmTarget = "1.8"
}
```

### 3. SDK Versioning
The project is configured to use **Compile SDK 36** and **Target SDK 36** to ensure compatibility with the latest Android features and security standards.
