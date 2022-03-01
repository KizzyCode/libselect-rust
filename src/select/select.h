#include <stdint.h>
#include <stdbool.h>


/**
 * @brief A struct holding a file descriptor together with it's event flags
 */
struct libselect_fd {
    /**
     * @brief The file descriptor to operate on
     */
    uint64_t handle;
    /**
     * @brief A flag for the `read`-event
     */
    bool read;
    /**
     * @brief A flag for the write event
     */
    bool write;
    /**
     * @brief A flag for an exceptional event (usually an error)
     */
    bool exception;
};


/**
 * @brief Calls `select` with the given file descriptors and timeout
 * 
 * @param fds The file descriptors and events to wait on
 * @param fds_len The amount of file descriptors
 * @param timeout_ms The maximum amount of time to wait for an event in milliseconds
 * @return int `0` on success or an appropriate `errno` on error
 */
int libselect_select(struct libselect_fd* fds, size_t fds_len, uint64_t timeout_ms);


/**
 * @brief Set the blocking object
 * 
 * @param fd The file descriptor to operate on
 * @param blocking Whether to make the file descriptor blocking or non-blocking
 * @return int `0` on success or an appropriate `errno` on error
 */
int set_blocking(uint64_t fd, bool blocking);
