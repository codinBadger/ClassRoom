#ifndef QUESTIONNAIRE_LOADER_H
#define QUESTIONNAIRE_LOADER_H

#include "SessionManager.h"
#include <string>

class QuestionnaireLoader {
public:
    // Load questionnaire from JSON file and return questionnaire ID
    // Returns -1 on error
    static int loadFromJsonFile(SessionManager& manager, const std::string& filename);
    
    // Helper to display results for a completed session
    static void displayResults(const SessionManager& manager, int sessionId);
};

#endif
