cmake_minimum_required(VERSION 3.10)
project(ClassRoomQuestionnaire VERSION 1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(${PROJECT_SOURCE_DIR}/include)

# Add library source files (excluding main.cpp)
set(LIB_SOURCES
    src/Question.cpp
    src/Questionnaire.cpp
    src/StudentSession.cpp
    src/SessionManager.cpp
)

# Create library
add_library(questionnaire_lib ${LIB_SOURCES})

# Main executable
add_executable(questionnaire_backend src/main.cpp)
target_link_libraries(questionnaire_backend questionnaire_lib pthread)

# Enable testing
enable_testing()
add_subdirectory(tests)
