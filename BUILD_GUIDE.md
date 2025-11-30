# Build Guide for ScreenCapture

This project is a Windows application built with **C++** and **Qt 6**.
To create a standalone executable file (ScreenCapture.exe), you need to build it in **Release** mode using Visual Studio.

## Prerequisites

1.  **Visual Studio 2022**
    *   Workload: **Desktop development with C++**
2.  **Qt 6.8.3**
    *   You need the **MSVC 2022 64-bit** kit.
    *   **Important**: For a single portable executable, you need a **Static Build** of Qt (e.g., `6.8.3-static_msvc2022_64`).
    *   If you only have the standard (shared) Qt installed, the executable will require Qt DLLs to run.
3.  **Qt Visual Studio Tools** (Extension for VS)

## Steps to Build

1.  **Open the Project**
    *   Launch Visual Studio 2022.
    *   Open `ScreenCapture.sln` in the project root.

2.  **Configure Qt**
    *   Go to **Extensions** > **Qt VS Tools** > **Qt Versions**.
    *   Ensure you have a Qt version registered.
    *   The project expects a version named `6.8.3-static_msvc2022_64` for Release builds.
    *   *Tip: If you don't have a static build, you can change the project properties to use your installed Qt version, but you will need to deploy DLLs later.*

3.  **Select Configuration**
    *   In the top toolbar, set the Solution Configuration to **Release**.
    *   Set the Solution Platform to **x64**.

4.  **Build**
    *   Menu: **Build** > **Build Solution** (or press `Ctrl+Shift+B`).
    *   Wait for the build to complete.

5.  **Locate the Executable**
    *   Once successful, the executable will be generated in:
        `build/Release/ScreenCapture.exe`

## Troubleshooting

*   **Qt Version Not Found**: If VS complains about missing Qt version, right-click the project in Solution Explorer > **Properties** > **Qt Project Settings** > **Qt Installation**, and select your installed Qt version.
*   **Missing DLLs**: If you built with a standard (shared) Qt version, the `.exe` won't run alone. You need to run `windeployqt` or copy the required DLLs (Qt6Core.dll, Qt6Gui.dll, etc.) to the same folder.
