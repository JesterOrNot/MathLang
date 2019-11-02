#include <exception>
#include <iostream>
#include <fstream>
#include <string>
#include <cmath>
#include <bits/stdc++.h>
#include "eval.h"
using namespace std;
int main(int argc, char* argv[]) {
    if (argc == 1 || argc > 2) {
        if (argc == 1) cout << "Not enough arguments\nUsage: mathlang [FILE_NAME]\n";
        if (argc > 2) cout << "Too many arguments\nUsage: mathlang [FILE_NAME]\n";
        return 0;
    } else {
        ifstream file(argv[1]);
        string str;
        int count = 1;
        while (getline(file, str)) {
            try {
                if(str.substr(0,4) == "sqrt") {
                    float val = std::stof(str.substr(5));
                    cout << sqrt(val) << endl;
                    count++;
                    continue;
                }
                cout << eval(str) << endl;
                count++;
            } catch(exception e) {
                cout << "Syntax Error on line " << count << endl;
            }
        }
    }
}