#include <mixr/mixr.hpp>
#include <mixr/Stream/Wav.hpp>
#include <mixr/mixr.h>
#include <mixr/Stream/Wav.h>
#include <mixr/Utils/ADPCM.h>
#include <thread>
#include <iostream>

using namespace mixr;
using namespace mixr::Utils;

int main() {
    Stream::Wav wav(R"(C:\Users\ollie\Documents\Audacity\17 Spectrum (Say My Name).wav)");
    auto format = wav.Format();
    auto data = wav.GetPCM();

    auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();

    SourceDescription description {
        .Type = SourceType::PCM,
        .Format = format
    };

    if (wav.IsADPCM()) {
        description.Type = SourceType::ADPCM,
        description.ADPCM = { .ChunkSize = wav.ADPCMInfo().ChunkSize };
    }

    auto buffer = context->CreateBuffer(data.data(), data.size());

    auto source = context->CreateSource(description);
    source->SubmitBuffer(buffer.get());

    source->Play();

    /*MxAudioStream* stream;
    mxStreamLoadWav(R"(C:\Users\ollie\Documents\Audacity\test.wav)", &stream);

    MxAudioFormat format = mxStreamGetFormat(stream);

    uint8_t* data;
    size_t dataLength;
    mxStreamGetPCM(stream, nullptr, &dataLength);
    data = new uint8_t[dataLength];
    mxStreamGetPCM(stream, data, &dataLength);

    MxDevice* device;
    MxContext* context;
    mxCreateSDLDevice(48000, 512, &device);
    mxDeviceGetContext(device, &context);

    //mxContextSetMasterVolume(context, 0.1f);

    MxSourceDescription description {
        .Type = MX_SOURCE_TYPE_PCM,
        .Format = format,
    };

    if (mxWavIsADPCM(stream)) {
        MxADPCMInfo adpcm = mxWavGetADPCMInfo(stream);

        description.Type = MX_SOURCE_TYPE_ADPCM;
        description.ADPCM = { .ChunkSize = adpcm.ChunkSize };
    }

    mxDestroyStream(stream);

    MxAudioBuffer buffer = mxContextCreateBuffer(context, data, dataLength);

    MxAudioSource source = mxContextCreateSource(context, &description);
    mxSourceSubmitBuffer(context, source, buffer);

    //mxSourceSetSpeed(context, source, 2.0);
    //mxSourceSetVolume(context, source, 0.5f);
    mxSourceSetLooping(context, source, true);
    //mxSourceSetPanning(context, source, -1.0f);

    mxSourcePlay(context, source);

    bool test = false;*/

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