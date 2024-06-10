#include <mixr/mixr.hpp>
#include <mixr/mixr.h>
#include <fstream>
#include <vector>
#include <thread>
#include <iostream>

using namespace mixr;

int main() {
    std::ifstream file(R"(C:\Users\ollie\Music\TESTFILES\Insomnia-16bitshort.raw)", std::ios::binary);
    std::vector<uint8_t> pcmData(std::istreambuf_iterator<char>{file}, {});

    auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();
    //context->SetMasterVolume(1 / 512.0f);

    AudioFormat format {
        .DataType = DataType::I16,
        .SampleRate = 44100,
        .Channels = Channels::Stereo
    };

    auto buffer = context->CreateBuffer(format, pcmData.data(), pcmData.size());

    auto source = context->CreateSource();
    source->SubmitBuffer(buffer.get());

    source->SetSpeed(1.30);
    //source->SetVolume(0.5f);
    source->SetLooping(true);

    source->Play();

    /*MxDevice* device;
    MxContext* context;
    mxCreateSDLDevice(48000, 512, &device);
    mxDeviceGetContext(device, &context);

    mxContextSetMasterVolume(context, 0.1f);

    MxAudioFormat format {
        .DataType = MX_DATA_TYPE_I16,
        .SampleRate = 44100,
        .Channels = MX_CHANNELS_STEREO
    };

    MxAudioBuffer buffer = mxContextCreateBuffer(context, &format, pcmData.data(), pcmData.size());

    MxAudioSource source = mxContextCreateSource(context);
    mxSourceSubmitBuffer(context, source, buffer);

    mxSourceSetSpeed(context, source, 2.0);
    mxSourceSetVolume(context, source, 0.5f);
    mxSourceSetLooping(context, source, true);

    mxSourcePlay(context, source);*/

    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    //mxDestroyDevice(device);

    return 0;
}