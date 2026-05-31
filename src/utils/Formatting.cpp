#include "Formatting.h"
#include <cmath>
#include <cstdio>

namespace formatting {

std::string format_value(double value, const std::string& unit) {
    double abs_val = std::abs(value);
    if (abs_val == 0.0) return "0 " + unit;

    double scaled;
    const char* prefix;

    if (abs_val >= 1'000'000.0)       { scaled = value / 1'000'000.0;  prefix = "M"; }
    else if (abs_val >= 1'000.0)      { scaled = value / 1'000.0;      prefix = "k"; }
    else if (abs_val >= 1.0)          { scaled = value;                prefix = ""; }
    else if (abs_val >= 0.001)        { scaled = value * 1'000.0;      prefix = "m"; }
    else if (abs_val >= 0.000'001)    { scaled = value * 1'000'000.0;  prefix = "μ"; }
    else if (abs_val >= 1e-9)         { scaled = value * 1e9;          prefix = "n"; }
    else                              { scaled = value * 1e12;         prefix = "p"; }

    int decimals;
    if (std::abs(scaled) >= 100.0)       decimals = 1;
    else if (std::abs(scaled) >= 10.0)   decimals = 2;
    else                                  decimals = 3;

    char buf[64];
    std::snprintf(buf, sizeof(buf), "%.*f %s%s", decimals, scaled, prefix, unit.c_str());
    return std::string(buf);
}

} // namespace formatting
