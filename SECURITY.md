# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Privacy and Offline Guarantee

Jotlet is designed from the ground up as a **100% offline, privacy-first personal notes application**.

- **No Remote Calls**: Jotlet never makes network requests, sends telemetry, or tracks usage.
- **No Third-Party Services**: No cloud syncing, analytics, crash reporting, or external dependencies are executed.
- **Local Storage Only**: All user notes are saved locally in SQLite at `~/.local/share/jotlet/notes.db`.
- **Offline Integrity**: You can run Jotlet in a completely air-gapped environment without any loss of functionality.

## Reporting a Vulnerability

If you discover a security vulnerability within Jotlet, please send an email to the project maintainers or open a private security advisory on GitHub.

Please do not disclose security issues in public issue trackers until they have been addressed.
