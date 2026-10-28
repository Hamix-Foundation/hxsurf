#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

#include "utils/errors.h"
#include "utils/file.h"
#include "utils/nsurl.h"
#include "utils/filepath.h"
#include "netsurf/fetch.h"

#include "hamix/fetch.h"

extern char **respaths;

static const char *hamix_fetch_filetype(const char *path)
{
	const char *ext = strrchr(path, '.');
	if (!ext)
		return "text/plain";
	ext++;
	if (strcasecmp(ext, "html") == 0 || strcasecmp(ext, "htm") == 0)
		return "text/html";
	if (strcasecmp(ext, "css") == 0)
		return "text/css";
	if (strcasecmp(ext, "png") == 0)
		return "image/png";
	if (strcasecmp(ext, "jpg") == 0 || strcasecmp(ext, "jpeg") == 0)
		return "image/jpeg";
	if (strcasecmp(ext, "gif") == 0)
		return "image/gif";
	if (strcasecmp(ext, "svg") == 0)
		return "image/svg+xml";
	return "text/plain";
}

static nsurl *gui_get_resource_url(const char *path)
{
	char buf[PATH_MAX];
	nsurl *url = NULL;

	if (respaths) {
		const char *found = filepath_sfind(respaths, buf, path);
		if (found) {
			netsurf_path_to_nsurl(found, &url);
			return url;
		}
	}
	snprintf(buf, sizeof(buf), "/usr/share/hxsurf/%s", path);
	netsurf_path_to_nsurl(buf, &url);
	return url;
}

static struct gui_fetch_table fetch_table = {
	.filetype = hamix_fetch_filetype,
	.get_resource_url = gui_get_resource_url,
};

struct gui_fetch_table *hamix_fetch_table = &fetch_table;
