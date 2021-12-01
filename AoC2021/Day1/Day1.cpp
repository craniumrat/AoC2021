#include <filesystem>
#include <vector>
#include <map>
#include <string>
#include <fstream>
#include <iostream>
#include <boost/spirit/home/x3.hpp>
#include <boost/fusion/include/adapt_struct.hpp>

using namespace std::filesystem;

namespace day1
{
	namespace ast
	{
		typedef std::map<std::string, int> bag_map;
		typedef std::pair<std::string, int> bag_pair;

		struct bag
		{
			std::string name;
			std::map<std::string, int> inner_bags;

			bool operator == (const bag& other) const {
				return name == other.name;
			}

			bool operator < (const bag& other) const {
				return name < other.name;
			}
		};

		using boost::fusion::operator<<;
	}

	namespace parser
	{
		namespace x3 = boost::spirit::x3;
		namespace ascii = boost::spirit::x3::ascii;

		using x3::int_;
		using ascii::char_;
		using x3::lit;
		using x3::lexeme;


		x3::rule<class adjective, std::string> adjective = "adjective";
		x3::rule<class color, std::string> color = "color";
		auto const adjective_def = +char_;
		auto const color_def = +char_;

		x3::rule<class desc, std::string> desc = "desc";
		auto const desc_def = adjective >> color;

		x3::rule<class inner_bag, std::pair<int, std::string>> inner_bag = "inner_bag";
		auto const inner_bag_def = int_ >> desc >> "bag" >> *"s";

		x3::rule<class outer_bag, std::string> outer_bag = "outer_bag";
		auto const outer_bag_def = desc >> "bags";

		//x3::rule<class inner_bags, std::vector<std::pair<int, std::string>>> inner_bags = "inner_bags";
		x3::rule<class inner_bags, std::vector<std::pair<int, std::string>>> inner_bags = "inner_bags";
		auto const inner_bags_def = inner_bag % ',';

		auto const no_other_bags = lit("no other bags");		
		x3::rule<class bag, day1::ast::bag> const bag = "bag";
		auto const bag_def = outer_bag >> "contain" >> (inner_bags | no_other_bags);

		BOOST_SPIRIT_DEFINE(adjective, color, desc, inner_bag, outer_bag, inner_bags, bag)

		//template<typename Iterator>
		//bool parse_inner_bag(Iterator first, Iterator last, std::unordered_map<std::string, int>& inners)
		//{
		//	int count;
		//	std::string name;

		//	auto const fcount = [&](auto& ctx){ count = _attr(ctx); };
		//	auto const fname = [&](auto& ctx) {name = _attr(ctx); }

		//	bool result = phrase_parse(first, last, int_[fint] >> desc_[fname], space);
		//	if (result && first == last)
		//	{
		//		inners.insert(name, count);
		//		return true;
		//	}

		//	return false;
		//}
	}
}

BOOST_FUSION_ADAPT_STRUCT(day1::ast::bag,
	name,
	(day1::ast::bag_map, bags)
)

int main()
{
	std::vector<day1::ast::bag> bags;

	path inputFile(L"input.txt");

	using boost::spirit::x3::ascii::space;
	typedef std::string::const_iterator iterator_type;

	std::string line;
	std::ifstream fin(inputFile);
	while (getline(fin, line))
	{
		iterator_type iter = line.begin();
		iterator_type const end = line.end();

		day1::ast::bag bag;
		std::cout << line << std::endl;

		bool result = phrase_parse(iter, end, day1::parser::bag, space, bag);
		if (result && iter == end)
		{
			std::cout << "-------------------------\n";
			std::cout << "Parsing succeeded\n";
			std::cout << "got: " << bag.name;
			for (auto&& b: bag.inner_bags)
			{
				std::cout << " " << b.first << ": " << b.second << std::endl;
			}
			std::cout << "\n-------------------------\n";
			bags.push_back(bag);
		}
		else
		{
			std::cout << "-------------------------\n";
			std::cout << "Parsing failed\n";
			std::cout << "-------------------------\n";
		}
	}

	return 0;
}