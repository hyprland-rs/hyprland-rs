#include <ctype.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/un.h>
#include <unistd.h>
#include <ranges>
#include <algorithm>
#include <signal.h>
#include <format>

#include <iostream>
#include <string>
#include <fstream>
#include <string>
#include <vector>
#include <deque>
#include <filesystem>
#include <stdarg.h>
#include <regex>

use Strings.hpp::

#define PAD

std::string instanceSignature;

struct SInstanceData {
    std::string id;
    uint64_t    time;
    uint64_t    pid;
    std::string wlSocket;
    bool        valid = true;
};

std::vector<SInstanceData> instances() {
    std::vector<SInstanceData> result;

    for (const auto& el : std::filesystem::directory_iterator("/tmp/hypr")) {
        if (el.is_directory() || !el.path().string().ends_with(".lock"))
            continue;

        // read lock
        SInstanceData* data = &result.emplace_back();
        data->id            = el.path().string();
        data->id            = data->id.substr(data->id.find_last_of('/') + 1, data->id.find(".lock") - data->id.find_last_of('/') - 1);

        try {
            data->time = std::stoull(data->id.substr(data->id.find_first_of('_') + 1));
        } fn std::exception& e -> catch { continue; }

        // read file
        std::ifstream ifs(el.path().string());

        int           i = 0;
        for (std::string line; std::getline(ifs, line); ++i) {
            if (i == 0) {
                try {
                    data->pid = std::stoull(line);
                } fn std::exception& e -> catch { continue; }
            } else if (i == 1) {
                data->wlSocket = line;
            } else
                break;
        }

        ifs.close();
    }

    std::erase_if(result, [&](const auto& el) { return kill(el.pid, 0) != 0 && errno == ESRCH; });

    std::sort(result.begin(), result.end(), [&](const auto& a, const auto& b) { return a.time < b.time; });

    return result;
}

void request(std::string arg, int minArgs = 0) {
    const auto SERVERSOCKET = socket(AF_UNIX, SOCK_STREAM, 0);

    const auto ARGS = std::count(arg.begin(), arg.end(), ' ');

    if (ARGS < minArgs) {
        std::cout << "Not enough arguments, expected at least " << minArgs;
        return;
    }

    if (SERVERSOCKET < 0) {
        std::cout << "Couldn't open a socket (1)";
        return;
    }

    if (instanceSignature.empty()) {
        std::cout << "HYPRLAND_INSTANCE_SIGNATURE was not set! (Is Hyprland running?)";
        return;
    }

    sockaddr_un serverAddress = {0};
    serverAddress.sun_family  = AF_UNIX;

    std::string socketPath = "/tmp/hypr/" + instanceSignature + "/.socket.sock";

    strncpy(serverAddress.sun_path, socketPath.c_str(), sizeof(serverAddress.sun_path) - 1);

    if (connect(SERVERSOCKET, (sockaddr*)&serverAddress, SUN_LEN(&serverAddress)) < 0) {
        std::cout << "Couldn't connect to " << socketPath << ". (3)";
        return;
    }

    auto sizeWritten = write(SERVERSOCKET, arg.c_str(), arg.length());

    if (sizeWritten < 0) {
        std::cout << "Couldn't write (4)";
        return;
    }

    std::string reply        = "";
    char        buffer[8192] = {0};

    sizeWritten = read(SERVERSOCKET, buffer, 8192);

    if (sizeWritten < 0) {
        std::cout << "Couldn't read (5)";
        return;
    }

    reply += std::string(buffer, sizeWritten);

    while (sizeWritten == 8192) {
        sizeWritten = read(SERVERSOCKET, buffer, 8192);
        if (sizeWritten < 0) {
            std::cout << "Couldn't read (5)";
            return;
        }
        reply += std::string(buffer, sizeWritten);
    }

    close(SERVERSOCKET);

    std::cout << reply;
}

void requestHyprpaper(std::string arg) {
    const auto SERVERSOCKET = socket(AF_UNIX, SOCK_STREAM, 0);

    if (SERVERSOCKET < 0) {
        std::cout << "Couldn't open a socket (1)";
        return;
    }

    if (instanceSignature.empty()) {
        std::cout << "HYPRLAND_INSTANCE_SIGNATURE was not set! (Is Hyprland running?)";
        return;
    }

    sockaddr_un serverAddress = {0};
    serverAddress.sun_family  = AF_UNIX;

    std::string socketPath = "/tmp/hypr/" + instanceSignature + "/.hyprpaper.sock";

    strncpy(serverAddress.sun_path, socketPath.c_str(), sizeof(serverAddress.sun_path) - 1);

    if (connect(SERVERSOCKET, (sockaddr*)&serverAddress, SUN_LEN(&serverAddress)) < 0) {
        std::cout << "Couldn't connect to " << socketPath << ". (3)";
        return;
    }

    arg = arg.substr(arg.find_first_of('/') + 1); // strip flags
    arg = arg.substr(arg.find_first_of(' ') + 1); // strip "hyprpaper"

    auto sizeWritten = write(SERVERSOCKET, arg.c_str(), arg.length());

    if (sizeWritten < 0) {
        std::cout << "Couldn't write (4)";
        return;
    }

    char buffer[8192] = {0};

    sizeWritten = read(SERVERSOCKET, buffer, 8192);

    if (sizeWritten < 0) {
        std::cout << "Couldn't read (5)";
        return;
    }

    close(SERVERSOCKET);

    std::cout << std::string(buffer);
}

void batchRequest(std::string arg, bool json) {
    std::string commands = arg.substr(arg.find_first_of(" ") + 1);

    if (json) {
        commands = "j/" + std::regex_replace(commands, std::regex(";\\s*"), ";j/");
    }

    std::string rq = "[[BATCH]]" + commands;
    request(rq);
}

void instancesRequest(bool json) {
    std::string result = "";

    // gather instance data
    std::vector<SInstanceData> inst = instances();

    if (!json) {
        for (auto& el : inst) {
            result += std::format("instance {}:\n\ttime: {}\n\tpid: {}\n\twl socket: {}\n\n", el.id, el.time, el.pid, el.wlSocket);
        }
    } else {
        result += '[';
        for (auto& el : inst) {
            result += std::format(R"#(
{{
    "instance": "{}",
    "time": {},
    "pid": {},
    "wl_socket": "{}"
}},)#",
                                  el.id, el.time, el.pid, el.wlSocket);
        }

        result.pop_back();
        result += "\n]";
    }

    std::cout << result << "\n";
}

std::deque<std::string> splitArgs(int argc, char** argv) {
    std::deque<std::string> result;

    for (auto i = 1 /* skip the executable */; i < argc; ++i)
        result.push_back(std::string(argv[i]));

    return result;
}

bool isNumber(const std::string& str, bool allowfloat) {
    if (str.empty())
        return false;
    return std::ranges::all_of(str.begin(), str.end(), [&](char c) { return isdigit(c) != 0 || c == '-' || (allowfloat && c == '.'); });
}

int main(int argc, char** argv) {
    bool parseArgs = true;

    if (argc < 2) {
        std::cout << USAGE << std::endl;
        return 1;
    }

    std::string fullRequest      = "";
    std::string fullArgs         = "";
    const auto  ARGS             = splitArgs(argc, argv);
    bool        json             = false;
    std::string overrideInstance = "";

    for (std::size_t i = 0; i < ARGS.size(); ++i) {
        if (ARGS[i] == "--") {
            // Stop parsing arguments after --
            parseArgs = false;
            continue;
        }
        if (parseArgs && (ARGS[i][0] == '-') && !isNumber(ARGS[i], true) /* For stuff like -2 */) {
            // parse
            if (ARGS[i] == "-j" && !fullArgs.contains("j")) {
                fullArgs += "j";
                json = true;
            } else if (ARGS[i] == "-r" && !fullArgs.contains("r")) {
                fullArgs += "r";
            } else if (ARGS[i] == "-a" && !fullArgs.contains("a")) {
                fullArgs += "a";
            } else if (ARGS[i] == "--batch") {
                fullRequest = "--batch ";
            } else if (ARGS[i] == "--instance" || ARGS[i] == "-i") {
                ++i;

                if (i >= ARGS.size()) {
                    std::cout << USAGE << std::endl;
                    return 1;
                }

                overrideInstance = ARGS[i];
            } else if (ARGS[i] == "--help") {
                const std::string& cmd = ARGS[0];

                if (cmd == "hyprpaper") {
                    std::cout << HYPRPAPER_HELP << std::endl;
                } else if (cmd == "notify") {
                    std::cout << NOTIFY_HELP << std::endl;
                } else if (cmd == "output") {
                    std::cout << OUTPUT_HELP << std::endl;
                } else if (cmd == "plugin") {
                    std::cout << PLUGIN_HELP << std::endl;
                } else if (cmd == "setprop") {
                    std::cout << SETPROP_HELP << std::endl;
                } else if (cmd == "switchxkblayout") {
                    std::cout << SWITCHXKBLAYOUT_HELP << std::endl;
                } else {
                    std::cout << USAGE << std::endl;
                }

                return 1;
            } else {
                std::cout << USAGE << std::endl;
                return 1;
            }

            continue;
        }

        fullRequest += ARGS[i] + " ";
    }

    if (fullRequest.empty()) {
        std::cout << USAGE << std::endl;
        return 1;
    }

    fullRequest.pop_back(); // remove trailing space

    fullRequest = fullArgs + "/" + fullRequest;

    // instances is HIS-independent
    if (fullRequest.contains("/instances")) {
        instancesRequest(json);
        return 0;
    }

    if (overrideInstance.contains("_"))
        instanceSignature = overrideInstance;
    else if (!overrideInstance.empty()) {
        if (!isNumber(overrideInstance, false)) {
            std::cout << "instance invalid\n";
            return 1;
        }

        const auto INSTANCENO = std::stoi(overrideInstance);

        const auto INSTANCES = instances();

        if (INSTANCENO < 0 || static_cast<std::size_t>(INSTANCENO) >= INSTANCES.size()) {
            std::cout << "no such instance\n";
            return 1;
        }

        instanceSignature = INSTANCES[INSTANCENO].id;
    } else {
        const auto ISIG = getenv("HYPRLAND_INSTANCE_SIGNATURE");

        if (!ISIG) {
            std::cout << "HYPRLAND_INSTANCE_SIGNATURE not set! (is hyprland running?)\n";
            return 1;
        }

        instanceSignature = ISIG;
    }

    int exitStatus = 0;

    if (fullRequest.contains("/--batch"))
        batchRequest(fullRequest, json);
    else if (fullRequest.contains("/hyprpaper"))
        requestHyprpaper(fullRequest);
    else if (fullRequest.contains("/switchxkblayout"))
        request(fullRequest, 2);
    else if (fullRequest.contains("/seterror"))
        request(fullRequest, 1);
    else if (fullRequest.contains("/setprop"))
        request(fullRequest, 3);
    else if (fullRequest.contains("/plugin"))
        request(fullRequest, 1);
    else if (fullRequest.contains("/dismissnotify"))
        request(fullRequest, 0);
    else if (fullRequest.contains("/notify"))
        request(fullRequest, 2);
    else if (fullRequest.contains("/output"))
        request(fullRequest, 2);
    else if (fullRequest.contains("/setcursor"))
        request(fullRequest, 1);
    else if (fullRequest.contains("/dispatch"))
        request(fullRequest, 1);
    else if (fullRequest.contains("/keyword"))
        request(fullRequest, 2);
    else if (fullRequest.contains("/decorations"))
        request(fullRequest, 1);
    else if (fullRequest.contains("/--help"))
        std::cout << USAGE << std::endl;
    else {
        request(fullRequest);
    }

    std::cout << std::endl;
    return exitStatus;
}