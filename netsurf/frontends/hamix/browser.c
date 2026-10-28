#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "utils/errors.h"
#include "utils/nsurl.h"
#include "netsurf/window.h"
#include "netsurf/browser_window.h"

#include "hamix/browser.h"

static struct gui_window *window_list = NULL;
static uint32_t next_win_num = 1;

struct gui_window *hamix_find_window_by_num(uint32_t win_num)
{
	struct gui_window *w = window_list;
	while (w) {
		if (w->win_num == win_num)
			return w;
		w = w->next;
	}
	return NULL;
}

static struct gui_window *gui_window_create(struct browser_window *bw,
					    struct gui_window *existing,
					    gui_window_create_flags flags)
{
	(void)existing;
	(void)flags;

	struct gui_window *gw = calloc(sizeof(struct gui_window), 1);
	if (!gw)
		return NULL;

	gw->win_num = next_win_num++;
	gw->bw = bw;
	gw->width = 1024;
	gw->height = 768;
	gw->title = strdup("HxSurf");

	gw->next = window_list;
	if (window_list)
		window_list->prev = gw;
	window_list = gw;

	return gw;
}

static void gui_window_destroy(struct gui_window *gw)
{
	if (!gw)
		return;

	if (gw->prev)
		gw->prev->next = gw->next;
	if (gw->next)
		gw->next->prev = gw->prev;
	if (window_list == gw)
		window_list = gw->next;

	if (gw->title)
		free(gw->title);
	if (gw->url)
		free(gw->url);
	if (gw->status)
		free(gw->status);

	free(gw);
}

static nserror gui_window_invalidate_area(struct gui_window *gw,
					  const struct rect *rect)
{
	(void)gw;
	(void)rect;
	return NSERROR_OK;
}

static nserror gui_window_get_scroll(struct gui_window *gw, int *sx, int *sy)
{
	if (!gw)
		return NSERROR_BAD_PARAMETER;
	if (sx)
		*sx = gw->scrollx;
	if (sy)
		*sy = gw->scrolly;
	return NSERROR_OK;
}

static nserror gui_window_set_scroll(struct gui_window *gw, const struct rect *rect)
{
	if (!gw || !rect)
		return NSERROR_BAD_PARAMETER;
	gw->scrollx = rect->x0;
	gw->scrolly = rect->y0;
	return NSERROR_OK;
}

static nserror gui_window_get_dimensions(struct gui_window *gw, int *width, int *height)
{
	if (!gw)
		return NSERROR_BAD_PARAMETER;
	if (width)
		*width = gw->width;
	if (height)
		*height = gw->height;
	return NSERROR_OK;
}

static nserror gui_window_event(struct gui_window *gw, enum gui_window_event event)
{
	(void)gw;
	(void)event;
	return NSERROR_OK;
}

static nserror gui_window_set_title(struct gui_window *gw, const char *title)
{
	if (!gw)
		return NSERROR_BAD_PARAMETER;
	if (gw->title)
		free(gw->title);
	gw->title = title ? strdup(title) : NULL;
	return NSERROR_OK;
}

static nserror gui_window_set_url(struct gui_window *gw, nsurl *url)
{
	if (!gw)
		return NSERROR_BAD_PARAMETER;
	if (gw->url)
		free(gw->url);
	gw->url = url ? strdup(nsurl_access(url)) : NULL;
	return NSERROR_OK;
}

static nserror gui_window_set_icon(struct gui_window *gw, struct hlcache_handle *icon)
{
	(void)gw;
	(void)icon;
	return NSERROR_OK;
}

static nserror gui_window_set_status(struct gui_window *gw, const char *status)
{
	if (!gw)
		return NSERROR_BAD_PARAMETER;
	if (gw->status)
		free(gw->status);
	gw->status = status ? strdup(status) : NULL;
	return NSERROR_OK;
}

static nserror gui_window_set_pointer(struct gui_window *gw, gui_pointer_shape shape)
{
	(void)gw;
	(void)shape;
	return NSERROR_OK;
}

static nserror gui_window_place_caret(struct gui_window *gw, int x, int y, int height,
				      const struct rect *clip)
{
	(void)gw;
	(void)x;
	(void)y;
	(void)height;
	(void)clip;
	return NSERROR_OK;
}

static nserror gui_window_drag_start(struct gui_window *gw, gui_drag_type type,
				     const struct rect *rect)
{
	(void)gw;
	(void)type;
	(void)rect;
	return NSERROR_OK;
}

static nserror gui_window_save_link(struct gui_window *gw, nsurl *url, const char *title)
{
	(void)gw;
	(void)url;
	(void)title;
	return NSERROR_OK;
}

static struct gui_window_table window_table = {
	.create = gui_window_create,
	.destroy = gui_window_destroy,
	.invalidate = gui_window_invalidate_area,
	.get_scroll = gui_window_get_scroll,
	.set_scroll = gui_window_set_scroll,
	.get_dimensions = gui_window_get_dimensions,
	.event = gui_window_event,
	.set_title = gui_window_set_title,
	.set_url = gui_window_set_url,
	.set_icon = gui_window_set_icon,
	.set_status = gui_window_set_status,
	.set_pointer = gui_window_set_pointer,
	.place_caret = gui_window_place_caret,
	.drag_start = gui_window_drag_start,
	.save_link = gui_window_save_link,
};

struct gui_window_table *hamix_window_table = &window_table;
