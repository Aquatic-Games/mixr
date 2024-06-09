//#include <mixr/mixr.hpp>
#include <mixr/mixr.h>
#include <fstream>
#include <vector>
#include <thread>
#include <iostream>

//using namespace mixr;

int main() {
    std::ifstream file(R"(C:\Users\ollie\Music\TESTFILES\Always There-32bitfloat.raw)", std::ios::binary);
    std::vector<uint8_t> data(std::istreambuf_iterator<char>{file}, {});

    /*auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();
    //context->SetMasterVolume(0.1f);

    AudioFormat format {
        .DataType = DataType::F32,
        .SampleRate = 44100,
        .Channels = Channels::Stereo
    };

    auto buffer = context->CreateBuffer(format, data.data(), data.size());

    auto source = context->CreateSource();
    source->SubmitBuffer(buffer.get());
    //source->SetSpeed(2);
    //source->SetVolume(0.5f);
    source->SetLooping(true);
    source->Play();

    //auto source2 = context->CreateSource();
    //source2->SubmitBuffer(buffer.get());
    //source2->Play();*/

    MxContext* context;
    mxCreateContext(48000, &context);

    MxAudioFormat format {
        .DataType = MX_DATA_TYPE_F32,
        .SampleRate = 44100,
        .Channels = MX_CHANNELS_STEREO
    };

    MxAudioBuffer buffer = mxContextCreateBuffer(context, &format, data.data(), data.size());

    MxAudioSource source = mxContextCreateSource(context);
    mxSourceSubmitBuffer(context, source, buffer);
    mxSourcePlay(context, source);

    /*while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }*/

    mxDestroyContext(context);

    return 0;
}