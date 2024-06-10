#include <mixr/mixr.hpp>
#include <mixr/Stream/Wav.hpp>
#include <mixr/mixr.h>
#include <mixr/Stream/Wav.h>
#include <thread>
#include <iostream>

using namespace mixr;

int main() {
    /*Stream::Wav wav(R"(C:\Users\ollie\Music\DEADLOCK.wav)");
    auto format = wav.Format();
    auto data = wav.GetPCM();

    auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();
    //context->SetMasterVolume(1 / 512.0f);

    auto buffer = context->CreateBuffer(format, data.data(), data.size());

    auto source = context->CreateSource();
    source->SubmitBuffer(buffer.get());

    //source->SetSpeed(1.30);
    //source->SetVolume(0.5f);
    source->SetLooping(true);
    source->SetPanning(0.0f);

    source->Play();*/

    MxAudioStream* stream;
    mxStreamLoadWav(R"(C:\Users\ollie\Music\DEADLOCK.wav)", &stream);

    MxAudioFormat format = mxStreamGetFormat(stream);

    uint8_t* data;
    size_t dataLength;
    mxStreamGetPCM(stream, nullptr, &dataLength);
    data = new uint8_t[dataLength];
    mxStreamGetPCM(stream, data, &dataLength);

    mxDestroyStream(stream);

    MxDevice* device;
    MxContext* context;
    mxCreateSDLDevice(48000, 512, &device);
    mxDeviceGetContext(device, &context);

    //mxContextSetMasterVolume(context, 0.1f);

    MxAudioBuffer buffer = mxContextCreateBuffer(context, &format, data, dataLength);

    MxAudioSource source = mxContextCreateSource(context);
    mxSourceSubmitBuffer(context, source, buffer);

    //mxSourceSetSpeed(context, source, 2.0);
    //mxSourceSetVolume(context, source, 0.5f);
    mxSourceSetLooping(context, source, true);
    //mxSourceSetPanning(context, source, -1.0f);

    mxSourcePlay(context, source);

    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    //mxDestroyDevice(device);

    return 0;
}