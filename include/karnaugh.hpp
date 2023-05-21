#pragma once
#include <iostream>
#include <string>

class Karnaugh {
private:
  int rows, cols;
  bool m_isFourVariable;

public:
  Karnaugh();
  ~Karnaugh();

  std::string createFourVariableMap(); 
  std::string createFiveVariableMap();
  bool isFourVariable(std::string message);
  
};