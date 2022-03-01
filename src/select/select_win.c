// Includes
#include <Winsock2.h>
#include <fcntl.h>
#include "select.h"


int libselect_select(struct libselect_fd* fds, size_t fds_len, uint64_t timeout_ms) {
    // Validate the input
    if (fds == NULL) {
        return 1;
    }
    WSASetLastError(0);
    
	// Create select-sets
	fd_set read_set, write_set, exception_set;
    FD_ZERO(&read_set);
    FD_ZERO(&write_set);
    FD_ZERO(&exception_set);

	// Populate sets
    SOCKET highest_fd = 0;
    for (size_t i = 0; i < fds_len; i++) {
        // Insert FD into sets
        struct libselect_fd* fd = fds + i;
        if (fd->read) {
            FD_SET((SOCKET)fd->handle, &read_set);
        }
        if (fd->write) {
            FD_SET((SOCKET)fd->handle, &write_set);
        }
        if (fd->exception) {
            FD_SET((SOCKET)fd->handle, &exception_set);
        }
        if (highest_fd < (SOCKET)fd->handle) {
            highest_fd = (SOCKET)fd->handle;
        }
    }

	// Create timeval-struct and call select
	struct timeval timeout = {
        .tv_sec = (long)(timeout_ms / 1000),
        .tv_usec = (long)((timeout_ms % 1000) * 1000)
    };
    if (select((int)highest_fd + 1, &read_set, &write_set, &exception_set, &timeout) == -1) {
        return WSAGetLastError();
    }

	// Copy the events
    for (size_t i = 0; i < fds_len; i++) {
        // Reset event flags
        struct libselect_fd* fd = fds + i;
        fd->read = false;
        fd->write = false;
        fd->exception = false;
        
        // Update the event flags
        if (FD_ISSET((SOCKET)fd->handle, &read_set)) {
            fd->read = true;
        }
        if (FD_ISSET((SOCKET)fd->handle, &write_set)) {
            fd->write = true;
        }
        if (FD_ISSET((SOCKET)fd->handle, &exception_set)) {
            fd->exception = true;
        }
    }
	return 0;
}


int set_blocking(uint64_t fd, bool blocking) {
	// Reset last error
	WSASetLastError(0);

	// Set blocking mode
	unsigned long mode = blocking ? 0 : 1;
    if (ioctlsocket((SOCKET)fd, FIONBIO, &mode) != 0) {
        return WSAGetLastError();
    }
	return 0;
}
