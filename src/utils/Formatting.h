#pragma once

#include <string>

namespace formatting {

/// Format a value with appropriate SI prefix and significant digits.
std::string format_value(double value, const std::string& unit);

/// Short alias for format_value.
inline std::string eng(double value, const std::string& unit) {
    return format_value(value, unit);
}

} // namespace formatting
