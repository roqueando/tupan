cmake_minimum_required(VERSION 3.20)
project(tupan VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ── Dependencies ──────────────────────────────────────────────────────

# wxWidgets — system package
find_package(wxWidgets REQUIRED COMPONENTS core base adv)
include(${wxWidgets_USE_FILE})

# nlohmann_json — header-only, via FetchContent
include(FetchContent)
FetchContent_Declare(nlohmann_json
    GIT_REPOSITORY https://github.com/nlohmann/json.git
    GIT_TAG v3.11.3)
FetchContent_MakeAvailable(nlohmann_json)

# wxMathPlot — optional, fetched from GitHub.
# If unavailable, the application falls back to built-in SimplePlotWidget.
option(TUPAN_USE_WXMATPLOT "Use wxMathPlot for plotting (fallback to built-in otherwise)" OFF)

if(TUPAN_USE_WXMATPLOT)
    FetchContent_Declare(wxmathplot
        GIT_REPOSITORY https://github.com/liquidassembler/wxMathPlot.git
        GIT_TAG master)
    FetchContent_MakeAvailable(wxmathplot)
    add_definitions(-DTUPAN_HAVE_WXMATPLOT=1)
endif()

# ── Sources ────────────────────────────────────────────────────────────

set(SOURCES
    src/main.cpp

    # App layer
    src/app/AppState.cpp
    src/app/Persistence.cpp

    # Domain — converters
    src/domain/converters/Common.cpp
    src/domain/converters/Buck.cpp
    src/domain/converters/Boost.cpp

    # Domain — inverter
    src/domain/inverter/VsiSingle.cpp
    src/domain/inverter/Pwm.cpp

    # Domain — components
    src/domain/components/Inductor.cpp
    src/domain/components/Capacitor.cpp
    src/domain/components/Load.cpp

    # Domain — metrics
    src/domain/metrics/Efficiency.cpp
    src/domain/metrics/Ripple.cpp
    src/domain/metrics/Thd.cpp

    # Simulation
    src/simulation/Integrator.cpp
    src/simulation/CircuitOdes.cpp

    # Schematic
    src/schematic/Layout.cpp
    src/schematic/Renderer.cpp
    src/schematic/ExportSvg.cpp

    # UI
    src/ui/MainFrame.cpp
    src/ui/CanvasPanel.cpp

    # Utilities
    src/utils/Formatting.cpp
    src/utils/Theme.cpp
)

set(HEADERS
    src/app/AppState.h
    src/app/Persistence.h

    src/domain/Types.h
    src/domain/converters/Common.h
    src/domain/converters/Buck.h
    src/domain/converters/Boost.h

    src/domain/inverter/VsiSingle.h
    src/domain/inverter/Pwm.h

    src/domain/components/Inductor.h
    src/domain/components/Capacitor.h
    src/domain/components/Load.h

    src/domain/metrics/Efficiency.h
    src/domain/metrics/Ripple.h
    src/domain/metrics/Thd.h

    src/simulation/Integrator.h
    src/simulation/CircuitOdes.h

    src/schematic/Primitives.h
    src/schematic/Layout.h
    src/schematic/Renderer.h
    src/schematic/ExportSvg.h

    src/ui/MainFrame.h
    src/ui/CanvasPanel.h

    src/utils/Formatting.h
    src/utils/Theme.h
)

add_executable(tupan ${SOURCES} ${HEADERS})

target_include_directories(tupan PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${wxWidgets_INCLUDE_DIRS}
)

if(TUPAN_USE_WXMATPLOT)
    target_include_directories(tupan PRIVATE
        ${wxmathplot_SOURCE_DIR}/include
        ${wxmathplot_SOURCE_DIR}/src
    )
endif()

target_link_libraries(tupan PRIVATE
    ${wxWidgets_LIBRARIES}
    nlohmann_json::nlohmann_json
)
