#include <mixr/mixr.hpp>
#include <fstream>
#include <vector>

using namespace mixr;

int main() {
    std::ifstream file(R"(C:\Users\ollie\Music\TESTFILES\house2-f32.raw)", std::ios::binary);
    std::vector<uint8_t> data(std::istreambuf_iterator<char>{file}, {});

    AudioFormat format {
        .DataType = DataType::F32,
        .SampleRate = 48000,
        .Channels = Channels::Stereo
    };

    std::unique_ptr<Context> context = std::make_unique<Context>(48000);
    auto buffer = context->CreateBuffer(format, data.data(), data.size());

    auto source = context->CreateSource();

    return 0;
}