/*
 * Copyright 2022 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_GENRES_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_GENRES_H_

// This should probably be called "common_enums.h" or something like that.

#include <type_traits>

namespace patinon {
namespace pvn_parsing {

// Should probably change to textual vs code. Examine uses.
enum class ParsingGenre {
  kNone,
  kSalient,
  kQuarrel,
};

enum class TextualSubGenre {
  kNone,
  kMaster,
  kTextualRight,
  kTextualIndent,
  kTextualLeft,
};

enum class SvtListType {
  kNone = 0,
  kBullet,
  kArabic,
  kLowerAlpha,
  kUpperAlpha,
  kLowerRoman,  // Since these are confusable, say ":i.", maybe best annotate.
  kUpperRoman,  // Rule is that ivx char combination in first means roman.
  kListBreak,
};

enum class SvtListAttachment {
  kNone = 0,
  kAttached,
  kDetached,
};

// X

enum class SvtListCompactness {
  kNone = 0,
  kCompact,
  kBlock,
  kSimple,  // Only for list items, not meaningful for lists themselves.
};

enum class StylingFlags {
  kNone = 0,
  kDoubleMath = 1 << 0,
  kDoubleTt = 1 << 1,
  kDoubleSQuote = 1 << 2,
  kDoubleDQuote = 1 << 3,
  kDoubleBold = 1 << 4,
  kDoubleEmph = 1 << 5,
  kDoubleUnderline = 1 << 6,
  kDoubleStrike = 1 << 7,
  kDoubleSemiVerb = 1 << 8,
};

inline StylingFlags operator|(StylingFlags lhs, StylingFlags rhs) {
  return static_cast<StylingFlags>(
      static_cast<std::underlying_type<StylingFlags>::type>(lhs) |
      static_cast<std::underlying_type<StylingFlags>::type>(rhs));
}

inline StylingFlags operator&(StylingFlags lhs, StylingFlags rhs) {
  return static_cast<StylingFlags>(
      static_cast<std::underlying_type<StylingFlags>::type>(lhs) &
      static_cast<std::underlying_type<StylingFlags>::type>(rhs));
}

inline StylingFlags operator^(StylingFlags lhs, StylingFlags rhs) {
  return static_cast<StylingFlags>(
      static_cast<std::underlying_type<StylingFlags>::type>(lhs) ^
      static_cast<std::underlying_type<StylingFlags>::type>(rhs));
}

inline StylingFlags operator~(StylingFlags rhs) {
  return static_cast<StylingFlags>(
      ~static_cast<std::underlying_type<StylingFlags>::type>(rhs));
}

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_GENRES_H_
