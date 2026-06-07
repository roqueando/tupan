"""Entry point for Tupan — Power Electronics Workbench.

Usage:
    python -m tupan
"""

import sys
from PySide6.QtWidgets import QApplication
from tupan.app.app import TupanApp
from tupan.ui.theme import apply_theme


def main():
    app = QApplication(sys.argv)
    app.setApplicationName("Tupan")
    app.setOrganizationName("Tupan")

    # Apply light theme
    apply_theme(app)

    window = TupanApp()
    window.show()

    sys.exit(app.exec())


if __name__ == "__main__":
    main()
