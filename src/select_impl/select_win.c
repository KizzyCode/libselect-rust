// Includes
#include <stdint.h>
#include <Winsock2.h>
#include <fcntl.h>


/// A libselect filedescriptor struct
struct libselect_fd {
    /// The file descriptor handle to operate on
    uint64_t handle;
    /// A flag for the `read`-event
    uint8_t read;
    /// A flag for the `write`-event
    uint8_t write;
    /// A flag for an exceptional event
    uint8_t exception;
};


/// Calls `select` with the given `fds`
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
        fd->read = 0;
        fd->write = 0;
        fd->exception = 0;
        
        // Update the event flags
        if (FD_ISSET((SOCKET)fd->handle, &read_set)) {
            fd->read = 1;
        }
        if (FD_ISSET((SOCKET)fd->handle, &write_set)) {
            fd->write = 1;
        }
        if (FD_ISSET((SOCKET)fd->handle, &exception_set)) {
            fd->exception = 1;
        }
    }
	return 0;
}

/// Sets the blocking mode for the given FD
int set_blocking(uint64_t fd, uint8_t blocking) {
	// Reset last error
	WSASetLastError(0);

	// Set blocking mode
	unsigned long mode = blocking ? 0 : 1;
    if (ioctlsocket((SOCKET)fd, FIONBIO, &mode) != 0) {
        return WSAGetLastError();
    }
	return 0;
}
