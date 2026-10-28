#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

#include "utils/errors.h"
#include "utils/log.h"
#include "utils/nsurl.h"
#include "utils/filepath.h"
#include "netsurf/netsurf.h"
#include "netsurf/browser_window.h"

#include "hamix/schedule.h"
#include "hamix/browser.h"
#include "hamix/fetch.h"
#include "hamix/bitmap.h"
#include "hamix/layout.h"
#include "hamix/hamix.h"

char **respaths = NULL;
static bool hamix_quit_requested = false;

static void hamix_quit(void)
{
	hamix_quit_requested = true;
}

static nserror hamix_launch_url(struct nsurl *url)
{
	(void)url;
	return NSERROR_OK;
}

static struct gui_misc_table hamix_misc_table = {
	.schedule = hamix_schedule,
	.quit = hamix_quit,
	.launch_url = hamix_launch_url,
};

static struct netsurf_table hamix_table = {
	.misc = &hamix_misc_table,
	.window = NULL,
	.fetch = NULL,
	.bitmap = NULL,
	.layout = NULL,
};

int hamix_init(const char *cache_dir)
{
	nserror ret;

	hamix_table.window = hamix_window_table;
	hamix_table.fetch = hamix_fetch_table;
	hamix_table.bitmap = hamix_bitmap_table;
	hamix_table.layout = hamix_layout_table;

	ret = netsurf_register(&hamix_table);
	if (ret != NSERROR_OK) {
		return -1;
	}

	ret = netsurf_init(cache_dir);
	if (ret != NSERROR_OK) {
		return -1;
	}

	return 0;
}

void hamix_shutdown(void)
{
	netsurf_exit();
}

struct hamix_window *hamix_window_create(int width, int height, void *userdata)
{
	struct hamix_window *hw = calloc(sizeof(struct hamix_window), 1);
	if (!hw)
		return NULL;

	hw->width = width > 0 ? width : 1024;
	hw->height = height > 0 ? height : 768;
	hw->userdata = userdata;
	hw->title = strdup("HxSurf");

	return hw;
}

void hamix_window_destroy(struct hamix_window *hw)
{
	if (!hw)
		return;
	if (hw->title)
		free(hw->title);
	if (hw->url)
		free(hw->url);
	if (hw->status)
		free(hw->status);
	free(hw);
}

void hamix_window_navigate(struct hamix_window *hw, const char *url)
{
	nsurl *nurl = NULL;
	if (!hw || !url)
		return;

	if (nsurl_create(url, &nurl) == NSERROR_OK) {
		if (hw->url)
			free(hw->url);
		hw->url = strdup(url);
		if (hw->bw) {
			browser_window_navigate(hw->bw, nurl, NULL, BW_NAVIGATE_NONE, NULL, NULL, NULL);
		}
		nsurl_unref(nurl);
	}
}

void hamix_poll(void)
{
	hamix_schedule_run();
}

int main(int argc, char **argv)
{
	const char *start_url = "about:hamix";

	if (argc > 1) {
		start_url = argv[1];
	}

	if (hamix_init(NULL) != 0) {
		fprintf(stderr, "Failed to initialize HxSurf NetSurf engine\n");
		return 1;
	}

	struct hamix_window *win = hamix_window_create(1024, 768, NULL);
	if (win) {
		hamix_window_navigate(win, start_url);
		hamix_poll();
		hamix_window_destroy(win);
	}

	hamix_shutdown();
	return 0;
}
