#include <Clifi.hpp>
#include <parser.hpp>
#include <subprocess.hpp>
#include <json.hpp>
#include <list>

using namespace nlohmann;

int main(int argc, char **argv){

    string homedir = getenv("HOME");
    string clifi_dir = "/.clifi";
    string stream_url;
    clifi_dir = homedir + clifi_dir;

    parser_init(argc, argv);

    ifstream input(clifi_dir + "/streams.json");
    json streams;
    input >> streams;

    for (long unsigned int i = 0; i < streams["streams"].size(); i++)
    {
        if(streams["streams"][i]["name"] == stream_name){
            stream_url = streams["streams"][i]["url"];
        }
    }

    /*
    for( std::string line; getline( input, line ); ){
        int x;
    }
    */
    subprocess::popen cmd("vlc", {"-vvv", "-I", "dummy", "-q", "--no-video", stream_url});

    return 0;
}