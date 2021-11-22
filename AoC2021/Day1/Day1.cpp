#include <filesystem>
#include <vector>
#include <string>
#include <fstream>
#include <iostream>

using namespace std::filesystem;

int main()
{
	std::vector <std::string> values;
	path inputFile(L"input.txt");

	std::string line;
	std::ifstream fin(inputFile);
	while (getline(fin, line))
	{
		values.push_back(line);
	}

	for (auto&& v:  values)
	{
		std::cout << v << std::endl;
	}

	return 0;
}