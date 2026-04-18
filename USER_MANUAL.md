# QR Studio Ultra - User Manual

Welcome to **QR Studio Ultra**, the ultimate tool for creating high-quality, customizable, and branded QR codes. This manual provides comprehensive instructions on how to use the application features on Android and Android TV.

---

## 1. Getting Started

### Installation

1. Locate the `app-universal-release-signed.apk` in your downloads or project folder.
2. Transfer the APK to your Android device.
3. Open the file and follow the prompts to install.
   - _Note: You may need to enable "Install from Unknown Sources" in your device settings._

### Launching the App

Find the **QR Studio Ultra** icon on your home screen or app drawer and tap to open.

---

## 2. Generating Ultra QR Codes

The core of the app is the "Ultra" generator which allows for deep customization.

### Step-by-Step Creation:

1. **Enter Data:** Type the URL, text, or data you want the QR code to contain into the input field.
2. **Choose Colors:**
   - **Primary Color:** The main color of the QR code modules.
   - **Secondary Color:** Used if "Linear Gradient" is selected as the fill type.
   - **Background Color:** The color of the QR code canvas (Default is white).
3. **Select Shapes:**
   - **Main Shape:** Choose between _Square_, _Circle_, _Rounded_, or _Diamond_ for the data modules.
   - **Eye Shape:** Customize the three large squares (eyes) in the corners. Options include _Square_, _Circle_, _Rounded_, or _Diamond_.
4. **Gradient Fill:** Toggle between "Solid" and "Linear" gradient styles.
5. **Add a Logo:** (Optional) Upload a PNG or JPEG logo to be placed in the center of the QR code. The app automatically adds a safe white border around the logo for readability.

---

## 3. Saving to Your Device

Once you have generated your perfect QR code, you can save it to your device's gallery.

1. Tap the **Save to Gallery** button.
2. The image will be saved to your device's standard **Pictures/Gallery** folder (usually `/storage/emulated/0/Pictures`).
3. **Troubleshooting Save Issues:**
   - Ensure you have granted **Storage Permissions** when prompted.
   - If the file does not appear immediately, check your "Recently Added" album in the Gallery app.
   - _Current Version Note:_ We are working on adding a custom folder picker for even more control over your saves.

---

## 4. Barcode Scanning

The app includes a comprehensive barcode scanner that reads QR codes and various barcode formats.

### Supported Barcode Formats:

- **QR Codes** — Two-dimensional codes for URLs, text, WiFi, and more
- **Code 128** — Alphanumeric barcodes for shipping, inventory, and logistics
- **EAN** (European Article Number) — Retail product barcodes
- **UPC** (Universal Product Code) — Product identification codes
- **ISBN** (International Standard Book Number) — Book identification and library systems
- **Code 39** — Industrial and automotive barcode format
- **PDF417** — 2D barcodes for identity documents and tickets
- And additional formats through the Tauri barcode scanner plugin

### How to Scan:

1. Navigate to the **Scanner** tab.
2. Grant camera permissions if requested.
3. Point your camera at a QR code or barcode.
4. The app will automatically detect the data and format.
5. The app will provide options to open links or copy the detected data.

---

## 5. Android TV Support

QR Studio Ultra is optimized for large screens:

- **Navigation:** Use your TV remote's D-pad (directional keys) to navigate menus.
- **Leanback Launcher:** The app will appear directly in your TV's "Apps" row for easy access.

---

## 6. Frequently Asked Questions (FAQ)

### Why is my app icon looking different?

If your icon appears with a white background or the default Android robot, please ensure you have uninstalled any previous "debug" versions of the app before installing the latest release. Android sometimes caches old icons.

### Where are my QR codes saved?

They are saved in the `Pictures` directory. Look for files starting with `QR_Studio_` followed by a timestamp.

### Can I use the app offline?

Yes! QR code generation happens entirely on your device and does not require an active internet connection (unless you are opening an external link from a scan).

---

## 7. Support & Updates

For technical support or feature requests, please contact the development team at Cypher Studio.

_Version: 1.0.0_
_Developer: Paul Bess / Cypher Studio_
