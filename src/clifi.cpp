#include <Clifi.hpp>
#include <argparse.hpp>
#include <subprocess.hpp>

using namespace std;

bool verbose = false;

int main(int argc, char **argv){
    argparse::ArgumentParser parser(__NAME);

    parser.add_argument("--version")
        .help("display version and exit")
        .default_value(false)
        .implicit_value(true);

    parser.add_argument("-v", "--verbose")
        .help("be verbose")
        .default_value(false)
        .implicit_value(true);
    
    parser.add_argument("stream")
        .help("name of stream to be launched")
        .action([](const std::string& value) { return value; });

    try {
      parser.parse_args(argc, argv);
    }
    catch (const std::runtime_error& err) {
      std::cout << err.what() << std::endl;
      std::cout << parser;
      exit(0);
    }

    if(parser["--verbose"] == true){
        verbose = true;
    }

    if(parser["--version"] == true){
        string _version = __VERSION;
        cout << "clifi C++ rewrite pre-alpha " + _version + "\n";
        exit(0);
    }

    subprocess::popen cmd("vlc", {"-I", "dummy", "-q", "--no-video", "https://www.youtube.com/watch?v=5qap5aO4i9A"});

    return 0;
}