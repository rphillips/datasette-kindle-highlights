from datasette import hookimpl
from datasette.utils.asgi import asgi_send, Response
from functools import wraps


def redirect_museum(request):
    return Response.redirect("/{}".format(request.url_vars["id"]), status=301)


@hookimpl
def register_routes():
    return [
        (r"^/highlights/highlight/(?P<id>\d+)$", redirect_museum),
    ]

