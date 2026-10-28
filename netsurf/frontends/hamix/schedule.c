#include <time.h>
#include <stdlib.h>
#include <stdbool.h>

#include "utils/sys_time.h"
#include "utils/errors.h"
#include "hamix/schedule.h"

struct nscallback {
	struct nscallback *next;
	struct timeval tv;
	void (*callback)(void *p);
	void *p;
};

static struct nscallback *schedule_list = NULL;

static nserror schedule_remove(void (*callback)(void *p), void *p)
{
	struct nscallback *cur_nscb;
	struct nscallback *prev_nscb;
	struct nscallback *unlnk_nscb;
	bool removed = false;

	if (schedule_list == NULL) {
		return NSERROR_NOT_FOUND;
	}

	cur_nscb = schedule_list;
	prev_nscb = NULL;

	while (cur_nscb != NULL) {
		if ((cur_nscb->callback == callback) && (cur_nscb->p == p)) {
			unlnk_nscb = cur_nscb;
			cur_nscb = unlnk_nscb->next;

			if (prev_nscb == NULL) {
				schedule_list = cur_nscb;
			} else {
				prev_nscb->next = cur_nscb;
			}
			free(unlnk_nscb);
			removed = true;
		} else {
			prev_nscb = cur_nscb;
			cur_nscb = prev_nscb->next;
		}
	}

	if (!removed) {
		return NSERROR_NOT_FOUND;
	}
	return NSERROR_OK;
}

nserror hamix_schedule(int tival, void (*callback)(void *p), void *p)
{
	struct nscallback *nscb;
	struct timeval tv;
	nserror ret;

	ret = schedule_remove(callback, p);
	if (tival < 0) {
		return ret;
	}

	tv.tv_sec = tival / 1000;
	tv.tv_usec = (tival % 1000) * 1000;

	nscb = calloc(1, sizeof(struct nscallback));
	if (!nscb) {
		return NSERROR_NOMEM;
	}

	gettimeofday(&nscb->tv, NULL);
	timeradd(&nscb->tv, &tv, &nscb->tv);

	nscb->callback = callback;
	nscb->p = p;

	nscb->next = schedule_list;
	schedule_list = nscb;

	return NSERROR_OK;
}

int hamix_schedule_run(void)
{
	struct timeval tv;
	struct timeval nexttime;
	struct timeval rettime;
	struct nscallback *cur_nscb;
	struct nscallback *prev_nscb;
	struct nscallback *unlnk_nscb;

	if (schedule_list == NULL) {
		return -1;
	}

	cur_nscb = schedule_list;
	prev_nscb = NULL;
	nexttime = cur_nscb->tv;

	gettimeofday(&tv, NULL);

	while (cur_nscb != NULL) {
		if (timercmp(&tv, &cur_nscb->tv, >)) {
			unlnk_nscb = cur_nscb;

			if (prev_nscb == NULL) {
				schedule_list = unlnk_nscb->next;
			} else {
				prev_nscb->next = unlnk_nscb->next;
			}

			unlnk_nscb->callback(unlnk_nscb->p);
			free(unlnk_nscb);

			if (schedule_list == NULL) {
				return -1;
			}

			cur_nscb = schedule_list;
			prev_nscb = NULL;
			nexttime = cur_nscb->tv;
		} else {
			if (timercmp(&nexttime, &cur_nscb->tv, >)) {
				nexttime = cur_nscb->tv;
			}
			prev_nscb = cur_nscb;
			cur_nscb = prev_nscb->next;
		}
	}

	timersub(&nexttime, &tv, &rettime);
	return (rettime.tv_sec * 1000) + (rettime.tv_usec / 1000);
}
