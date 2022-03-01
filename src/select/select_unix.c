// Includes
#include <sys/select.h>
#include <sys/socket.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include "select.h"


int libselect_select(struct libselect_fd* fds, size_t fds_len, uint64_t timeout_ms) {
    // Validate the input
    if (fds == NULL) {
        return EFAULT;
    }
    errno = 0;

    // Create select-sets
    fd_set read_set, write_set, exception_set;
    FD_ZERO(&read_set);
    FD_ZERO(&write_set);
    FD_ZERO(&exception_set);

    // Populate sets
    int highest_fd = 0;
    for (size_t i = 0; i < fds_len; i++) {
        // Insert FD into sets
        struct libselect_fd* fd = fds + i;
        if (fd->read) {
            FD_SET((int)fd->handle, &read_set);
        }
        if (fd->write) {
            FD_SET((int)fd->handle, &write_set);
        }
        if (fd->exception) {
            FD_SET((int)fd->handle, &exception_set);
        }
        if (highest_fd < (int)fd->handle) {
            highest_fd = (int)fd->handle;
        }
    }

    // Create timeval-struct and call select
    struct timeval timeout = {
        .tv_sec = timeout_ms / 1000,
        .tv_usec = (timeout_ms % 1000) * 1000
    };
    if (select(highest_fd + 1, &read_set, &write_set, &exception_set, &timeout) == -1) {
        return errno;
    }

    // Copy the events
    for (size_t i = 0; i < fds_len; i++) {
        // Reset event flags
        struct libselect_fd* fd = fds + i;
        fd->read = false;
        fd->write = false;
        fd->exception = false;
        
        // Update the event flags
        if (FD_ISSET((int)fd->handle, &read_set)) {
            fd->read = true;
        }
        if (FD_ISSET((int)fd->handle, &write_set)) {
            fd->write = true;
        }
        if (FD_ISSET((int)fd->handle, &exception_set)) {
            fd->exception = true;
        }
    }
    return 0;
}


int set_blocking(uint64_t fd, bool blocking) {
    // Reset errno
    errno = 0;

    // Get current flags
    int flags = fcntl((int)fd, F_GETFL, 0);
    if (flags == -1) {
        return errno;
    }

    // Add new flag and call fcntl
    flags = blocking ? (flags & ~O_NONBLOCK) : (flags | O_NONBLOCK);
    if (fcntl((int)fd, F_SETFL, flags) == -1) {
        return errno;
    }
    return 0;
}
