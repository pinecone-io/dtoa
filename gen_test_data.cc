#include <iostream>
#include <stdio.h>
#include <nlohmann/json.hpp>
#include <fstream>
#include <string>
#include <random>

void write_one_f64(std::ofstream &file, double raw) {
  file.write(reinterpret_cast<const char*>(&raw), sizeof(raw));

  nlohmann::json json_ob = raw;
  auto stringValue = json_ob.dump();
  int stringSize = stringValue.size();
  file.write(reinterpret_cast<const char*>(&stringSize), sizeof(int));
  file.write(stringValue.c_str(), stringSize);
}

void write_one_f32(std::ofstream &file, float raw) {
  file.write(reinterpret_cast<const char*>(&raw), sizeof(raw));

  nlohmann::json json_ob = raw;
  auto stringValue = json_ob.dump();
  int stringSize = stringValue.size();
  file.write(reinterpret_cast<const char*>(&stringSize), sizeof(int));
  file.write(stringValue.c_str(), stringSize);
}

int main() {
  double multiple = -1.00007;
  std::ofstream file_f64("nholmann_json_f64.txt", std::ios::binary);
  if (file_f64.is_open()) {
    // boundaries
    write_one_f64(file_f64, 0.0);
    write_one_f64(file_f64, -0.0);
    write_one_f64(file_f64, 1.0);
    write_one_f64(file_f64, -1.0);
    write_one_f64(file_f64, std::numeric_limits<double>::min());
    write_one_f64(file_f64, std::numeric_limits<double>::max());
    write_one_f64(file_f64, std::numeric_limits<double>::quiet_NaN());
    write_one_f64(file_f64, std::numeric_limits<double>::epsilon());
    write_one_f64(file_f64, std::numeric_limits<double>::infinity());
    write_one_f64(file_f64, -std::numeric_limits<double>::infinity());

    // known bads
    write_one_f64(file_f64, 4599.99999);
    write_one_f64(file_f64, 9.904578032905936e+15);

    //    return 0;
    double current = 1.0;
    while (current > 0.0) {
      //double raw = (double)dis(gen);
      current = current/multiple;
      write_one_f64(file_f64, current);
    }
    current = 0.00001;
    while (current != std::numeric_limits<double>::quiet_NaN()
	   && current != std::numeric_limits<double>::infinity()) {
      current = current*multiple;
      write_one_f64(file_f64, current);
    }
  }

  std::ofstream file_f32("nholmann_json_f32.txt", std::ios::binary);
  if (file_f32.is_open()) {
    // boundaries
    write_one_f32(file_f32, 0.0);
    write_one_f32(file_f32, -0.0);
    write_one_f32(file_f32, 1.0);
    write_one_f32(file_f32, -1.0);
    write_one_f32(file_f32, std::numeric_limits<double>::min());
    write_one_f32(file_f32, std::numeric_limits<double>::max());
    write_one_f32(file_f32, std::numeric_limits<double>::quiet_NaN());
    write_one_f32(file_f32, std::numeric_limits<double>::epsilon());
    write_one_f32(file_f32, std::numeric_limits<double>::infinity());
    write_one_f32(file_f32, -std::numeric_limits<double>::infinity());

    // known bads
    write_one_f32(file_f32, 4599.99999);
    write_one_f32(file_f32, 9.904578032905936e+15);
    write_one_f32(file_f32, 2.220446049250313e-16);

    double current = 1.0;
    while (current > 0.0) {
      //double raw = (double)dis(gen);
      current = current/multiple;
      write_one_f32(file_f32, current);
    }
    current = 0.000001;
    while (current != std::numeric_limits<double>::quiet_NaN()
	   && current != std::numeric_limits<double>::infinity()) {
      current = current*multiple;
      write_one_f32(file_f32, current);
    }
  }
}
