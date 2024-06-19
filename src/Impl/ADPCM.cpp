#include "ADPCM.h"

inline int Clamp(int value, int min, int max) {
    return value <= min ? min : value >= max ? max : value;
}

namespace mixr::ADPCM {
    void DecodeIMAChunk(uint8_t* inBuffer, size_t chunkSize, uint8_t* outBuffer, bool stereo) {
        int channels = stereo ? 2 : 1;

        int predictor[channels];
        int stepIndex[channels];
        int step[channels];

        size_t newIndex[channels];
        if (stereo)
            newIndex[1] = 2;

        int chunkOffset = 4;
        predictor[0] = (uint16_t) (inBuffer[0] | inBuffer[1] << 8);
        stepIndex[0] = inBuffer[2];
        step[0] = StepTable[stepIndex[0]];

        if (stereo) {
            chunkOffset = 8;
            predictor[1] = (uint16_t) (inBuffer[4] | inBuffer[5] << 8);
            stepIndex[1] = inBuffer[6];
            step[1] = StepTable[stepIndex[1]];
        }

        for (size_t i = chunkOffset; i < chunkSize; i++) {
            uint8_t b = inBuffer[i];

            int channel = 0;
            if (stereo) {
                // TODO: Can condense this into a single statement and remove the ternary, I think.
                bool isRightChannel = (i % 8) / 4 == 1;
                channel = isRightChannel ? 1 : 0;
            }

            for (int n = 0; n < 2; n++) {
                uint8_t nibble = (uint8_t) (n == 1 ? (b >> 4) & 0xF : (b & 0xF));

                int sign = nibble & 8;
                int delta = nibble & 7;
                int diff = step[channel] >> 3;
                if ((delta & 4) == 4)
                    diff += step[channel];
                if ((delta & 2) == 2)
                    diff += step[channel] >> 1;
                if ((delta & 1) == 1)
                    diff += step[channel] >> 2;
                if (sign)
                    predictor[channel] += diff;
                else
                    predictor[channel] -= diff;

                predictor[channel] = Clamp(predictor[channel], INT16_MIN, INT16_MAX);

                stepIndex[channel] += IndexTable[nibble];
                stepIndex[channel] = Clamp(stepIndex[channel], 0, 88);
                step[channel] = StepTable[stepIndex[channel]];

                uint16_t currentPredictor = (uint16_t) predictor[channel];

                outBuffer[newIndex[channel]] = (uint8_t) (currentPredictor & 0xFF);
                outBuffer[newIndex[channel] + 1] = (uint8_t) (currentPredictor >> 8);

                newIndex[channel] += channels * 2;
            }
        }
    }
}