# TokenDagger GPT-OSS exclusion

Revision [`1eed815d1ef3a23a73b0d6554f99fc0be6ff4164`](https://github.com/M4THYOU/TokenDagger/tree/1eed815d1ef3a23a73b0d6554f99fc0be6ff4164) is excluded from the GPT-OSS/o200k specialist benchmark without executing its 21-special-token path. GPT-2/r50k remains eligible for the normal full-ID gate.

## Source gate

[`CoreBPE::find_next_special_token`](https://github.com/M4THYOU/TokenDagger/blob/1eed815d1ef3a23a73b0d6554f99fc0be6ff4164/src/tiktoken/tiktoken.cpp#L130-L152) iterates `next_special_cache` with a range-for loop. When a special token is absent, the loop erases that current key and then continues iterating the same map.

The pinned map's [`erase`](https://github.com/M4THYOU/TokenDagger/blob/1eed815d1ef3a23a73b0d6554f99fc0be6ff4164/src/tiktoken/hash_table8.hpp#L1267-L1281) moves the last occupied pair into the erased slot and decrements the filled count. Continuing the invalidated range iteration can skip or misread entries. GPT-OSS supplies 21 allowed specials, most of which are absent from an ordinary input, so this path is not admitted as a safe public tokenizer call.

The benchmark does not patch the competitor, convert the map, or claim a measured GPT-OSS output. Each GPT-OSS cell emits `excluded_source_undefined_behavior` and points back to this audit. A future upstream revision can enter the ordinary parity gate after the invalidating erase is removed.
