cmake_minimum_required(VERSION 3.8)
project(waypoint_navigation)

find_package(ament_cmake_auto REQUIRED)

ament_auto_find_build_dependencies()

if(NOT DEFINED CMAKE_SUPPRESS_DEVELOPER_WARNINGS)
     set(CMAKE_SUPPRESS_DEVELOPER_WARNINGS 1 CACHE INTERNAL "No dev warnings")
endif()

r2r_cargo(
  rcl
)

ament_auto_package(
  INSTALL_TO_SHARE
)
