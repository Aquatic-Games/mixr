#include <mixr/mixr.hpp>
#include <fstream>
#include <vector>
#include <thread>
#include <iostream>

using namespace mixr;

int main() {
    std::ifstream file(R"(C:\Users\ollie\Music\TESTFILES\Always There-32bitfloat.raw)", std::ios::binary);
    std::vector<uint8_t> data(std::istreambuf_iterator<char>{file}, {});

    auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();

    AudioFormat format {
        .DataType = DataType::F32,
        .SampleRate = 44100,
        .Channels = Channels::Stereo
    };

    auto buffer = context->CreateBuffer(format, data.data(), data.size());

    auto source = context->CreateSource();
    source->SubmitBuffer(buffer.get());
    source->Play();

    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    return 0;
}