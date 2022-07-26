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

#ifndef BASE_DIR_PATINON_EXPLORATORY_MISC_CHECK_MACROS_H_
#define BASE_DIR_PATINON_EXPLORATORY_MISC_CHECK_MACROS_H_

#include <cstdio>
#include <cstdlib>
#include <type_traits>

namespace pvn {
namespace check_macros {

constexpr int kValueBufSize = 32;

template <typename T, typename Enable = void>
struct ToString {
  static void Run(const T&, char* buf) { snprintf(buf, kValueBufSize, "(?)"); }
};

template <>
struct ToString<float, void> {
  static void Run(float value, char* buf) {
    snprintf(buf, kValueBufSize, "%.9g", static_cast<double>(value));
  }
};

template <>
struct ToString<double, void> {
  static void Run(double value, char* buf) {
    snprintf(buf, kValueBufSize, "%.16g", value);
  }
};

template <typename T>
struct ToString<T, typename std::enable_if<std::is_integral<T>::value>::type> {
  static void Run(const T& value, char* buf) {
    snprintf(buf, kValueBufSize, "%lld", static_cast<long long>(value));
  }
};

template <typename T>
struct ToString<T*, void> {
  static void Run(T* value, char* buf) {
    snprintf(buf, kValueBufSize, "%p", value);
  }
};

template <typename T>
struct ToString<T, typename std::enable_if<std::is_enum<T>::value>::type> {
  static void Run(const T& value, char* buf) {
    snprintf(buf, kValueBufSize, "(enum value %d)", static_cast<int>(value));
  }
};

inline void Failure(const char* file, int line, const char* macro,
                    const char* condition) {
  fprintf(stderr, "%s:%d: %s condition not satisfied: %s\n", file, line, macro,
          condition);
  abort();
}

template <typename LhsType, typename RhsType>
inline void Failure(const char* file, int line, const char* macro,
                    const char* lhs, const LhsType& lhs_value, const char* op,
                    const char* rhs, const RhsType& rhs_value) {
  char lhs_value_buf[kValueBufSize];
  ToString<LhsType>::Run(lhs_value, lhs_value_buf);
  char rhs_value_buf[kValueBufSize];
  ToString<RhsType>::Run(rhs_value, rhs_value_buf);
  fprintf(stderr,
          "%s:%d: %s condition not satisfied:   [ %s %s %s ]   with values   [ "
          "%s %s %s ].\n",
          file, line, macro, lhs, op, rhs, lhs_value_buf, op, rhs_value_buf);
  abort();
}

#define PVN_CHECK_IMPL(macro, condition)                                  \
  do {                                                                    \
    if (!(condition)) {                                                   \
      pvn::check_macros::Failure(__FILE__, __LINE__, #macro, #condition); \
    }                                                                     \
  } while (false)

#define PVN_CHECK_OP_IMPL(macro, lhs, op, rhs)                                \
  do {                                                                        \
    const auto& lhs_value = (lhs);                                            \
    const auto rhs_value = (rhs);                                             \
    if (!(lhs_value op rhs_value)) {                                          \
      pvn::check_macros::Failure(__FILE__, __LINE__, #macro, #lhs, lhs_value, \
                                 #op, #rhs, rhs_value);                       \
    }                                                                         \
  } while (false)

#define PVN_CHECK(condition) PVN_CHECK_IMPL(PVN_CHECK, condition)
#define PVN_CHECK_EQ(x, y) PVN_CHECK_OP_IMPL(PVN_CHECK_EQ, x, ==, y)
#define PVN_CHECK_NE(x, y) PVN_CHECK_OP_IMPL(PVN_CHECK_NE, x, !=, y)
#define PVN_CHECK_GE(x, y) PVN_CHECK_OP_IMPL(PVN_CHECK_GE, x, >=, y)
#define PVN_CHECK_GT(x, y) PVN_CHECK_OP_IMPL(PVN_CHECK_GT, x, >, y)
#define PVN_CHECK_LE(x, y) PVN_CHECK_OP_IMPL(PVN_CHECK_LE, x, <=, y)
#define PVN_CHECK_LT(x, y) PVN_CHECK_OP_IMPL(PVN_CHECK_LT, x, <, y)

#ifdef NDEBUG
#define PVN_DCHECK_IS_ENABLED false
#else
#define PVN_DCHECK_IS_ENABLED true
#endif

#define PVN_DCHECK(condition) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK(condition)
#define PVN_DCHECK_EQ(x, y) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK_EQ(x, y)
#define PVN_DCHECK_NE(x, y) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK_NE(x, y)
#define PVN_DCHECK_GE(x, y) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK_GE(x, y)
#define PVN_DCHECK_GT(x, y) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK_GT(x, y)
#define PVN_DCHECK_LE(x, y) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK_LE(x, y)
#define PVN_DCHECK_LT(x, y) \
  if (PVN_DCHECK_IS_ENABLED) PVN_CHECK_LT(x, y)

}  // end namespace check_macros
}  // end namespace pvn

#endif  // BASE_DIR_PATINON_EXPLORATORY_MISC_CHECK_MACROS_H_
