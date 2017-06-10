
AC_PREREQ([2.53])

AC_INIT([dlopen_rust], [0.1.0])
AM_INIT_AUTOMAKE([1.6 foreign subdir-objects])
m4_ifdef([AM_SILENT_RULES], [
     AM_SILENT_RULES([no])
])

AC_CANONICAL_HOST

AC_PROG_CC_C99
AM_PROG_CC_C_O

LT_INIT

AC_CONFIG_MACRO_DIRS([m4])

AC_MSG_CHECKING(for --enable-rust)
AC_ARG_ENABLE(rust, AS_HELP_STRING([--enable-rust], [Enable rust language features (default: no)]),
[
    AC_CHECK_PROG(CARGO, [cargo], [yes], [no])
    AC_CHECK_PROG(RUSTC, [rustc], [yes], [no])

    AS_IF([test "$CARGO" != "yes" -o "$RUSTC" != "yes"], [
      AC_MSG_FAILURE("Rust based plugins cannot be built $CARGO $RUSTC")
    ])
    with_rust=yes
    AC_MSG_RESULT(yes)
],
[
    AC_MSG_RESULT(no)
])
AM_CONDITIONAL([RUST_ENABLE],[test -n "$with_rust"])


AC_MSG_CHECKING(for --enable-debug)
AC_ARG_ENABLE(debug, AS_HELP_STRING([--enable-debug], [Enable debug features (default: no)]),
[
  AC_MSG_RESULT(yes)
  with_debug=yes
  debug_rust_defs=""
  debug_cargo_defs=""
  debug_c_defs="-g3 -DDEBUG -O0"
],
[
  AC_MSG_RESULT(no)
  # We always supply debug info, for rpm to extract
  debug_rust_defs="-C debuginfo"
  debug_cargo_defs="--release"
  debug_c_defs=""
])
AM_CONDITIONAL([DEBUG],[test -n "$with_debug"])

AC_MSG_CHECKING(for --enable-asan)
AC_ARG_ENABLE(asan, AS_HELP_STRING([--enable-asan], [Enable ASAN features (default: no)]),
[
    AC_MSG_RESULT(yes)
    with_asan=yes
    asan_rust_defs="-Z sanitizer=address"
    asan_c_defs="-fsanitize=address -fno-omit-frame-pointer"
],
[
    AC_MSG_RESULT(no)
])
AM_CONDITIONAL([ASAN],[test -n "$with_asan"])

AC_SUBST([debug_rust_defs])
AC_SUBST([debug_cargo_defs])
AC_SUBST([debug_c_defs])
AC_SUBST([asan_rust_defs])
AC_SUBST([asan_c_defs])


AC_CONFIG_FILES([
  Makefile
])

AC_OUTPUT




