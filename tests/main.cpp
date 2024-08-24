#include <mixr/mixr.hpp>
#include <mixr/Stream/Wav.hpp>
#include <mixr/Stream/Flac.hpp>

#include <mixr/mixr.h>
#include <mixr/Stream/Wav.h>
#include <mixr/Stream/Flac.h>

#include <thread>
#include <iostream>

using namespace mixr;

struct CallbackData {
    std::unique_ptr<Stream::AudioStream> Stream;
    AudioSource* Source;
    std::vector<uint8_t> Buffer;
    std::vector<std::unique_ptr<AudioBuffer>> Buffers;
    int CurrentBuffer;
};

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Please enter filename as argument" << std::endl;
        return 1;
    }

    auto stream = std::make_unique<Stream::Flac>(argv[1]);
    auto format = stream->Format();
    //auto data = wav.GetPCM();

    auto device = std::make_unique<Device::SdlDevice>(48000);
    Context* context = device->Context();

    SourceDescription description {
       /* .Type = */ SourceType::PCM,
       /* .Format = */ format
    };

    //if (wav.IsADPCM()) {
    //    description.Type = SourceType::ADPCM,
    //    description.ADPCM = { .ChunkSize = wav.ADPCMInfo().ChunkSize };
    //}

    std::vector<uint8_t> buffer;
    buffer.resize(format.SampleRate);

    stream->GetBuffer(buffer.data(), buffer.size());
    auto buffer1 = context->CreateBuffer(buffer.data(), buffer.size());

    stream->GetBuffer(buffer.data(), buffer.size());
    auto buffer2 = context->CreateBuffer(buffer.data(), buffer.size());

    auto source = context->CreateSource(description);
    source->SubmitBuffer(buffer1.get());
    source->SubmitBuffer(buffer2.get());

    std::vector<std::unique_ptr<AudioBuffer>> buffers;
    buffers.push_back(std::move(buffer1));
    buffers.push_back(std::move(buffer2));

    auto cbData = std::make_unique<CallbackData>(CallbackData {
        std::move(stream),
        source.get(),
        std::move(buffer),
        std::move(buffers),
        0
    });

    source->SetBufferFinishedCallback([](void* userData) -> void {
        auto cbData = (CallbackData*) userData;

        auto size = cbData->Stream->GetBuffer(cbData->Buffer.data(), cbData->Buffer.size());
        std::cout << "Request Buffer: " << size << " bytes returned" << std::endl;

        if (size < cbData->Buffer.size()) {
            std::cout << "Restart" << std::endl;
            cbData->Stream->Restart();
        }

        cbData->Buffers[cbData->CurrentBuffer]->Update(cbData->Buffer.data(), size);
        cbData->Source->SubmitBuffer(cbData->Buffers[cbData->CurrentBuffer].get());

        cbData->CurrentBuffer++;
        if (cbData->CurrentBuffer >= cbData->Buffers.size())
            cbData->CurrentBuffer = 0;

    }, cbData.get());

    //source->SetSpeed(50);
    source->Play();

    /*MxAudioStream* stream;
    mxStreamLoadFlac(R"(C:\Users\ollie\Music\Music\Various Artists\NOW Millennium- 2000 - 2001 (Disc 3)\19 Inner Smile.flac)", &stream);

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

    /*if (mxWavIsADPCM(stream)) {
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

        //std::cout << buffer->ID() << std::endl;
        //buffer.reset();

        //buffer = context->CreateBuffer(data.data(), data.size());
        //source->SubmitBuffer(buffer.get());
        //source->Play();

        /*std::cout << source->ID() << std::endl;
        source.reset();

        source = context->CreateSource(description);
        source->SubmitBuffer(buffer.get());
        source->Play();*/

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