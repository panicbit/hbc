// Generated by gir (https://github.com/gtk-rs/gir @ 74b6e47217b7)
// from /usr/share/gir-1.0 (@ ???)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) FLATPAK_ERROR_ABORTED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_ALREADY_INSTALLED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_AUTHENTICATION_FAILED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_DIFFERENT_REMOTE);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_DOWNGRADE);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_EXPORT_FAILED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_INVALID_DATA);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_INVALID_NAME);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_INVALID_REF);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_NEED_NEW_FLATPAK);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_NOT_AUTHORIZED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_NOT_CACHED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_NOT_INSTALLED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_ONLY_PULLED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_OUT_OF_SPACE);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_PERMISSION_DENIED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_REF_NOT_FOUND);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_REMOTE_NOT_FOUND);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_REMOTE_USED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_RUNTIME_NOT_FOUND);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_RUNTIME_USED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_SETUP_FAILED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_SKIPPED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_UNTRUSTED);
    PRINT_CONSTANT((gint) FLATPAK_ERROR_WRONG_USER);
    PRINT_CONSTANT((guint) FLATPAK_INSTALL_FLAGS_NONE);
    PRINT_CONSTANT((guint) FLATPAK_INSTALL_FLAGS_NO_DEPLOY);
    PRINT_CONSTANT((guint) FLATPAK_INSTALL_FLAGS_NO_PULL);
    PRINT_CONSTANT((guint) FLATPAK_INSTALL_FLAGS_NO_STATIC_DELTAS);
    PRINT_CONSTANT((guint) FLATPAK_INSTALL_FLAGS_NO_TRIGGERS);
    PRINT_CONSTANT((guint) FLATPAK_LAUNCH_FLAGS_DO_NOT_REAP);
    PRINT_CONSTANT((guint) FLATPAK_LAUNCH_FLAGS_NONE);
    PRINT_CONSTANT(FLATPAK_MAJOR_VERSION);
    PRINT_CONSTANT(FLATPAK_MICRO_VERSION);
    PRINT_CONSTANT(FLATPAK_MINOR_VERSION);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_CANCELLED);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_EXISTS);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_FAILED);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_INVALID_ARGUMENT);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_NOT_ALLOWED);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_NOT_FOUND);
    PRINT_CONSTANT((gint) FLATPAK_PORTAL_ERROR_WINDOW_DESTROYED);
    PRINT_CONSTANT((guint) FLATPAK_QUERY_FLAGS_ALL_ARCHES);
    PRINT_CONSTANT((guint) FLATPAK_QUERY_FLAGS_NONE);
    PRINT_CONSTANT((guint) FLATPAK_QUERY_FLAGS_ONLY_CACHED);
    PRINT_CONSTANT((guint) FLATPAK_QUERY_FLAGS_ONLY_SIDELOADED);
    PRINT_CONSTANT((gint) FLATPAK_REF_KIND_APP);
    PRINT_CONSTANT((gint) FLATPAK_REF_KIND_RUNTIME);
    PRINT_CONSTANT((gint) FLATPAK_REMOTE_TYPE_LAN);
    PRINT_CONSTANT((gint) FLATPAK_REMOTE_TYPE_STATIC);
    PRINT_CONSTANT((gint) FLATPAK_REMOTE_TYPE_USB);
    PRINT_CONSTANT((gint) FLATPAK_STORAGE_TYPE_DEFAULT);
    PRINT_CONSTANT((gint) FLATPAK_STORAGE_TYPE_HARD_DISK);
    PRINT_CONSTANT((gint) FLATPAK_STORAGE_TYPE_MMC);
    PRINT_CONSTANT((gint) FLATPAK_STORAGE_TYPE_NETWORK);
    PRINT_CONSTANT((gint) FLATPAK_STORAGE_TYPE_SDCARD);
    PRINT_CONSTANT((guint) FLATPAK_TRANSACTION_ERROR_DETAILS_NON_FATAL);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_OPERATION_INSTALL);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_OPERATION_INSTALL_BUNDLE);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_OPERATION_LAST_TYPE);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_OPERATION_UNINSTALL);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_OPERATION_UPDATE);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_REMOTE_GENERIC_REPO);
    PRINT_CONSTANT((gint) FLATPAK_TRANSACTION_REMOTE_RUNTIME_DEPS);
    PRINT_CONSTANT((guint) FLATPAK_TRANSACTION_RESULT_NO_CHANGE);
    PRINT_CONSTANT((guint) FLATPAK_UNINSTALL_FLAGS_NONE);
    PRINT_CONSTANT((guint) FLATPAK_UNINSTALL_FLAGS_NO_PRUNE);
    PRINT_CONSTANT((guint) FLATPAK_UNINSTALL_FLAGS_NO_TRIGGERS);
    PRINT_CONSTANT((guint) FLATPAK_UPDATE_FLAGS_NONE);
    PRINT_CONSTANT((guint) FLATPAK_UPDATE_FLAGS_NO_DEPLOY);
    PRINT_CONSTANT((guint) FLATPAK_UPDATE_FLAGS_NO_PRUNE);
    PRINT_CONSTANT((guint) FLATPAK_UPDATE_FLAGS_NO_PULL);
    PRINT_CONSTANT((guint) FLATPAK_UPDATE_FLAGS_NO_STATIC_DELTAS);
    PRINT_CONSTANT((guint) FLATPAK_UPDATE_FLAGS_NO_TRIGGERS);
    return 0;
}
