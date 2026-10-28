#ifndef NETSURF_HAMIX_BROWSER_H
#define NETSURF_HAMIX_BROWSER_H

#include <stdint.h>
#include <stdbool.h>
#include "netsurf/window.h"

extern struct gui_window_table *hamix_window_table;

struct gui_window {
	struct gui_window *next;
	struct gui_window *prev;
	uint32_t win_num;
	struct browser_window *bw;
	int width;
	int height;
	int scrollx;
	int scrolly;
	char *title;
	char *url;
	char *status;
	void *userdata;
};

struct gui_window *hamix_find_window_by_num(uint32_t win_num);

#endif
