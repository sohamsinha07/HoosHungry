cmake_minimum_required(VERSION 3.16)
project(hooshungry_client CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(cpr REQUIRED)
find_package(nlohmann_json REQUIRED)

add_executable(hooshungry_client main.cpp)
target_link_libraries(hooshungry_client PRIVATE cpr::cpr nlohmann_json::nlohmann_json)
