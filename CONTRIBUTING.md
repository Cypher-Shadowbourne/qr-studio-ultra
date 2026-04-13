# Contributing to QR Studio Ultra

Thanks for taking an interest in QR Studio Ultra.

This project is building toward something more ambitious than a generic QR utility: a privacy-first, offline-first QR studio with a native Rust renderer, a fast Svelte interface, and a strong bias toward polished output that still scans reliably in the real world.

If you want to help with that mission, you are very welcome here.

## Project Priorities

Contributions are most useful when they strengthen one or more of these goals:

- Better QR rendering quality without hurting scan reliability
- Cleaner export and save workflows on desktop and Android
- More useful payload types and practical generation presets
- Better scanner support, especially across real device conditions
- Thoughtful UI polish that feels intentional rather than generic
- Stability, performance, and maintainability across the Tauri + Svelte + Rust stack

## Before You Start

Please open an issue first for:

- New features
- Larger UI redesigns
- Rendering engine changes
- New Android-specific behavior
- Changes that affect QR scannability or encoding rules

For small fixes, docs updates, typo fixes, or focused cleanup, you can usually go straight to a pull request.

## Development Setup

Make sure you have:

- Node.js
- Rust toolchain
- Tauri 2.x prerequisites
- Android Studio / Android SDK if you plan to work on mobile builds

Install dependencies:

```bash
npm install
```

Useful commands:

```bash
# Type and Svelte checks
npm run check

# Web dev server
npm run dev

# Production web build
npm run build

# Tauri desktop dev
npm run tauri dev
```

Android contributors should also read the build notes and patch details in [README.md](/c:/Users/pbess/Desktop/qr-studio-ultra-development/README.md).

## Workflow

1. Fork the repository and create a focused branch.
2. Keep the scope tight. One fix or one feature per branch is ideal.
3. Make your changes with real-world use in mind, not just visual novelty.
4. Run checks before opening a pull request.
5. Write a clear PR description that explains what changed and why.

Example branch names:

- `fix/logo-opacity-rendering`
- `feat/new-barcode-format-support`
- `docs/improve-contributing-guide`

## Quality Bar

Please aim for the following:

- Changes should be understandable without detective work
- UI additions should fit the existing product direction
- Fancy styling should never come at the cost of scan reliability
- Mobile-facing changes should be conservative about performance and memory use
- Rust changes should stay defensive around bounds, image sizes, and platform behavior

If your change affects rendering, scanning, or exports, include notes about how you tested it.

## Good Contribution Areas

Strong contribution ideas include:

- New payload generators
- Better wallet/payment URI support
- Barcode and scanner improvements
- Safer or cleaner Android save/share flows
- Export quality improvements
- Better accessibility and usability
- Visual polish that still feels premium and practical
- Bug fixes in the Rust renderer or Svelte UI

## Pull Requests

A strong PR usually includes:

- A short summary of the problem
- The approach you took
- Screenshots or screen recordings for visible UI changes
- Notes about testing
- Any limitations, tradeoffs, or follow-up ideas

Please keep PRs focused. Smaller, well-explained pull requests are much easier to review and merge.

## Reporting Bugs

When opening a bug report, please include as much of this as you can:

- Device or platform
- Desktop or Android
- Steps to reproduce
- Expected behavior
- Actual behavior
- Screenshots if relevant
- Example payload or QR settings if the issue is rendering-related

If the bug is about scannability, mention:

- Which QR style was used
- Whether a logo was enabled
- What scanner app or camera was used
- Whether the issue happens consistently or only sometimes

## Design Notes

QR Studio Ultra is not trying to be a bland utility app.

Polish is welcome, but it should feel deliberate. The best contributions here tend to balance three things:

- strong aesthetics
- practical usability
- dependable scanning behavior

That balance matters more than adding lots of options for their own sake.

## Code of Conduct

Be respectful, constructive, and generous with context.

Good collaboration makes this project better. Clear bug reports, thoughtful reviews, and kind disagreement are all part of that.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](/c:/Users/pbess/Desktop/qr-studio-ultra-development/LICENSE).
