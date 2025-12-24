// Copyright (c) 2015 Cesanta Software Limited
// All rights reserved

#include "mongoose.h"

#include <libgen.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static const char *s_http_port = "8080";
static struct mg_serve_http_opts s_http_server_opts;

static void ev_handler(struct mg_connection *nc, int ev, void *p) {
  if (ev == MG_EV_HTTP_REQUEST) {
    mg_serve_http(nc, (struct http_message *) p, s_http_server_opts);
  }
}

static void build_document_root(char out[PATH_MAX], const char *argv0) {
  char exe_path[PATH_MAX];
  char exe_dir[PATH_MAX];

  out[0] = '\0';

  if (argv0 != NULL && realpath(argv0, exe_path) != NULL) {
    // dirname() may modify its input, so copy.
    strncpy(exe_dir, exe_path, sizeof(exe_dir) - 1);
    exe_dir[sizeof(exe_dir) - 1] = '\0';
    snprintf(out, PATH_MAX, "%s/html", dirname(exe_dir));
    return;
  }

  // Fallback: relative to current working directory.
  snprintf(out, PATH_MAX, "html");
}

int main(int argc, char **argv) {
  struct mg_mgr mgr;
  struct mg_connection *nc;
  char document_root[PATH_MAX];

  mg_mgr_init(&mgr, NULL);
  printf("Starting web server on port %s\n", s_http_port);
  nc = mg_bind(&mgr, s_http_port, ev_handler);
  if (nc == NULL) {
    printf("Failed to create listener\n");
    return 1;
  }

  // Set up HTTP server parameters
  mg_set_protocol_http_websocket(nc);

  build_document_root(document_root, (argc > 0) ? argv[0] : NULL);
  printf("Document root: %s\n", document_root);
  s_http_server_opts.document_root = document_root;
  // Enable CGI (e.g. /cgi-bin/env.cgi, /test1/GET_POST.cgi, /cgi-bin/adder.exe)
  s_http_server_opts.cgi_file_pattern = "**.cgi$|**.exe$";
  s_http_server_opts.index_files = "index.html,index.htm";
  s_http_server_opts.enable_directory_listing = "yes";

  for (;;) {
    mg_mgr_poll(&mgr, 1000);
  }
  mg_mgr_free(&mgr);

  return 0;
}
