#include <mixr/mixr.hpp>
#include <mixr/Stream/Wav.hpp>
//#include <mixr/mixr.h>
#include <mixr/Stream/Wav.h>
#include <mixr/Utils/ADPCM.h>
#include <thread>
#include <iostream>

using namespace mixr;
using namespace mixr::Utils;

int main() {
    //Stream::Wav wav(R"(C:\Users\ollie\Documents\Audacity\test.wav)");
    //auto format = wav.Format();
    //auto data = wav.GetPCM();

    //if (wav.IsADPCM()) {
    //    data = ADPCM::DecodeIMA(data.data(), data.size(), format.Channels == Channels::Stereo, wav.ADPCMInfo().ChunkSize);
    //}

    //constexpr int numSounds = 1;

    /*auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();
    //context->SetMasterVolume(1.0f / numSounds);

    BufferDescription description {
        .Type = PcmType::PCM,
        .Format = format
    };

    if (wav.IsADPCM()) {
        description.Type = PcmType::ADPCM,
        description.ADPCM = { .ChunkSize = wav.ADPCMInfo().ChunkSize };
    }

    auto buffer = context->CreateBuffer(description, data.data(), data.size());

    auto source = context->CreateSource();
    source->SubmitBuffer(buffer.get());

    //source->ClearBuffers();

    //source->SetSpeed(0.25);
    //source->SetVolume(0.5f);
    //source->SetLooping(true);
    //source->SetPanning(0.0f);
    //source->SetChannelVolumes(-1, 1);

    source->Play();*/

    MxAudioStream* stream;
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

    MxBufferDescription description {
        .Type = MX_PCM_TYPE_PCM,
        .Format = format,
    };

    if (mxWavIsADPCM(stream)) {
        MxADPCMInfo adpcm = mxWavGetADPCMInfo(stream);

        description.Type = MX_PCM_TYPE_ADPCM;
        description.ADPCM = { .ChunkSize = adpcm.ChunkSize };
    }

    mxDestroyStream(stream);

    MxAudioBuffer buffer = mxContextCreateBuffer(context, &description, data, dataLength);

    MxAudioSource source = mxContextCreateSource(context);
    mxSourceSubmitBuffer(context, source, buffer);

    //mxSourceSetSpeed(context, source, 2.0);
    //mxSourceSetVolume(context, source, 0.5f);
    mxSourceSetLooping(context, source, true);
    //mxSourceSetPanning(context, source, -1.0f);

    mxSourcePlay(context, source);

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

    mxDestroyDevice(device);

    return 0;
}