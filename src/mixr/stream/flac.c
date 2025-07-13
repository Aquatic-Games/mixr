#include "mixr/stream.h"

#include <FLAC/stream_decoder.h>

#include "stream_internal.h"
#include "../internal.h"

typedef struct
{
    FLAC__StreamDecoder* decoder;
} Flac;

FLAC__StreamDecoderWriteStatus mxFLACWriteCallback(const FLAC__StreamDecoder* decoder, const FLAC__Frame* frame, const FLAC__int32* const buffer[], void* clientData)
{

}

void mxFLACMetadataCallback(const FLAC__StreamDecoder* decoder, const FLAC__StreamMetadata* metadata, void* clientData)
{
    Stream* s = (Stream*) clientData;
    Flac* flac = (Flac*) s->streamData;

    MxAudioFormat format;

    switch (metadata->type)
    {
        case FLAC__METADATA_TYPE_STREAMINFO:
        {
            const FLAC__StreamMetadata_StreamInfo* info = &metadata->data.stream_info;
            format.sampleRate = info->sample_rate;
            format.channels = (uint8_t) info->channels;

            switch (info->bits_per_sample)
            {
                //case 8:
                //    format.dataType = MX_DATA_TYPE_U8;
                //    break;
                case 16:
                    format.dataType = MX_DATA_TYPE_I16;
                    break;
                case 32:
                    format.dataType = MX_DATA_TYPE_I32;
                    break;
            }

            s->format = format;

            break;
        }
    }
}

void mxFLACErrorCallback(const FLAC__StreamDecoder* decoder, FLAC__StreamDecoderErrorStatus status, void* clientData)
{

}

void mxFLACDestroyStream(void* stream)
{

}

size_t mxFLACGetDataSize(void* stream)
{

}

size_t mxFLACGetBuffer(void* stream, uint8_t* buffer, size_t length)
{
    return 0;
}

MxResult mxStreamLoadFlac(const char* path, MxStream** stream)
{
    FLAC__StreamDecoder* decoder = FLAC__stream_decoder_new();

    if (!decoder)
    {
        mxSetErrorString("Failed to create FLAC decoder.");
        return MX_RESULT_UNKNOWN_ERROR;
    }

    Stream* s = malloc(sizeof(Stream));
    s->destroyStream = mxFLACDestroyStream;
    s->getDataSize = mxFLACGetDataSize;

    const FLAC__StreamDecoderInitStatus status =
        FLAC__stream_decoder_init_file(decoder, path, mxFLACWriteCallback, mxFLACMetadataCallback, mxFLACErrorCallback, s);

    if (status != FLAC__STREAM_DECODER_INIT_STATUS_OK)
    {
        if (status == FLAC__STREAM_DECODER_INIT_STATUS_ERROR_OPENING_FILE)
        {
            mxSetErrorString("File not found.");
            return MX_RESULT_FILE_NOT_FOUND;
        }
        else
        {
            mxSetErrorString("Error creating flac decoder. See stdout.");
            printf("%s\n", FLAC__StreamDecoderInitStatusString[status]);
            return MX_RESULT_UNKNOWN_ERROR;
        }
    }

    Flac* flac = malloc(sizeof(Flac));
    s->streamData = flac;
    flac->decoder = decoder;

    FLAC__stream_decoder_process_until_end_of_metadata(decoder);

    *stream = (MxStream*) s;

    return MX_RESULT_OK;
}
