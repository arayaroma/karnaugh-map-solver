#include "../include/karnaugh.hpp"

Karnaugh::Karnaugh() {
  rows = 0;
  cols = 0;
  m_isFourVariable = false;
}

Karnaugh::~Karnaugh() {}

bool Karnaugh::isFourVariable(std::string message) {
  if (message == "4") {
    m_isFourVariable = true;
    return true;
  }
  return false;
}

std::string Karnaugh::createFourVariableMap() {
  std::string karnaughMap = "";
  if (m_isFourVariable) {
    karnaughMap = "AB\nCD | 00 | 01 | 11 | 10 |\n"
                  "-----------------------|\n"
                  "00 |    |    |    |    |\n"
                  "-----------------------|\n"
                  "01 |    |    |    |    |\n"
                  "-----------------------|\n"
                  "11 |    |    |    |    |\n"
                  "-----------------------|\n"
                  "10 |    |    |    |    |\n";
  }
  return karnaughMap;
}

std::string Karnaugh::createFiveVariableMap() {
  std::string karnaughMap = "";
  if (!m_isFourVariable) {
    karnaughMap = "ABC\nDE | 000 | 001 | 011 | 010 | 110 | 111 | 101 | 100 |\n"
                  "---------------------------------------------------|\n"
                  "00 |     |     |     |     |     |     |     |     |\n"
                  "---------------------------------------------------|\n"
                  "01 |     |     |     |     |     |     |     |     |\n"
                  "---------------------------------------------------|\n"
                  "11 |     |     |     |     |     |     |     |     |\n"
                  "---------------------------------------------------|\n"
                  "10 |     |     |     |     |     |     |     |     |\n";
  }
  return karnaughMap;
}