#include <mixr/mixr.hpp>
#include <mixr/Stream/Wav.hpp>
#include <mixr/mixr.h>
#include <mixr/Stream/Wav.h>
#include <thread>
#include <iostream>

using namespace mixr;

int main() {
    Stream::Wav wav1(R"(C:\Users\ollie\Music\dedune-start.wav)");
    auto format1 = wav1.Format();
    auto data1 = wav1.GetPCM();

    Stream::Wav wav2(R"(C:\Users\ollie\Music\dedune-loop.wav)");
    auto format2 = wav2.Format();
    auto data2 = wav2.GetPCM();

    auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();
    //context->SetMasterVolume(1 / 512.0f);

    auto buffer1 = context->CreateBuffer(format1, data1.data(), data1.size());
    auto buffer2 = context->CreateBuffer(format2, data2.data(), data2.size());

    auto source = context->CreateSource();
    source->SubmitBuffer(buffer1.get());
    source->SubmitBuffer(buffer2.get());

    //source->SetSpeed(5);
    //source->SetVolume(0.5f);
    source->SetLooping(true);
    source->SetPanning(0.0f);

    source->Play();

    /*MxAudioStream* stream;
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

    mxSourcePlay(context, source);*/

    bool test = false;

    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));

        //source->Stop();
        //source->Play();

        /*if (test) {
            source->Play();
        } else {
            source->Pause();
        }

        test = !test;*/
    }

    //mxDestroyDevice(device);

    return 0;
}