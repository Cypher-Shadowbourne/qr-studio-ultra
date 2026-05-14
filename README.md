![Cypher Shadowbourne banner](docs/assets/cypher-shadowbourne-banner.png)

# QR Studio Ultra

![Platform](https://img.shields.io/badge/Platform-Desktop_%2B_Android-3b82f6)
![Framework](https://img.shields.io/badge/Framework-Tauri_2.0-FFC131?logo=tauri&logoColor=white)
![Frontend](https://img.shields.io/badge/Frontend-Svelte-FF3E00?logo=svelte&logoColor=white)
![Backend](https://img.shields.io/badge/Backend-Rust-000000?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-111827)

QR Studio Ultra is a privacy-first QR and barcode studio built with Svelte, Tauri, and a native Rust rendering engine.

It is designed for people who want more than a basic black-and-white QR utility. The app focuses on polished visual output, practical payload generation, offline use, and native-quality exports without sending data to outside services.

> Android build artifacts are kept outside the repo. The source here is the canonical project state.

## Screenshots

|                                                                   |                                                                   |
| ----------------------------------------------------------------- | ----------------------------------------------------------------- |
| ![QR Studio Ultra screenshot 1](docs/screenshots/gallery-01.png)  | ![QR Studio Ultra screenshot 2](docs/screenshots/gallery-02.png)  |
| ![QR Studio Ultra screenshot 3](docs/screenshots/gallery-03.png)  | ![QR Studio Ultra screenshot 4](docs/screenshots/gallery-04.png)  |
| ![QR Studio Ultra screenshot 5](docs/screenshots/gallery-05.png)  | ![QR Studio Ultra screenshot 6](docs/screenshots/gallery-06.png)  |
| ![QR Studio Ultra screenshot 7](docs/screenshots/gallery-07.png)  | ![QR Studio Ultra screenshot 8](docs/screenshots/gallery-08.png)  |
| ![QR Studio Ultra screenshot 9](docs/screenshots/gallery-09.png)  | ![QR Studio Ultra screenshot 10](docs/screenshots/gallery-10.png) |
| ![QR Studio Ultra screenshot 11](docs/screenshots/gallery-11.png) | ![QR Studio Ultra screenshot 12](docs/screenshots/gallery-12.png) |
| ![QR Studio Ultra screenshot 13](docs/screenshots/gallery-13.png) | ![QR Studio Ultra screenshot 14](docs/screenshots/gallery-14.png) |

## Quick Start

If you want to run the project locally right away:

```bash
npm install
npm run tauri dev
```

If you only want the web UI during development:

```bash
npm run dev
```

## Release Picks

Current packaged Android builds in this workspace:

| Build                           | Best for                           |
| ------------------------------- | ---------------------------------- |
| external Android package export | device testing and release sharing |

Desktop development runs through Tauri with:

```bash
npm run tauri dev
```

## What It Does

QR Studio Ultra currently supports:

- Custom QR generation with solid or multi-stop gradients
- Styled finder eyes and module shapes
- Frame and ring overlays
- Center logo upload with crop, size, and opacity controls
- QR and barcode scanning
- Encrypted QR payload generation and decryption
- Wallet/payment QR creation with saved wallet profiles
- Common payload presets like URL, Wi-Fi, vCard, email, SMS, phone, geo, event, social links, and more
- Desktop Tauri use and Android packaging

## Feature Matrix

| Area       | Included                                                                                        |
| ---------- | ----------------------------------------------------------------------------------------------- |
| QR styling | solid fills, multi-stop gradients, custom module shapes, styled eyes                            |
| Logos      | upload, crop, center placement, size control, opacity control                                   |
| Scanner    | QR, Code 128, EAN, UPC, ISBN, Code 39, PDF417 and more; square targeting UI, animated scan line |
| Payloads   | URL, Wi-Fi, vCard, email, SMS, phone, geo, event, social, crypto and more                       |
| Encryption | passphrase-protected QR payloads (ChaCha20-Poly1305, AES-256-GCM), local-only processing        |
| Crypto     | wallet profiles, richer payment URI generation, optional amount/label/message                   |
| Platforms  | Tauri desktop workflow, Android builds, native save/share integration                           |
| Rendering  | Rust-backed QR generation and logo compositing                                                  |

## Why This Project Exists

Most QR apps are either too plain, too intrusive, or too careless about quality.

QR Studio Ultra takes a different route:

- generation stays local
- encryption stays on-device
- rendering quality matters
- exports are meant to look intentional
- scanning reliability is treated as a product concern, not an afterthought

The app uses a Rust backend for the heavy rendering work and a Svelte frontend for fast interaction and UI flexibility.

## Highlights

### Native Rust Rendering

The QR image itself is rendered in Rust instead of relying entirely on a browser-only drawing path.

That gives the project tighter control over:

- module shaping
- finder eye rendering
- gradient fills
- logo compositing
- image export quality

Recent rendering work also moved toward cleaner direct-module sampling, which noticeably improved edge quality and reduced visual artifacts.

### Logo Workflow That Respects the QR

Center logos are handled with a crop-first workflow in the frontend and then composited natively in Rust.

Current logo features include:

- crop and framing flow before generation
- adjustable logo size
- adjustable logo opacity
- centered compositing with a protected backing plate to keep the code more scan-safe

### Scanner Support Beyond QR

The scanner is not limited to QR codes.

The app can request a wider set of barcode formats through the Tauri barcode scanner plugin, and the scanner UI includes:

- square target aperture
- animated scan line
- result type/format display

**Supported Barcode Formats:**

- **Code 128** — Alphanumeric barcodes commonly used for shipping labels and inventory
- **EAN** (European Article Number) — Standard retail barcodes for products
- **UPC** (Universal Product Code) — North American product identification codes
- **ISBN** (International Standard Book Number) — Book identification codes
- **Code 39** — Alphanumeric codes used in various industries
- **PDF417** — 2D barcodes for documents and identity verification
- And additional formats through the Tauri barcode scanner plugin

### Wallet and Payment Support

The crypto flow has moved beyond a single address field.

It now supports:

- multiple wallet networks
- saved wallet profiles
- optional amount, label, and message fields
- richer URI generation for supported networks

### Encrypted Payloads

QR Studio Ultra includes a built-in engine for creating and reading encrypted QR codes.

Key security features:

- **Local Processing**: Encryption and decryption happen entirely on-device in the Rust backend.
- **Strong Algorithms**: Supports **ChaCha20-Poly1305** (default) and **AES-256-GCM**.
- **Key Derivation**: Uses **Argon2id** for robust password-based key derivation.
- **Portable Format**: Encrypted payloads use a custom `QRU1:` prefix, allowing the app to automatically detect and prompt for passphrases when scanning.

## Showcase Notes

The app is meant to cover two use cases that usually get split across multiple tools:

- a fast everyday QR utility that works locally
- a more polished studio for branded or visually customized codes

That is why the project mixes:

- native rendering work
- payload-oriented utility features
- visual customization controls
- Android practicality

The result is intentionally a little more ambitious than a standard generator.

## Tech Stack

- Frontend: Svelte 5, TypeScript, Canvas
- Native shell: Tauri 2.0
- Backend: Rust
- Package Manager: npm
- QR engine: `fast_qr`
- Image processing: `image`
- Native plugins: barcode scanner, dialog, shell/opener, opener

## Scripts

The following scripts are available in `package.json`:

| Script | Description |
| :--- | :--- |
| `npm run dev` | Starts the Vite development server for the web UI. |
| `npm run build` | Builds the frontend for production. |
| `npm run preview` | Previews the production build locally. |
| `npm run check` | Runs Svelte-check and syncs Svelte-Kit. |
| `npm run check:watch` | Runs Svelte-check in watch mode. |
| `npm run test` | Runs frontend tests using Vitest. |
| `npm run test:watch` | Runs frontend tests in watch mode. |
| `npm run test:rust` | Runs Rust backend tests using Cargo. |
| `npm run tauri` | Wrapper for the Tauri CLI. |

To run the app in development mode (Desktop):
```bash
npm run tauri dev
```

To build the app for production (Desktop):
```bash
npm run tauri build
```

## Environment Variables

| Variable | Description | Required |
| :--- | :--- | :--- |
| `TODO` | Identify any required environment variables for CI/CD or specialized builds. | No |

*Note: For Android builds, sensitive information is typically handled via `keystore.properties` (see `keystore.properties.example`).*

## Tests

### Frontend Tests
The frontend uses **Vitest** for unit and component testing.
```bash
npm run test
```

### Backend Tests
The Rust backend uses standard **Cargo** tests.
```bash
npm run test:rust
```

## Project Structure

Key areas of the repo:

- `src/`
  Frontend source code (Svelte 5).
  - `src/routes/+page.svelte`: Main app UI, state, QR options, overlays, scanner UX, wallet flow, and encryption/decryption views.
- `src-tauri/`
  Native backend and configuration (Tauri/Rust).
  - `src-tauri/src/lib.rs`: Native commands, QR rendering, image composition, mobile/desktop save logic.
  - `src-tauri/tauri.conf.json`: Tauri app configuration.
  - `src-tauri/gen/android`: Generated Android project files and Android-specific patches.
- `static/`
  Static assets for the frontend.
- `docs/`
  Project documentation and screenshots.

## Running the Project

### Prerequisites

You will want:

- Node.js
- Rust toolchain
- Tauri prerequisites
- Android Studio / Android SDK if building for Android

Install dependencies:

```bash
npm install
```

### Web Dev

```bash
npm run dev
```

### Desktop Tauri Dev

```bash
npm run tauri dev
```

### Checks

```bash
npm run check
```

### Production Web Build

```bash
npm run build
```

## Typical Workflows

### Desktop Use

Best for:

- quick design iteration
- testing gradients and overlays
- checking logo placement
- working on the Svelte UI

Command:

```bash
npm run tauri dev
```

### Android Build and Device Testing

Best for:

- scanner validation on real cameras
- save/share testing
- performance checks on physical hardware
- verifying packaging for different Android CPU targets

Example:

```bash
npx tauri android build --target aarch64
```

## Android Builds

Example Android build command:

```bash
npx tauri android build --target aarch64
```

This repo has also been used to produce:

- an Android release APK for newer devices
- a separate `armeabi-v7a` build for older devices

If you are working on Android packaging, also inspect the generated Android files in `src-tauri/gen/android`, because this project includes local fixes and compatibility tweaks there.

## Android Notes

This project has needed a few practical Android-side adjustments for real builds, including:

- Kotlin plugin compatibility fixes in generated Gradle modules
- JVM target compatibility tweaks
- SDK version alignment

Those changes are part of why the repo includes generated Android project files instead of treating them as completely disposable.

## Product Direction

QR Studio Ultra is trying to sit in a useful middle ground:

- more polished than a throwaway utility
- more private than ad-heavy online tools
- more expressive than default QR libraries
- still grounded in real scan behavior

That means flashy ideas are welcome, but they need to earn their place by staying usable.

## Roadmap Flavor

The strongest future work for this project usually falls into one of these buckets:

- better visual polish that still scans cleanly
- stronger export and print workflows
- broader payload support
- cleaner native Android behavior
- refined desktop creator experience

That combination is where the app feels most distinctive.

## Contributing

Contributions are welcome.

Start here:

- `CONTRIBUTING.md`

If you are proposing rendering changes, scanner changes, or new styling systems, please read that guide first so the work lines up with the project’s priorities.

## License

This project is released under the `MIT` License.
