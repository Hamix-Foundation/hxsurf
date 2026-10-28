#ifndef NETSURF_HAMIX_SCHEDULE_H
#define NETSURF_HAMIX_SCHEDULE_H

#include "netsurf/errors.h"

nserror hamix_schedule(int tival, void (*callback)(void *p), void *p);
int hamix_schedule_run(void);

#endif
