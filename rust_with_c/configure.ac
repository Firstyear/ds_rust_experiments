AC_PREREQ([2.53])

AC_INIT([rwc], [0.1.0])
AM_INIT_AUTOMAKE([1.6 foreign subdir-objects])
m4_ifdef([AM_SILENT_RULES], [
     AM_SILENT_RULES([yes])
])

AC_CANONICAL_HOST

AC_PROG_CC_C99
AM_PROG_CC_C_O

AC_CHECK_PROG(CARGO, [cargo], [yes], [no])
AC_CHECK_PROG(RUSTC, [rustc], [yes], [no])

AS_IF([test "$CARGO" != "yes" -o "$RUSTC" != "yes"], [
  AC_MSG_FAILURE("Rust based plugins cannot be built $CARGO $RUSTC")
])

LT_INIT

AC_CONFIG_MACRO_DIRS([m4])

AC_MSG_CHECKING(for --enable-debug)
AC_ARG_ENABLE(debug, AS_HELP_STRING([--enable-debug], [Enable debug features (default: no)]),
[
  AC_MSG_RESULT(yes)
  with_debug=yes
],
[
  AC_MSG_RESULT(no)
])
AM_CONDITIONAL([DEBUG],[test -n "$with_debug"])


AC_CONFIG_FILES([
  Makefile
])

AC_OUTPUT

