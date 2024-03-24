#include <cstring> // for std::memcpy
#include <iostream>
#include <string>
#include <vector>

#include "msbt_bindings.h"
#include "msbt/msbt.h" // Updated include path

extern "C"
{
    // Convert std::string to binary
    unsigned char *cxx_string_to_binary(const char *text, size_t *length)
    {
        std::string str(text);
        auto msbt = oepd::msbt::FromText(str);
        auto binary = msbt.ToBinary();

        *length = binary.size();
        unsigned char *c_binary = new unsigned char[binary.size()];
        std::memcpy(c_binary, binary.data(), binary.size());

        return c_binary;
    }

    // Convert binary to std::string
    const char *cxx_binary_to_string(const uint8_t *binary, size_t length)
    {
        std::vector<unsigned char> vec(binary, binary + length);
        auto msbt = oepd::msbt::FromBinary(vec);
        auto str = msbt.ToText();

        char *cstr = new char[str.size() + 1];
        std::memcpy(cstr, str.c_str(), str.size() + 1);

        return cstr;
    }
    void free_cxx_string(char *str)
    {
        delete[] str;
    }

    void free_cxx_binary(unsigned char *binary)
    {
        delete[] binary;
    }
}