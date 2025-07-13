#include <stdbool.h>
#include <stdio.h>

#include "stream_internal.h"
#include "../internal.h"

#define RIFF 0x46464952
#define WAVE 0x45564157
#define FMT  0x20746D66
#define DATA 0x61746164

#define READV(File, DataType, Name) DataType Name;\
    fread(&Name, sizeof(Name), 1, File)

#define READS(File, Size) fseek(File, Size, SEEK_CUR)

typedef struct
{
    FILE* wav;
    MxAudioFormat format;
    size_t dataStartPos;
    size_t dataSize;
} Wav;

void mxWAVDestroyStream(void* stream)
{
    Wav* wav = (Wav*) stream;
    fclose(wav->wav);
    free(wav);
}

size_t mxWAVGetDataSize(void* stream)
{
    const Wav* wav = (Wav*) stream;
    return wav->dataSize;
}

size_t mxWAVGetBuffer(void* stream, uint8_t* buffer, size_t length)
{
    Wav* wav = (Wav*) stream;

    const size_t currentPos = ftell(wav->wav);
    if (currentPos >= wav->dataStartPos + wav->dataSize)
        return 0;

    return fread(buffer, length, 1, wav->wav);
}

MxResult mxStreamLoadWav(const char* path, MxStream** stream)
{
    FILE* file = fopen(path, "rb");
    if (!file)
    {
        mxSetErrorString("File not found.");
        return MX_RESULT_FILE_NOT_FOUND;
    }

    Wav* wav = malloc(sizeof(Wav));
    wav->wav = file;

    READV(file, uint32_t, magic);
    if (magic != RIFF)
    {
        mxSetErrorString("Expected 'RIFF', was not found. It's most likely this file is not a WAV file.");
        free(wav);
        fclose(file);
        return MX_RESULT_MALFORMED_FILE;
    }

    READS(file, sizeof(uint32_t));

    READV(file, uint32_t, fileType);
    if (fileType != WAVE)
    {
        mxSetErrorString("Malformed file: Expected 'WAVE', was not found.");
        free(wav);
        fclose(file);
        return MX_RESULT_MALFORMED_FILE;
    }

    bool hasReadFmtChunk = false;
    bool hasReadDataChunk = false;
    MxAudioFormat format;

    while (!hasReadFmtChunk || !hasReadDataChunk)
    {
        READV(file, uint32_t, chunk);
        READV(file, uint32_t, chunkSize);

        switch (chunk)
        {
            case FMT:
            {
                READV(file, uint16_t, formatType);
                READV(file, uint16_t, numChannels);
                READV(file, uint32_t, sampleRate);
                READS(file, 6);
                READV(file, uint16_t, bitsPerSample);

                format.sampleRate = sampleRate;
                format.channels = (uint8_t) numChannels;

                switch (bitsPerSample)
                {
                    case 8:
                        format.dataType = MX_DATA_TYPE_I8;
                        break;
                    case 16:
                        format.dataType = MX_DATA_TYPE_I16;
                        break;
                    case 32:
                    {
                        switch (formatType)
                        {
                            case 1:
                                format.dataType = MX_DATA_TYPE_I32;
                                break;
                            case 3:
                                format.dataType = MX_DATA_TYPE_F32;
                                break;
                            default:
                                mxSetErrorString("Unsupported data type.");
                                free(wav);
                                fclose(file);
                                return MX_RESULT_UNSUPPORTED_FORMAT;
                        }
                        break;
                    }
                    default:
                        mxSetErrorString("Unsupported bits per sample.");
                        free(wav);
                        fclose(file);
                        return MX_RESULT_UNSUPPORTED_FORMAT;
                }

                hasReadFmtChunk = true;

                break;
            }

            case DATA:
            {
                wav->dataStartPos = ftell(file);
                wav->dataSize = chunkSize;

                hasReadDataChunk = true;
                break;
            }

            default:
                READS(file, chunkSize);
                break;
        }
    }

    fseek(file, wav->dataStartPos, SEEK_SET);

    Stream* s = malloc(sizeof(Stream));
    s->streamData = wav;
    s->format = format;
    s->destroyStream = mxWAVDestroyStream;
    s->getDataSize = mxWAVGetDataSize;
    s->getBuffer = mxWAVGetBuffer;

    *stream = (MxStream*) s;

    return MX_RESULT_OK;
}
