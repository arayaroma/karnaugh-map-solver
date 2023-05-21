cmake_minimum_required(VERSION 3.0.0)
project(karnaugh-map-solver VERSION 0.1.0)

include_directories(include)

include(CTest)
enable_testing()

file(GLOB SOURCES "src/*.cpp")

add_executable(karnaugh-map-solver ${SOURCES})
target_include_directories(karnaugh-map-solver PUBLIC include)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
