#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Service Service;

/**
 * A boxed value represented by a pointer
 *
 * This struct wraps a value in a `Box`, but exposes
 * it as a pointer. The pointer can never be null.
 *
 * For optional values (and nullable pointers), see [FNullableBox]
 */
typedef struct Service *FBox_Service;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

FBox_Service service_new(void);

void service_delete(FBox_Service opaque);

int32_t service_get_value(const struct Service *opaque);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
