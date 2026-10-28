CWARNFLAGS += -Werror

CFLAGS += -std=c99 \
	  -Dhamix -Dnshamix -g \
	  -DHAMIX_RESPATH=\"$(NETSURF_HAMIX_RESOURCES)\"

LDFLAGS += -lm

HAMIX_RESOURCES_DIR := $(FRONTEND_RESOURCES_DIR)

MESSAGES_FILTER=hamix
MESSAGES_TARGET=$(HAMIX_RESOURCES_DIR)

ifneq ($(HOST),FreeBSD)
  CFLAGS += -D_POSIX_C_SOURCE=200809L \
	    -D_XOPEN_SOURCE=700 \
	    -D_BSD_SOURCE \
	    -D_DEFAULT_SOURCE \
	    -D_NETBSD_SOURCE
endif

ifeq ($(HOST),Windows_NT)
  CFLAGS += -U__STRICT_ANSI__
endif

S_FRONTEND := main.c schedule.c bitmap.c browser.c layout.c fetch.c

SOURCES = $(S_COMMON) $(S_IMAGE) $(S_BROWSER) $(S_PDF) $(S_FRONTEND)
EXETARGET := hxsurf

install-hamix:

package-hamix:
