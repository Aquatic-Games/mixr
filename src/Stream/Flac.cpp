#include "mixr/Stream/Flac.hpp"

#include <iostream>
#include <stdexcept>

namespace mixr::Stream {
    class FlacDecoder : public FLAC::Decoder::File {
    public:
        std::vector<uint8_t> Buffer;

        std::vector<uint8_t>* BufferToWriteTo;

        AudioFormat Format;
        size_t MaxBufferSize;
        size_t TotalSamples;

        ::FLAC__StreamDecoderWriteStatus write_callback(const ::FLAC__Frame* frame, const FLAC__int32* const buffer[]) override {
            for (int i = 0; i < frame->header.blocksize; i++) {
                uint16_t l = buffer[0][i];
                uint16_t r = buffer[1][i];
                BufferToWriteTo->push_back(l & 0xFF);
                BufferToWriteTo->push_back(l >> 8);
                BufferToWriteTo->push_back(r & 0xFF);
                BufferToWriteTo->push_back(r >> 8);
            }

            return FLAC__STREAM_DECODER_WRITE_STATUS_CONTINUE;
        }

        void metadata_callback(const ::FLAC__StreamMetadata* metadata) override {
            auto streamInfo = &metadata->data.stream_info;

            DataType type;
            switch (streamInfo->bits_per_sample) {
                case 8:
                    type = DataType::U8;
                    break;

                case 16:
                    type = DataType::I16;
                    break;

                case 32:
                    type = DataType::I32;
                    break;

                default:
                    throw std::runtime_error("Unsupported bits per sample: " + std::to_string(streamInfo->bits_per_sample));
            }

            Channels channels;
            switch (streamInfo->channels) {
                case 1:
                    channels = Channels::Mono;
                    break;

                case 2:
                    channels = Channels::Stereo;
                    break;

                default:
                    throw std::runtime_error("Unsupported number of channels: " + std::to_string(streamInfo->channels));
            }

            Format = {
                .DataType = type,
                .SampleRate = streamInfo->sample_rate,
                .Channels = channels
            };

            MaxBufferSize = streamInfo->min_blocksize * streamInfo->channels * streamInfo->bits_per_sample / 8;
            TotalSamples = streamInfo->total_samples * streamInfo->channels * streamInfo->bits_per_sample / 8;
        }

        void error_callback(::FLAC__StreamDecoderErrorStatus status) override {
            std::cout << "Error: " << status << std::endl;
        }
    };

    Flac::Flac(const std::string& path) {
        _file = std::make_unique<FlacDecoder>();

        _file->init(path.c_str());
        _file->process_until_end_of_metadata();
    }

    AudioFormat Flac::Format() {
        return dynamic_cast<FlacDecoder*>(_file.get())->Format;
    }

    std::vector<uint8_t> Flac::GetPCM() {
        auto decoder = dynamic_cast<FlacDecoder*>(_file.get());

        std::vector<uint8_t> buffer;
        buffer.reserve(decoder->TotalSamples);

        decoder->BufferToWriteTo = &buffer;
        decoder->process_until_end_of_stream();
        decoder->BufferToWriteTo = nullptr;

        return buffer;
    }

}