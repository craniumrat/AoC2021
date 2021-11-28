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
		struct rule
		{
			std::string name;
			int min_1;
			int max_1;
			int min_2;
			int max_2;
		};

		//typedef std::map<std::string, int> bag_map;
		//typedef std::pair<std::string, int> bag_pair;

		//struct bag
		//{
		//	std::string name;
		//	bag_map bags;
		//};

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

		x3::rule<class rule, day1::ast::rule> const rule = "rule";

		auto const rule_def =
			lexeme [+(char_ - ':') >> ':']
			>> int_ >> '-' >> int_
			>> "or"
			>> int_ >> '-' >> int_;

		BOOST_SPIRIT_DEFINE(rule)

		//x3::rule<class bag, day1::ast::bag> const bag = "bag";

		//auto const adjective = +char_;
		//auto const color = +char_;

		//auto const desc = adjective >> color;
		//auto const inner_bag = int_ >> desc >> "bag" >> *"s";
		//auto const outer_bag = desc >> "bags";
		//auto const inner_bags = inner_bag % ',';
		//auto const no_other_bags = lit("no other bags");
		//
		//auto const bag_def = outer_bag >> "contain" >> (inner_bags | no_other_bags);

		//BOOST_SPIRIT_DEFINE(bag)

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

BOOST_FUSION_ADAPT_STRUCT(day1::ast::rule, name, min_1, max_1, min_2, max_2)
//BOOST_FUSION_ADAPT_STRUCT(day1::ast::bag,
//	name,
//	(day1::ast::bag_map, bags)
//)

int main()
{
	std::vector <day1::ast::rule> rules;
	//std::vector<day1::ast::bag> bags;

	path inputFile(L"input.txt");

	using boost::spirit::x3::ascii::space;
	typedef std::string::const_iterator iterator_type;

	std::string line;
	std::ifstream fin(inputFile);
	while (getline(fin, line))
	{
		iterator_type iter = line.begin();
		iterator_type const end = line.end();

		day1::ast::rule rule;
		//day1::ast::bag bag;
		std::cout << line << std::endl;

		bool result = phrase_parse(iter, end, day1::parser::rule, space, rule);
		//bool result = phrase_parse(iter, end, day1::parser::bag, space, bag);
		if (result && iter == end)
		{
			std::cout << "-------------------------\n";
			std::cout << "Parsing succeeded\n";
			std::cout << "got: " << rule.name << ": " << rule.min_1 << "-" << rule.max_1 << " " << rule.min_2 << "-" << rule.max_2 << std::endl;
			//std::cout << "got: " << bag.name;
			//for (auto&& b: bag.bags)
			//{
			//	std::cout << " " << b.first << ": " << b.second << std::endl;
			//}
			//std::cout << "\n-------------------------\n";
			rules.push_back(rule);
			//bags.push_back(bag);
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