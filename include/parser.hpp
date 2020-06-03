#include "Clifi.hpp"

bool help = (find(args.begin(), args.end(), "--help") != args.end());
bool help_short = (find(args.begin(), args.end(), "-h") != args.end());
bool version = (find(args.begin(), args.end(), "--version") != args.end());
bool devel = (find(args.begin(), args.end(), "--devel") != args.end());