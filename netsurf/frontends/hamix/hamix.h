#ifndef NETSURF_HAMIX_H
#define NETSURF_HAMIX_H

#include <stdint.h>
#include <stdbool.h>
#include "netsurf/browser_window.h"
#include "netsurf/netsurf.h"

struct hamix_window {
	uint32_t id;
	int width;
	int height;
	int scroll_x;
	int scroll_y;
	char *title;
	char *url;
	char *status;
	void *userdata;
	struct browser_window *bw;
	struct hamix_window *next;
};

int hamix_init(const char *cache_dir);
void hamix_shutdown(void);
struct hamix_window *hamix_window_create(int width, int height, void *userdata);
void hamix_window_destroy(struct hamix_window *hw);
void hamix_window_navigate(struct hamix_window *hw, const char *url);
void hamix_poll(void);

#endif
