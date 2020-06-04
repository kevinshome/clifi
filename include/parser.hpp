#include "Clifi.hpp"
#include <argparse.hpp>
#include <subprocess.hpp>

bool verbose = false;
string stream_name;

void parser_init(int argc, char **argv){

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
        .action([](const std::string& value) { return value; })
        .default_value(false);
    
    parser.add_argument("-k", "--kill")
        .help("kill all running VLC instances")
        .default_value(false)
        .implicit_value(true);

    try {
      parser.parse_args(argc, argv);
    }
    catch (const std::runtime_error& err) {
      cout << err.what() << endl;
      cout << parser;
      exit(0);
    }

    if(parser["--verbose"] == true){
        verbose = true;
    }

    if(parser["--version"] == true){
        string _version = __VERSION;
        string _release_stage = __RELEASE_STAGE;
        cout << "clifi " << _release_stage << " " << _version << endl;
        exit(0);
    }

    if(parser["--kill"] == true){
        subprocess::popen cmd("killall", {"vlc"});
        exit(0);
    }

    try{
        stream_name = parser.get("stream");
    }
    catch(const std::exception& err){
        stream_name = "lofi";
    }
}