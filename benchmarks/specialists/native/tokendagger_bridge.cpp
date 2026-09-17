#include "native_bridge.h"

#include <string>
#include <utility>
#include <vector>

#include "tiktoken.hpp"
#include "tiktoken.cpp"

namespace {

static_assert(sizeof(int) == sizeof(int32_t), "TokenDagger token IDs require 32-bit int");

// Owns the immutable public encoder and its configured special-token filter.
struct TokenDaggerHandle {
    tiktoken::CoreBPE encoding;
    emhash8::HashSet<std::string> allowed_special;

    // Constructs the public CoreBPE once, outside all benchmark timers.
    TokenDaggerHandle(const std::string& pattern, const std::vector<VocabItem>& vocab,
                      const std::vector<VocabItem>& special_vocab)
        : encoding(pattern, vocab, special_vocab) {
        allowed_special.reserve(special_vocab.size());
        for (const auto& item : special_vocab) {
            // HashSet::emplace_unique is broken at this pinned revision; the
            // ordinary public insert overload preserves ownership correctly.
            allowed_special.insert(item.token_string);
        }
    }
};

// Clears an output slot before a bridge call attempts to populate it.
void clear_buffer(NativeBuffer* buffer) {
    *buffer = NativeBuffer{nullptr, 0, nullptr};
}

// Copies an exception message into an owned buffer that can cross the C ABI.
void store_error(NativeBuffer* error, const char* message) noexcept {
    try {
        auto* owned = new std::string(message);
        *error = NativeBuffer{owned->data(), owned->size(), owned};
    } catch (...) {
        clear_buffer(error);
    }
}

} // namespace

// Constructs TokenDagger's public CoreBPE from one pinned model definition.
extern "C" void* st_tokendagger_create(const uint8_t* pattern, size_t pattern_len,
                                       const NativeRankEntry* ranks, size_t rank_count,
                                       const NativeSpecialEntry* specials, size_t special_count,
                                       NativeBuffer* error) noexcept {
    clear_buffer(error);
    try {
        std::vector<VocabItem> vocab;
        vocab.reserve(rank_count);
        for (size_t index = 0; index < rank_count; ++index) {
            const auto& entry = ranks[index];
            std::vector<unsigned char> bytes(entry.data, entry.data + entry.len);
            vocab.push_back(VocabItem{static_cast<int>(entry.rank), std::move(bytes), {}});
        }

        std::vector<VocabItem> special_vocab;
        special_vocab.reserve(special_count);
        for (size_t index = 0; index < special_count; ++index) {
            const auto& entry = specials[index];
            std::string token(reinterpret_cast<const char*>(entry.data), entry.len);
            std::vector<unsigned char> bytes(entry.data, entry.data + entry.len);
            special_vocab.push_back(VocabItem{static_cast<int>(entry.id), std::move(bytes), token});
        }

        std::string regex(reinterpret_cast<const char*>(pattern), pattern_len);
        return new TokenDaggerHandle(regex, vocab, special_vocab);
    } catch (const std::exception& exception) {
        store_error(error, exception.what());
    } catch (...) {
        store_error(error, "unknown TokenDagger constructor error");
    }
    return nullptr;
}

// Encodes one string through TokenDagger's public scalar CoreBPE::encode API.
extern "C" int st_tokendagger_encode(void* opaque, const uint8_t* text, size_t text_len,
                                     NativeBuffer* output, NativeBuffer* error) noexcept {
    clear_buffer(output);
    clear_buffer(error);
    try {
        auto* handle = static_cast<TokenDaggerHandle*>(opaque);
        std::string input(reinterpret_cast<const char*>(text), text_len);
        auto encoded = handle->encoding.encode(input, handle->allowed_special);
        auto* ids = new std::vector<int>(std::move(encoded.first));
        *output = NativeBuffer{ids->data(), ids->size(), ids};
        return 0;
    } catch (const std::exception& exception) {
        store_error(error, exception.what());
    } catch (...) {
        store_error(error, "unknown TokenDagger encode error");
    }
    return 1;
}

// Releases one vector returned by st_tokendagger_encode.
extern "C" void st_tokendagger_release_output(NativeBuffer output) noexcept {
    delete static_cast<std::vector<int>*>(output.owner);
}

// Releases one error string returned by a TokenDagger bridge call.
extern "C" void st_tokendagger_release_error(NativeBuffer error) noexcept {
    delete static_cast<std::string*>(error.owner);
}

// Destroys one persistent TokenDagger encoder.
extern "C" void st_tokendagger_destroy(void* opaque) noexcept {
    delete static_cast<TokenDaggerHandle*>(opaque);
}

// Reports identity from the exact PCRE2 library loaded by this process.
extern "C" int st_tokendagger_pcre2_info(char* version, size_t version_capacity,
                                         uint32_t* jit_available, uint32_t* header_major,
                                         uint32_t* header_minor) noexcept {
    if (version == nullptr || jit_available == nullptr || header_major == nullptr ||
        header_minor == nullptr) {
        return 1;
    }
    const int required = pcre2_config(PCRE2_CONFIG_VERSION, nullptr);
    if (required < 0 || static_cast<size_t>(required) > version_capacity) {
        return 2;
    }
    if (pcre2_config(PCRE2_CONFIG_VERSION, version) < 0) {
        return 3;
    }
    if (pcre2_config(PCRE2_CONFIG_JIT, jit_available) < 0) {
        return 4;
    }
    *header_major = PCRE2_MAJOR;
    *header_minor = PCRE2_MINOR;
    return 0;
}
