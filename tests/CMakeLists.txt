cmake_minimum_required(VERSION 3.10)

# Simple test executable
add_executable(test_questionnaire test_questionnaire.cpp)
target_link_libraries(test_questionnaire questionnaire_lib pthread)

add_test(NAME QuestionnaireTest COMMAND test_questionnaire)

# JSON loader test executable
add_executable(test_json_loader test_json_loader.cpp)
target_link_libraries(test_json_loader questionnaire_lib pthread)

add_test(NAME JsonLoaderTest COMMAND test_json_loader WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
